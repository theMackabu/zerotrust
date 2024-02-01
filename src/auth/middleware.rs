use crate::{config::db::Pool, http::token, models::user::User, schema::users};
use actix_service::forward_ready;
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::HttpResponse;
use actix_web::{guard::GuardContext, http::header::HeaderValue};
use actix_web::{http::header, Error};
use diesel::prelude::RunQueryDsl;
use futures::future::{ok, LocalBoxFuture, Ready};
use macros_rs::fmtstr;
use std::collections::BTreeMap;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future { ok(AuthenticationMiddleware { service }) }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(pool) = req.app_data::<Data<Pool>>() {
            if let Err(_) = users::table.first::<User>(&mut pool.get().unwrap()) {
                let (request, _pl) = req.into_parts();
                let header = (header::LOCATION, "/setup");
                let response = HttpResponse::TemporaryRedirect().insert_header(header).finish();
                return Box::pin(async { Ok(ServiceResponse::new(request, response.map_into_right_body())) });
            }

            if let Some(cookie) = req.cookie("sp_token") {
                let token = cookie.value();
                if let Ok(token_data) = token::decode_token(token.to_string()) {
                    if token::verify_token(&token_data, pool).is_ok() {
                        let res = self.service.call(req);
                        return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
                    }
                }
            }
        }

        let config = crate::CONFIG.get().unwrap();
        let prefix = config.settings.server.prefix.clone();
        let (request, _pl) = req.into_parts();

        let response = HttpResponse::TemporaryRedirect()
            .insert_header((header::LOCATION, fmtstr!("/{prefix}/login")))
            .finish()
            .map_into_right_body();

        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
    }
}

pub fn setup_guard(_ctx: &GuardContext<'_>) -> bool {
    let pool = crate::POOL.get().unwrap();
    match users::table.first::<User>(&mut pool.get().unwrap()) {
        Ok(_) => false,
        Err(_) => true,
    }
}

pub fn token_guard(ctx: &GuardContext<'_>) -> bool {
    let pool = crate::POOL.get().unwrap();
    let empty_header = HeaderValue::from_static("");
    let cookie_string = ctx.head().headers().get("cookie").unwrap_or(&empty_header).to_str().unwrap_or("");

    if !cookie_string.is_empty() {
        let cookies: BTreeMap<&str, &str> = cookie_string
            .split("; ")
            .map(|pair| pair.trim().split("=").collect::<Vec<&str>>())
            .map(|vec| (vec[0], vec[1]))
            .collect();

        if cookies.contains_key("sp_token") {
            match token::decode_token(cookies.get("sp_token").unwrap().to_string()) {
                Ok(token) => !User::is_valid_login_session(&token.claims, &mut pool.get().unwrap()),
                Err(_) => true,
            }
        } else {
            return true;
        }
    } else {
        return true;
    }
}
