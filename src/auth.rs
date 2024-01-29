use crate::config::structs::Config;
use crate::pages::{render, TeraState};
use macros_rs::string;
use tera::Context;

use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data, Payload},
    HttpRequest, HttpResponse,
};

#[actix_web::get("/_sp/login")]
pub async fn login(req: HttpRequest, config: Data<Config>, tera: Data<TeraState>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    let tera = tera.get_ref();
    let mut ctx = Context::new();

    let name = match req.headers().get("SelectService") {
        Some(name) => name.to_str().unwrap_or(""),
        None => "",
    };

    ctx.insert("service_name", name);
    let payload = render("login", &tera.0, &mut ctx);

    HttpResponse::build(StatusCode::OK).content_type(ContentType::html()).body(payload)
}
