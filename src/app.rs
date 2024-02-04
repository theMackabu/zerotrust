use macros_rs::{str, string};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tera::Context;
use toml_edit::{value, Array};

use crate::{
    config::{db::Pool, structs::Config},
    http::{
        errors::{Error, JsonError},
        token,
    },
    models::user::{User, UserDTO},
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Setup {
    pub account: Account,
    pub settings: Settings,
    pub service: Service,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub icon: String,
    pub prefix: String,
    pub accent: String,
    pub secret: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub display: String,
    pub address: String,
    pub port: Option<u16>,
    pub tls: bool,
}

pub async fn dashboard(req: HttpRequest, pool: Data<Pool>, config: Data<Config>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    if let Some(cookie) = req.cookie("sp_token") {
        if let Ok(token_data) = token::decode_token(cookie.value().to_string(), config.as_ref()) {
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

pub async fn setup(req: HttpRequest, tera: Data<TeraState>, config: Data<Config>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "setup '{}'", req.uri());

    send!().body(render("setup", &tera.get_ref().0, &mut Context::new(), config.as_ref()))
}

pub async fn setup_handler(req: HttpRequest, body: Json<Setup>, pool: Data<Pool>) -> Result<HttpResponse, JsonError> {
    tracing::info!(method = string!(req.method()), "setup '{}'", req.uri());

    let mut config = Config::new();
    let mut edit = config.edit();

    let user_dto = UserDTO {
        admin: true,
        username: body.account.username.to_lowercase(),
        email: body.account.email.to_lowercase(),
        password: body.account.password.clone(),
        tokens: json!([]).to_string(),
        services: json!([]).to_string(),
        providers: json!(["basic"]).to_string(),
    };

    let port = match body.service.port {
        Some(port) => port,
        None => match body.service.tls {
            true => 443,
            false => 80,
        },
    } as i64;

    edit["settings"]["secret"] = value(body.settings.secret.clone());
    edit["settings"]["app"]["logo"] = value(body.settings.icon.clone());
    edit["settings"]["app"]["accent"] = value(body.settings.accent.clone());
    edit["settings"]["server"]["prefix"] = value(body.settings.prefix.clone());

    edit["backends"][body.service.name.clone()]["port"] = value(port);
    edit["backends"][body.service.name.clone()]["providers"] = value(Array::default());
    edit["backends"][body.service.name.clone()]["tls"] = value(body.service.tls.clone());
    edit["backends"][body.service.name.clone()]["address"] = value(body.service.address.clone());
    edit["backends"][body.service.name.clone()]["display_name"] = value(body.service.display.clone());

    config.set(Config::from_str(&edit.to_string()));
    config.write();

    match User::signup(user_dto, &mut pool.get().unwrap()) {
        Ok(_) => Ok(ok!().finish()),
        Err(err) => Err(JsonError {
            status: 500,
            message: str!(err.clone()),
        }),
    }
}
