pub mod middleware;

use macros_rs::string;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_json::json;
use tera::Context;

use crate::{
    account,
    config::db::Pool,
    config::structs::Config,
    http::errors::Error,
    models::user::LoginDTO,
    pages::{render, TeraState},
};

use actix_web::{
    cookie::{time::Duration, Cookie},
    dev::ConnectionInfo,
    http::{header::ContentType, StatusCode},
    web::{Data, Form},
    HttpRequest, HttpResponse,
};

#[derive(Debug, Deserialize)]
pub struct Login {
    email: String,
    password: String,
    remember: Option<String>,
}

pub async fn login(req: HttpRequest, config: Data<&OnceCell<Config>>, tera: Data<TeraState>) -> HttpResponse {
    tracing::info!(method = string!(req.method()), "internal '{}'", req.uri());

    let config = config.get_ref();
    let tera = tera.get_ref();
    let mut page = Context::new();

    let name = match req.headers().get("SelectService") {
        Some(name) => name.to_str().unwrap_or("(none)"),
        None => "(none)",
    };

    println!("{:?}", req.cookie("sp_token"));

    page.insert("service_name", name);
    page.insert("logged_in", &json!(false));
    page.insert("button_status", "enabled");
    page.insert("email_placeholder", &"");
    page.insert("remember_checked", &"");
    page.insert("password_placeholder", &"");

    let payload = render("login", &tera.0, &mut page);

    HttpResponse::build(StatusCode::OK).content_type(ContentType::html()).body(payload)
}

pub async fn form_handler(req: HttpRequest, conn: ConnectionInfo, body: Form<Login>, pool: Data<Pool>, tera: Data<TeraState>) -> Result<HttpResponse, Error> {
    tracing::info!(method = string!(req.method()), "login '{}'", req.uri());

    let email = body.email.to_lowercase();
    let password = body.password.clone();
    let remember = body.remember.clone();

    let tera = tera.get_ref();
    let mut page = Context::new();

    let name = match req.headers().get("SelectService") {
        Some(name) => name.to_str().unwrap_or("(none)"),
        None => "(none)",
    };

    match remember.unwrap_or(string!("off")).as_str() {
        "on" => page.insert("remember_checked", &"checked"),
        _ => page.insert("remember_checked", &""),
    }

    page.insert("service_name", name);
    page.insert("logged_in", &json!(true));
    page.insert("button_status", "disabled");
    page.insert("email_placeholder", &email);
    page.insert("password_placeholder", &"•".repeat(password.len()));

    let payload = render("login", &tera.0, &mut page);
    let login_dto = LoginDTO { password, username_or_email: email };

    match account::login(login_dto, &pool) {
        Ok(res) => {
            println!("{:?}", conn.host());

            let cookie = Cookie::build("sp_token", res.token)
                .max_age(Duration::hours(10))
                // .expires(None) for dont remember me
                .domain(conn.host())
                .secure(false)
                .path("/")
                .http_only(true)
                .finish();
            Ok(HttpResponse::build(StatusCode::OK).content_type(ContentType::html()).cookie(cookie).body(payload))
        }
        Err(err) => {
            println!("{err:?}");
            Err(err)
        }
    }
}
