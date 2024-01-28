mod config;
mod file;

use config::structs::Config;
use futures::{channel::mpsc::unbounded, sink::SinkExt, stream::StreamExt};
use macros_rs::string;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{filter::LevelFilter, prelude::*};

use actix_web::{
    guard::GuardContext,
    http::{header, StatusCode},
    web::{BytesMut, Data, Path, Payload},
    App, Error, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
};

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

fn guard_websocket(ctx: &GuardContext<'_>) -> bool { ctx.head().headers().get("upgrade").map_or(false, |h| h == "websocket") }

#[actix_web::get("{url:.*}")]
async fn get_proxy(url: Path<String>, req: HttpRequest, config: Data<Config>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "request '{}'", req.uri());

    if let Some(name) = req.headers().get("SelectService") {
        let service = match name.as_bytes() {
            b"cs27" => format!("http://localhost:9309/{url}"),
            _ => return HttpResponse::build(StatusCode::OK).body("not found"),
        };

        let client = reqwest::Client::new();
        let request = client.get(&service).headers(convert_headers(req.headers()));
        let response = request.send().await.unwrap();

        let proxied_response = HttpResponse::build(StatusCode::OK);
        transfer_headers(proxied_response, response.headers()).streaming(response.bytes_stream())
    } else {
        HttpResponse::build(StatusCode::OK).body("not found")
    }
}

#[actix_web::get("{url:.*}", guard = "guard_websocket")]
async fn ws_proxy(url: Path<String>, req: HttpRequest, mut payload: Payload) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "websocket '{}'", req.uri());

    if let Some(name) = req.headers().get("SelectService") {
        let service = match name.as_bytes() {
            b"cs27" => format!("ws://localhost:9309/{url}"),
            _ => return HttpResponse::build(StatusCode::OK).body("not found"),
        };

        let client = awc::Client::new();
        let mut ws = client.ws(service);

        for (key, value) in req.headers().iter() {
            ws = ws.header(key, value);
        }

        let (res, socket) = ws.connect().await.unwrap();

        assert_eq!(res.status().as_u16(), 101);

        let mut io = socket.into_parts().io;
        let (mut tx, rx) = unbounded();
        let mut buf = BytesMut::new();

        actix_web::rt::spawn(async move {
            loop {
                tokio::select! {
                    res = payload.next() => {
                        match res {
                            None => return,
                            Some(body) => {
                                let body = body.unwrap();
                                io.write_all(&body).await.unwrap();
                            }
                        }
                    }

                    res = io.read_buf(&mut buf) => {
                        let size = res.unwrap();
                        let bytes = buf.split_to(size).freeze();
                        tx.send(Ok::<_, Error>(bytes)).await.unwrap();
                    }
                }
            }
        });

        HttpResponse::SwitchingProtocols().streaming(rx)
    } else {
        HttpResponse::build(StatusCode::OK).body("not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "INFO");

    let config = config::read();
    let app = || App::new().app_data(Data::new(config::read())).service(ws_proxy).service(get_proxy);

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
