use actix_web::{HttpResponse, Result, web};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};

use crate::app_data::AppData;
use crate::auth::model::{RefreshToken, Token};

#[derive(Deserialize, Clone)]
pub struct TokenPayload {
    auth_code: String,
    client_secret: String,
}

#[derive(Serialize, Clone)]
pub struct TokenResponse {
    access_token: Token,
    refresh_token: RefreshToken,
}

pub async fn handle(item: web::Json<TokenPayload>, data: Data<AppData>) -> Result<HttpResponse> {
    let (token, refresh_token) = data
        .as_ref()
        .auth()
        .exchange_token(&item.auth_code, &item.client_secret)?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: token,
        refresh_token,
    }))
}
