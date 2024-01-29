mod catch;
mod errors;

use crate::{auth, config::structs::Config, pages::create_templates};
use actix_web_static_files::ResourceFiles;
use errors::Error;
use futures_util::StreamExt;
use include_dir::{include_dir, Dir};
use macros_rs::{clone, string, ternary};
use once_cell::sync::{Lazy, OnceCell};
use std::collections::BTreeMap;

use actix_web::{
    dev::PeerAddr,
    error::ErrorInternalServerError,
    guard,
    http::StatusCode,
    middleware::ErrorHandlers,
    web::{self, Data, Payload},
    App, HttpRequest, HttpResponse, HttpServer,
};

struct Backend {
    url: url::Url,
    providers: Vec<String>,
}

type Backends = BTreeMap<String, Backend>;

static ASSETS_DIR: Dir<'_> = include_dir!("src/pages/dist/_sp/assets");

static BACKEND_LIST: Lazy<Backends> = Lazy::new(|| {
    let mut backends: Backends = BTreeMap::new();
    let config = crate::CONFIG.get().unwrap();

    for (name, item) in config.backends.iter() {
        let tls = match item.tls {
            None => "http",
            Some(is_tls) => ternary!(is_tls, "https", "http"),
        };

        let url = format!("{tls}://{}:{}", item.address, item.port);

        backends.insert(
            clone!(name),
            Backend {
                url: url::Url::parse(&url).unwrap(),
                providers: clone!(item.providers),
            },
        );
    }

    return backends;
});

// add initial setup (no services found & how to config)
async fn proxy(req: HttpRequest, payload: Payload, peer_addr: Option<PeerAddr>, config: Data<&OnceCell<Config>>, backends: Data<&Lazy<Backends>>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "request '{}'", req.uri());

    let config = config.get_ref();

    if let Some(name) = req.headers().get("SelectService") {
        let name = name.to_str().unwrap_or("");
        let (mut url, providers) = match backends.get(name) {
            Some(item) => (clone!(item.url), clone!(item.providers)),
            None => return Err(Error::NotFound { message: "Service not found" }),
        };

        for provider in providers {
            if provider == "basic" {
                continue;
            }

            config.get().unwrap().providers.get(&provider);
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

        for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
            client_response.insert_header((header_name.clone(), header_value.clone()));
        }

        tracing::info!(service = name, status = string!(res.status()), "responded");
        Ok(client_response.streaming(res))
    } else {
        Err(Error::NotFound { message: "No service header" })
    }
}

async fn proxy_ws(req: HttpRequest, client_stream: Payload, config: Data<&OnceCell<Config>>, backends: Data<&Lazy<Backends>>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "websocket '{}'", req.uri());

    let config = config.get_ref();

    if let Some(name) = req.headers().get("SelectService") {
        let name = name.to_str().unwrap_or("");
        let mut url = match backends.get(name) {
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
pub async fn start() -> std::io::Result<()> {
    let config = crate::CONFIG.get().unwrap();

    let app = || {
        let files = crate::helpers::build_hashmap(&ASSETS_DIR);

        App::new()
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, errors::not_found))
            .app_data(Data::new(&crate::CONFIG))
            .app_data(Data::new(create_templates()))
            .app_data(Data::new(&BACKEND_LIST))
            .service(auth::login)
            .service(ResourceFiles::new("/_sp/assets", files))
            .service(web::scope("{url:.*}").guard(guard::Header("upgrade", "websocket")).route("", web::to(proxy_ws)))
            .default_service(web::to(proxy))
    };

    tracing::info!(address = config.settings.address, port = config.settings.port, "server started");
    HttpServer::new(app).bind(config.get_address()).unwrap().run().await
}
