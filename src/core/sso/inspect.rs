use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::app_data::AppData;
use crate::auth::model::Token;

#[derive(Deserialize, Clone)]
pub struct TokenPayload {
    access_token: Token,
}

pub async fn handle(item: web::Json<TokenPayload>, data: Data<AppData>) -> Result<HttpResponse> {
    let result = data.auth_handler.inspect(&item.access_token)?;
    Ok(HttpResponse::Ok().json(result))
}
