use crate::{config::structs::Config, models::user::LoginInfoDTO};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO, config: &Config) -> String {
        let max_age = config.settings.max_age;
        let secret = config.settings.secret.as_bytes();

        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000;
        let payload = UserToken {
            iat: now,
            exp: now + max_age,
            user: login.username.clone(),
            login_session: login.login_session.clone(),
        };

        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(&secret)).unwrap()
    }
}
