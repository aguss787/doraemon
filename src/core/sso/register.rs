use actix_web::{HttpResponse, Result, web};
use actix_web::web::Data;
use serde::Deserialize;

use crate::app_data::AppData;

#[derive(Deserialize, Clone)]
pub struct UserPayload {
    username: String,
    password: String,
}

pub async fn handle(item: web::Json<UserPayload>, data: Data<AppData>) -> Result<HttpResponse> {
    data.as_ref()
        .auth()
        .register(&item.username, &item.password)?;
    Ok(HttpResponse::Ok().finish())
}
