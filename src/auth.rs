pub mod middleware;

use macros_rs::string;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use tera::Context;

use crate::{
    config::db::Pool,
    config::structs::Config,
    http::{
        errors::{Error, JsonError},
        token,
    },
    models::{
        token::UserToken,
        user::{LoginDTO, User},
    },
    pages::{render, TeraState},
};

use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    dev::ConnectionInfo,
    http::{header::ContentType, StatusCode},
    web::{Data, Json},
    HttpRequest, HttpResponse,
};

#[derive(Debug, Deserialize)]
pub struct Login {
    email: String,
    password: String,
    remember: bool,
}

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

fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str { s.split(suffix).next().unwrap_or(s) }

pub async fn login(req: HttpRequest, config: Data<&OnceCell<Config>>, tera: Data<TeraState>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    let config = config.get_ref();
    let tera = tera.get_ref();
    let mut page = Context::new();

    match req.headers().get("SelectService") {
        None => page.insert("service_name", "(no service selected)"),
        Some(name) => page.insert("service_name", name.to_str().unwrap_or("(service name error)")),
    };

    send!().body(render("login", &tera.0, &mut page))
}

pub async fn login_handler(req: HttpRequest, conn: ConnectionInfo, body: Json<Login>, pool: Data<Pool>) -> Result<HttpResponse, JsonError> {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    let email = body.email.to_lowercase();
    let password = body.password.clone();
    let remember = body.remember.clone();

    let login_dto = LoginDTO { password, username_or_email: email };

    match User::login(login_dto, &mut pool.get().unwrap()) {
        Some(logged_user) => {
            let token = UserToken::generate_token(&logged_user);

            if logged_user.login_session.is_empty() {
                Err(JsonError {
                    status: 401,
                    message: "Wrong username or password, please try again.",
                })
            } else {
                let cookie_builder = Cookie::build("sp_token", token).domain(remove_suffix(conn.host(), ":")).secure(false).path("/").http_only(true);

                let cookie = match remember {
                    true => cookie_builder.max_age(Duration::seconds(604800)).finish(),
                    false => cookie_builder.expires(None).finish(),
                };

                Ok(ok!().cookie(cookie).finish())
            }
        }
        None => Err(JsonError {
            status: 401,
            message: "Wrong username or password, please try again.",
        }),
    }
}

pub async fn logout(req: HttpRequest, tera: Data<TeraState>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    let tera = tera.get_ref();
    let mut page = Context::new();

    match req.headers().get("SelectService") {
        None => page.insert("service_name", "(no service selected)"),
        Some(name) => page.insert("service_name", name.to_str().unwrap_or("(service name error)")),
    };

    send!().body(render("logout", &tera.0, &mut page))
}

pub async fn logout_handler(req: HttpRequest, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    if let Some(cookie) = req.cookie("sp_token") {
        if let Ok(token_data) = token::decode_token(cookie.value().to_string()) {
            if let Ok(username) = token::verify_token(&token_data, &pool) {
                if let Ok(user) = User::find_user_by_username(&username, &mut pool.get().unwrap()) {
                    User::logout(user.id, &mut pool.get().unwrap());
                    return Ok(ok!().finish());
                }
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
