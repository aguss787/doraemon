use actix_web::{HttpResponse, Result, web};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};

use crate::app_data::AppData;
use crate::auth::Token;

#[derive(Deserialize, Clone)]
pub struct UserPayload {
    username: String,
    password: String,
}

#[derive(Serialize, Clone)]
pub struct TokenResponse {
    access_token: Token,
}

pub async fn handle(item: web::Json<UserPayload>, data: Data<AppData>) -> Result<HttpResponse> {
    let token = data
        .as_ref()
        .auth()
        .authorize(&item.username, &item.password)?;
    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: token,
    }))
}
