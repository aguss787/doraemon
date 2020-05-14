use actix_web::web::Data;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use crate::app_data::AppData;
use crate::core::sso::error::SsoError;
use actix_web::cookie::Cookie;

#[derive(Deserialize, Clone)]
pub struct UserPayload {
    username: String,
    password: String,
}

pub async fn handle_register(
    data: Data<AppData>,
    payload: web::Form<UserPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    req.cookie("csrf")
        .ok_or(SsoError::CookieNotFound)?;

    data.as_ref()
        .auth()
        .register(&payload.username, &payload.password)?;

    Ok(HttpResponse::Ok().body("Register completed! Please return to login page. No, I haven't implement redirect to login page"))
}

pub async fn handle_form(data: Data<AppData>) -> Result<HttpResponse> {
    let template = data.templater.register_page()?;

    let cookie = Cookie::build("csrf", "")
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .body(template)
    )
}
