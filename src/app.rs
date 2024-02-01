use macros_rs::string;
use once_cell::sync::OnceCell;
use tera::Context;

use crate::{
    config::{db::Pool, structs::Config},
    http::{errors::Error, token},
    models::user::User,
    pages::{render, TeraState},
};

use actix_web::{
    http::{header::ContentType, StatusCode},
    web::Data,
    HttpRequest, HttpResponse,
};

macro_rules! send {
    () => {
        HttpResponse::build(StatusCode::OK).content_type(ContentType::html())
    };
}

pub async fn dashboard(req: HttpRequest, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    if let Some(cookie) = req.cookie("sp_token") {
        if let Ok(token_data) = token::decode_token(cookie.value().to_string()) {
            if let Ok(login_info) = User::find_login_info_by_token(&token_data.claims, &mut pool.get().unwrap()) {
                return Ok(send!().body(serde_json::to_string(&login_info).unwrap()));
            }
        }

        Err(Error::InternalError {
            message: "Error while processing token, please try again!",
        })
    } else {
        Err(Error::BadClientData {
            message: "Token missing from request",
        })
    }
}

pub async fn setup(req: HttpRequest, config: Data<&OnceCell<Config>>, tera: Data<TeraState>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "setup '{}'", req.uri());

    let tera = tera.get_ref();
    let mut page = Context::new();
    let config = config.get_ref().get().unwrap();

    page.insert("config_body", config);
    send!().body(render("setup", &tera.0, &mut page))
}

pub async fn setup_handler(req: HttpRequest, config: Data<&OnceCell<Config>>, tera: Data<TeraState>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "setup '{}'", req.uri());

    let tera = tera.get_ref();
    let mut page = Context::new();
    let config = config.get_ref().get().unwrap();

    page.insert("config_body", config);
    send!().body(render("setup", &tera.0, &mut page))
}
