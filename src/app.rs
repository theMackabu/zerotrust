use macros_rs::string;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{
    config::{db::Pool, structs::Config},
    http::{errors::Error, token},
    models::user::User,
    pages::{render, TeraState},
};

use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data, Json},
    HttpRequest, HttpResponse,
};

macro_rules! send {
    () => {
        HttpResponse::build(StatusCode::OK).content_type(ContentType::html())
    };
}

macro_rules! ok {
    () => {
        HttpResponse::build(StatusCode::OK)
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setup {
    pub account: Account,
    pub settings: Settings,
    pub service: Service,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub icon: String,
    pub prefix: String,
    pub accent: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub display: String,
    pub address: String,
    pub port: u16,
    pub tls: bool,
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

pub async fn setup(req: HttpRequest, tera: Data<TeraState>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "setup '{}'", req.uri());

    send!().body(render("setup", &tera.get_ref().0, &mut Context::new()))
}

pub async fn setup_handler(req: HttpRequest, body: Json<Setup>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "setup '{}'", req.uri());

    println!("{body:?}");

    ok!().finish()
}
