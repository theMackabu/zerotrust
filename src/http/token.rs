use actix_web::web::Data;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

use crate::{
    config::{db::Pool, structs::Config},
    models::{token::UserToken, user::User},
};

pub fn decode_token(token: String, config: &Config) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    let secret = config.settings.secret.as_bytes();
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(&secret), &Validation::default())
}

pub fn verify_token(token_data: &TokenData<UserToken>, pool: &Data<Pool>) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &mut pool.get().unwrap()) {
        Ok(token_data.claims.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}
