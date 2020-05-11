use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::app_data::AppData;

#[derive(Deserialize, Clone)]
pub struct UserPayload {
    username: String,
    password: String,
}

pub async fn handle_register(
    data: Data<AppData>,
    req: web::Form<UserPayload>,
) -> Result<HttpResponse> {
    data.as_ref()
        .auth()
        .register(&req.username, &req.password)?;
    Ok(HttpResponse::Ok().body("Register completed! Please return to login page. No, I haven't implement redirect to login page"))
}

pub async fn handle_form(
    data: Data<AppData>,
) -> Result<HttpResponse> {
    let template = data
        .templater
        .register_page()?;
    Ok(HttpResponse::Ok().body(template))
}
