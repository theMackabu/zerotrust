use actix_web::{http::header::HeaderValue, web};
use macros_rs::str;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    config::db::Pool,
    http::{errors::Error, token},
    models::{
        token::UserToken,
        user::{LoginDTO, LoginInfoDTO, User, UserDTO},
    },
};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserDTO, pool: &web::Data<Pool>) -> Result<String, Error> {
    match User::signup(user, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(Error::BadClientData { message: str!(message) }),
    }
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<TokenBodyResponse, Error> {
    match User::login(login, &mut pool.get().unwrap()) {
        Some(logged_user) => match serde_json::from_value(json!({ "token": UserToken::generate_token(&logged_user), "token_type": "bearer" })) {
            Ok(token_res) => {
                if logged_user.login_session.is_empty() {
                    Err(Error::Unauthorized {
                        message: "Wrong username or password, please try again!",
                    })
                } else {
                    Ok(token_res)
                }
            }
            Err(err) => Err(Error::InternalError { message: str!(err.to_string()) }),
        },
        None => Err(Error::Unauthorized {
            message: "Wrong username or password, please try again!",
        }),
    }
}

pub fn logout(authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), Error> {
    if let Ok(authen_str) = authen_header.to_str() {
        if token::is_auth_header_valid(authen_header) {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token::decode_token(token.to_string()) {
                if let Ok(username) = token::verify_token(&token_data, pool) {
                    if let Ok(user) = User::find_user_by_username(&username, &mut pool.get().unwrap()) {
                        User::logout(user.id, &mut pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(Error::InternalError {
        message: "Error while processing token, please try again!",
    })
}

pub fn me(authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<LoginInfoDTO, Error> {
    if let Ok(authen_str) = authen_header.to_str() {
        if token::is_auth_header_valid(authen_header) {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token::decode_token(token.to_string()) {
                if let Ok(login_info) = User::find_login_info_by_token(&token_data.claims, &mut pool.get().unwrap()) {
                    return Ok(login_info);
                }
            }
        }
    }

    Err(Error::InternalError {
        message: "Error while processing token, please try again!",
    })
}
