use actix_web::{http::header::HeaderValue, web};
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use crate::{
    config::db::Pool,
    models::{token::UserToken, user::User},
};

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    let config = crate::CONFIG.get().unwrap();
    let secret = config.settings.secret.as_bytes();

    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(&secret), &Validation::default())
}

pub fn verify_token(token_data: &TokenData<UserToken>, pool: &web::Data<Pool>) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &mut pool.get().unwrap()) {
        Ok(token_data.claims.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}

pub fn is_auth_header_valid(authen_header: &HeaderValue) -> bool {
    if let Ok(authen_str) = authen_header.to_str() {
        return authen_str.starts_with("bearer") || authen_str.starts_with("Bearer");
    }

    return false;
}
