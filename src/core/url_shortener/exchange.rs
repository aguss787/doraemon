use actix_web::web::{Bytes, Data};
use actix_web::{HttpResponse, Result};
use serde::Serialize;

use crate::app_data::AppData;
use crate::auth::model::{RefreshToken, Token};

#[derive(Serialize, Clone)]
pub struct TokenResponse {
    access_token: Token,
    refresh_token: RefreshToken,
}

pub async fn handle(body: Bytes, data: Data<AppData>) -> Result<HttpResponse> {
    let auth_code = bytes_to_string(body)?;

    let client_secret = &data.as_ref().config.url.client_secret;

    let (token, refresh_token) = data
        .as_ref()
        .auth()
        .exchange_token(&auth_code, client_secret)?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: token,
        refresh_token,
    }))
}

fn bytes_to_string(raw: Bytes) -> Result<String> {
    match String::from_utf8(raw.to_vec()) {
        Err(e) => Err(actix_web::error::ErrorBadRequest(e)),
        Ok(s) => Ok(s),
    }
}
