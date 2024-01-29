mod auth;
mod config;
mod helpers;
mod pages;

use actix_web_static_files::ResourceFiles;
use awc::Client;
use config::structs::Config;
use futures_util::StreamExt;
use include_dir::{include_dir, Dir};
use macros_rs::{clone, string, ternary};
use once_cell::sync::Lazy;
use pages::create_templates;
use std::collections::BTreeMap;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::LevelFilter, prelude::*};
use url::Url;

use actix_web::{
    dev::PeerAddr,
    error::ErrorInternalServerError,
    guard,
    http::StatusCode,
    web::{self, Data, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

struct Backend {
    url: Url,
    providers: Vec<String>,
}

type Backends = BTreeMap<String, Backend>;

static ASSETS_DIR: Dir<'_> = include_dir!("src/pages/dist/_sp/assets");

static BACKEND_LIST: Lazy<Backends> = Lazy::new(|| {
    let mut backends: Backends = BTreeMap::new();
    let config = config::read();

    for (name, item) in config.backends {
        let tls = match item.tls {
            None => "http",
            Some(is_tls) => ternary!(is_tls, "https", "http"),
        };

        let url = format!("{tls}://{}:{}", item.address, item.port);

        backends.insert(
            name,
            Backend {
                url: Url::parse(&url).unwrap(),
                providers: item.providers,
            },
        );
    }

    return backends;
});

// fn error_handlers() -> ErrorHandlers<BoxBody> {
//     ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
// }
//
// // Error handler for a 404 Page not found error.
// fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
//     let response = get_error_response(&res, "Page not found");
//     Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
//         res.into_parts().0,
//         response.map_into_left_body(),
//     )))
// }

// add initial setup (no services found & how to config)
async fn proxy(req: HttpRequest, payload: Payload, peer_addr: Option<PeerAddr>, config: Data<Config>, backends: Data<Backends>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "request '{}'", req.uri());

    let config = config.get_ref();

    if let Some(name) = req.headers().get("SelectService") {
        let mut url = match backends.get(name.to_str().unwrap_or("")) {
            Some(item) => clone!(item.url),
            None => return Ok(HttpResponse::build(StatusCode::NOT_FOUND).body("Service not found")),
        };

        url.set_path(req.uri().path());
        url.set_query(req.uri().query());

        let client = Client::builder().disable_redirects().finish();
        let forwarded_req = client.request_from(url.as_str(), req.head()).no_decompress();

        let forwarded_req = match peer_addr {
            Some(PeerAddr(addr)) => forwarded_req.insert_header(("x-forwarded-for", addr.ip().to_string())),
            None => forwarded_req,
        };

        let res = forwarded_req.send_stream(payload).await.map_err(ErrorInternalServerError)?;
        let mut client_response = HttpResponse::build(res.status());

        for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
            client_response.insert_header((header_name.clone(), header_value.clone()));
        }

        tracing::info!(status = string!(res.status()), "response '{}'", req.uri());
        Ok(client_response.streaming(res))
    } else {
        Ok(HttpResponse::build(StatusCode::NOT_FOUND).body("No service header"))
    }
}

async fn proxy_ws(req: HttpRequest, client_stream: Payload, config: Data<Config>, backends: Data<Backends>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    tracing::info!(method = string!(req.method()), "websocket '{}'", req.uri());

    let config = config.get_ref();

    if let Some(name) = req.headers().get("SelectService") {
        let mut url = match backends.get(name.to_str().unwrap_or("")) {
            Some(item) => clone!(item.url),
            None => return Ok(HttpResponse::build(StatusCode::NOT_FOUND).body("Service not found")),
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
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Target did not reply with 101 upgrade")));
        }

        let mut client_response = HttpResponse::SwitchingProtocols();
        client_response.upgrade("websocket");
        for (header, value) in target_response.headers() {
            client_response.insert_header((header.to_owned(), value.to_owned()));
        }

        let target_upgrade = target_response.upgrade().await?;
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
        Ok(client_response.streaming(target_stream))
    } else {
        Ok(HttpResponse::build(StatusCode::NOT_FOUND).body("No service header"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "INFO");

    let config = config::read();

    let app = || {
        let files = helpers::build_hashmap(&ASSETS_DIR);

        App::new()
            .app_data(Data::new(&BACKEND_LIST))
            .app_data(Data::new(config::read()))
            .app_data(Data::new(create_templates()))
            .service(auth::login)
            .service(ResourceFiles::new("/_sp/assets", files))
            .service(web::scope("{url:.*}").guard(guard::Header("upgrade", "websocket")).route("", web::to(proxy_ws)))
            .default_service(web::to(proxy))
    };

    let formatting_layer = BunyanFormattingLayer::new("server".into(), std::io::stdout)
        .skip_fields(vec!["file", "line"].into_iter())
        .expect("Unable to create logger");

    tracing_subscriber::registry()
        .with(LevelFilter::from(tracing::Level::INFO))
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

    tracing::info!(address = config.settings.address, port = config.settings.port, "server started");
    HttpServer::new(app).bind(config.get_address()).unwrap().run().await
}
