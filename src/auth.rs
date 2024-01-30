use crate::config::structs::Config;
use crate::pages::{render, TeraState};
use macros_rs::string;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use tera::Context;

use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{BytesMut, Data, Form},
    HttpRequest, HttpResponse,
};

#[derive(Debug, Deserialize)]
struct Login {
    email: String,
    password: String,
    remember: String,
}

#[actix_web::get("/_sp/login")]
pub async fn login(req: HttpRequest, config: Data<&OnceCell<Config>>, tera: Data<TeraState>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    let tera = tera.get_ref();
    let mut page = Context::new();

    let name = match req.headers().get("SelectService") {
        Some(name) => name.to_str().unwrap_or("(none)"),
        None => "(none)",
    };

    page.insert("service_name", name);
    let payload = render("login", &tera.0, &mut page);

    HttpResponse::build(StatusCode::OK).content_type(ContentType::html()).body(payload)
}

#[actix_web::post("/_sp/login")]
pub async fn form_handler(req: HttpRequest, config: Data<&OnceCell<Config>>, body: Form<Login>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "login '{}'", req.uri());
    println!("{:?}", body);

    HttpResponse::build(StatusCode::OK).body("logged in, redirect now")
}
