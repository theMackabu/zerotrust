#![allow(dead_code)]

use super::catch::FromResidual;
use crate::pages::{create_templates, render};

use actix_web::{
    dev, error,
    http::{header::ContentType, StatusCode},
    middleware::ErrorHandlerResponse,
    HttpResponse,
};

use derive_more::{Display, Error};
use macros_rs::str;
use std::convert::Infallible;
use tera::Context;

#[derive(Debug, Display, Error)]
pub(crate) enum Error {
    #[display(fmt = "{}", message)]
    NotFound { message: &'static str },
    #[display(fmt = "{}", message)]
    InternalError { message: &'static str },
    #[display(fmt = "{}", message)]
    BadClientData { message: &'static str },
    #[display(fmt = "{}", message)]
    ConnectionRefused { message: &'static str },
    #[display(fmt = "{}", message)]
    Timeout { message: &'static str },
    #[display(fmt = "{}", message)]
    Unauthorized { message: &'static str },
    #[display(fmt = "{}", message)]
    Ratelimit { message: &'static str },
    #[display(fmt = "{}", message)]
    Generic { status: StatusCode, message: &'static str },
}

fn create_error(name: &str, msg: &str) -> String {
    let tera = create_templates();
    let mut ctx = Context::new();

    ctx.insert("error_message", name);
    ctx.insert("error_name", msg);

    render("error", &tera.0, &mut ctx)
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let payload = create_error(&self.status_code().to_string(), &self.to_string());
        let mut res = HttpResponse::build(self.status_code());

        return res.content_type(ContentType::html()).body(payload);
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::NotFound { .. } => StatusCode::NOT_FOUND,
            Error::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadClientData { .. } => StatusCode::BAD_REQUEST,
            Error::ConnectionRefused { .. } => StatusCode::SERVICE_UNAVAILABLE,
            Error::Timeout { .. } => StatusCode::GATEWAY_TIMEOUT,
            Error::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            Error::Ratelimit { .. } => StatusCode::TOO_MANY_REQUESTS,
            Error::Generic { status, .. } => status,
        }
    }
}

impl FromResidual<Result<Infallible, actix_web::Error>> for Result<HttpResponse, Error> {
    fn from_residual(residual: Result<Infallible, actix_web::Error>) -> Self {
        let err = residual.unwrap_err();
        return Err(Error::Generic {
            status: err.as_response_error().status_code(),
            message: str!(err.as_response_error().to_string()),
        });
    }
}

impl FromResidual<Result<Infallible, reqwest::Error>> for Result<HttpResponse, Error> {
    fn from_residual(residual: Result<Infallible, reqwest::Error>) -> Self {
        let err = residual.unwrap_err();
        return Err(Error::Generic {
            status: err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            message: str!(err.to_string()),
        });
    }
}

pub(crate) fn not_found<B>(sr: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, res) = sr.into_parts();
    let res = res.set_body(create_error("Not Found", "Page not found"));
    let res = dev::ServiceResponse::new(req, res).map_into_boxed_body();
    Ok(ErrorHandlerResponse::Response(res.map_into_right_body()))
}
