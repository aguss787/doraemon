use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::app_data::AppData;
use crate::auth::model::{RefreshToken, Token};

#[derive(Deserialize, Clone)]
pub struct UserPayload {
    username: String,
    password: String,
}

#[derive(Serialize, Clone)]
pub struct TokenResponse {
    access_token: Token,
    refresh_token: RefreshToken,
}

pub async fn handle(item: web::Json<UserPayload>, data: Data<AppData>) -> Result<HttpResponse> {
    let (token, refresh_token) = data
        .auth_handler
        .get_token(&item.username, &item.password)?;
    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: token,
        refresh_token,
    }))
}
