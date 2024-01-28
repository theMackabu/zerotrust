mod config;
mod file;

use config::structs::Config;
use macros_rs::string;
use std::env;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::LevelFilter, prelude::*};

use actix_web::{
    get,
    http::header::{self, ContentType},
    http::StatusCode,
    web::{Bytes, Data, Path},
    App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Responder,
};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

fn convert_headers(current_headers: &header::HeaderMap) -> HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();

    for (key, value) in current_headers.iter() {
        let name = HeaderName::from_bytes(key.as_str().as_bytes()).expect("Failed to convert header name");
        let value = HeaderValue::from_bytes(value.as_bytes()).expect("Failed to convert header value");
        headers.insert(name, value);
    }

    return headers;
}

fn transfer_headers(mut proxied_response: HttpResponseBuilder, current_headers: &HeaderMap) -> HttpResponseBuilder {
    for (key, value) in current_headers.iter() {
        proxied_response.append_header((key, value));
    }

    return proxied_response;
}

#[get("{url:.*}")]
async fn handler(url: Path<String>, req: HttpRequest, config: Data<Config>) -> impl Responder {
    tracing::info!(method = string!(req.method()), "request '{}'", req.uri());

    let client = reqwest::Client::new();
    let request = client.get(&url.clone()).headers(convert_headers(req.headers()));
    let response = request.send().await.unwrap();

    let proxied_response = HttpResponse::build(StatusCode::OK);
    transfer_headers(proxied_response, response.headers()).streaming(response.bytes_stream())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "INFO");

    let config = config::read();
    let app = || App::new().app_data(Data::new(config::read())).service(handler);

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
