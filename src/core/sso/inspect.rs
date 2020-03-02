use actix_web::{HttpResponse, Result, web};
use actix_web::web::Data;
use serde::Deserialize;

use crate::AppData;
use crate::auth::Token;

#[derive(Deserialize, Clone)]
pub struct TokenPayload {
    access_token: Token,
}

pub async fn handle(item: web::Json<TokenPayload>, data: Data<AppData>) -> Result<HttpResponse> {
    let result = data.as_ref().auth.inspect(&item.access_token)?;
    Ok(HttpResponse::Ok().json(result))
}
