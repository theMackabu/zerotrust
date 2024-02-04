pub mod catch;
pub mod errors;
pub mod token;

use actix_files as afs;
use actix_web_static_files::ResourceFiles;
use errors::Error;
use futures_util::StreamExt;
use include_dir::{include_dir, Dir};
use macros_rs::{clone, fmtstr, string};

use crate::{
    app,
    auth::{self, middleware},
    config::{db::Pool, structs::Config},
    pages::create_templates,
};

use actix_web::{
    dev::PeerAddr,
    error::ErrorInternalServerError,
    guard,
    http::StatusCode,
    middleware::ErrorHandlers,
    web::{self, Data, Payload},
    App, HttpRequest, HttpResponse, HttpServer,
};

static ASSETS_DIR: Dir<'_> = include_dir!("src/pages/dist/assets_provider");

async fn proxy(req: HttpRequest, payload: Payload, peer_addr: Option<PeerAddr>, config: Data<Config>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "request '{}'", req.uri());

    let config = config.get_ref();

    if let Some(name) = req.headers().get("SelectService") {
        let name = name.to_str().unwrap_or("");
        let (mut url, providers) = match config.backends().get(name) {
            Some(item) => (clone!(item.url), clone!(item.providers)),
            None => return Err(Error::NotFound { message: "Service not found" }),
        };

        for provider in providers {
            if provider == "basic" {
                continue;
            }

            config.providers.get(&provider);
        }

        url.set_path(req.uri().path());
        url.set_query(req.uri().query());

        let client = awc::Client::builder().disable_redirects().finish();
        let forwarded_req = client.request_from(url.as_str(), req.head()).no_decompress();

        let forwarded_req = match peer_addr {
            Some(PeerAddr(addr)) => forwarded_req.insert_header(("x-forwarded-for", addr.ip().to_string())),
            None => forwarded_req,
        };

        let res = catch::_try!(forwarded_req.send_stream(payload).await.map_err(ErrorInternalServerError));
        let mut client_response = HttpResponse::build(res.status());

        match res.status().as_u16() {
            400 => {
                return Err(Error::ConnectionRefused {
                    message: fmtstr!("Sorry, this page could not be loaded from upstream."),
                })
            }
            404 => {
                return Err(Error::NotFound {
                    message: fmtstr!("Sorry, this page could not be found on upstream."),
                })
            }
            _ => {}
        }

        for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
            client_response.insert_header((header_name.clone(), header_value.clone()));
        }

        tracing::info!(service = name, status = string!(res.status()), "responded");
        Ok(client_response.streaming(res))
    } else {
        Err(Error::NotFound { message: "No service header" })
    }
}

async fn proxy_ws(req: HttpRequest, client_stream: Payload, config: Data<Config>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "websocket '{}'", req.uri());

    let config = config.get_ref();

    if let Some(name) = req.headers().get("SelectService") {
        let name = name.to_str().unwrap_or("");
        let mut url = match config.backends().get(name) {
            Some(item) => clone!(item.url),
            None => return Err(Error::NotFound { message: "Service not found" }),
        };

        url.set_path(req.uri().path());
        url.set_query(req.uri().query());

        let mut request = reqwest::Client::new().get(url);
        for (key, value) in req.headers() {
            request = request.header(key, value);
        }
        let target_response = request.send().await.unwrap();

        let status = target_response.status().as_u16();
        if status != 101 {
            return Err(Error::ConnectionRefused {
                message: "Target did not reply with 101 upgrade",
            });
        }

        let mut client_response = HttpResponse::SwitchingProtocols();
        client_response.upgrade("websocket");
        for (header, value) in target_response.headers() {
            client_response.insert_header((header.to_owned(), value.to_owned()));
        }

        let target_upgrade = catch::_try!(target_response.upgrade().await);
        let (target_rx, mut target_tx) = tokio::io::split(target_upgrade);

        actix_web::rt::spawn(async move {
            let mut client_stream = client_stream.map(|result| result.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)));
            let mut client_read = tokio_util::io::StreamReader::new(&mut client_stream);
            let result = tokio::io::copy(&mut client_read, &mut target_tx).await;
            if let Err(err) = result {
                println!("Error proxying websocket client bytes to target: {err}")
            }
        });

        let target_stream = tokio_util::io::ReaderStream::new(target_rx);
        tracing::info!(service = name, status, "connected");

        Ok(client_response.streaming(target_stream))
    } else {
        Err(Error::NotFound { message: "No service header" })
    }
}

#[actix_web::main]
pub async fn start(pool: Pool, path: String) -> std::io::Result<()> {
    let config = Config::new().set_path(&path).read();

    let app = move || {
        let config = Config::new().set_path(&path.clone()).read();
        let prefix = config.settings.server.prefix.clone();
        let files = crate::helpers::build_hashmap(&ASSETS_DIR);

        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(create_templates()))
            .app_data(Data::new(pool.clone()))
            .route("/setup", web::get().guard(middleware::setup_guard).to(app::setup))
            .route("/setup", web::post().guard(middleware::setup_guard).to(app::setup_handler))
            .route(fmtstr!("/{prefix}/login"), web::get().guard(middleware::token_guard).to(auth::login))
            .route(fmtstr!("/{prefix}/logout"), web::get().to(auth::logout).wrap(middleware::Authentication))
            .route(fmtstr!("/{prefix}/app"), web::get().to(app::dashboard).wrap(middleware::Authentication))
            .route(fmtstr!("/{prefix}/api/login"), web::post().guard(middleware::token_guard).to(auth::login_handler))
            .route(fmtstr!("/{prefix}/api/logout"), web::post().to(auth::logout_handler).wrap(middleware::Authentication))
            .service(ResourceFiles::new(fmtstr!("/{prefix}/assets"), files))
            .service(afs::Files::new(fmtstr!("/{prefix}/static"), config.get_static()).index_file("index.html"))
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, errors::not_found))
            .service(
                web::scope("{url:.*}")
                    .guard(guard::Header("upgrade", "websocket"))
                    .route("", web::to(proxy_ws))
                    .wrap(middleware::Authentication),
            )
            .default_service(web::to(proxy).wrap(middleware::Authentication))
    };

    tracing::info!(address = config.get_address().0, port = config.get_address().1, "server started");
    HttpServer::new(app).bind(config.get_address()).unwrap().run().await
}
