use actix_service::forward_ready;
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::Error;
use actix_web::HttpResponse;
use futures::future::{ok, LocalBoxFuture, Ready};

use crate::{
    config::db::Pool,
    http::{errors, token},
};

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
            if let Some(authen_header) = req.headers().get("Authorization") {
                if let Ok(authen_str) = authen_header.to_str() {
                    if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                        let token = authen_str[6..authen_str.len()].trim();
                        if let Ok(token_data) = token::decode_token(token.to_string()) {
                            if token::verify_token(&token_data, pool).is_ok() {
                                let res = self.service.call(req);
                                return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
                            }
                        }
                    }
                }
            }
        }

        let (request, _pl) = req.into_parts();
        let response = HttpResponse::Unauthorized()
            .body(errors::create_error(StatusCode::UNAUTHORIZED, "Sorry, but this upstream resource is restricted.", Some("Unauthorized")))
            .map_into_right_body();

        return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
    }
}
