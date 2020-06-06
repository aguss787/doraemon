use actix_web::web::Data;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use crate::app_data::AppData;
use crate::core::sso::error::SsoError;
use crate::core::sso::utils::{get_activation_url, send_activation_mail};
use actix_web::cookie::Cookie;

#[derive(Deserialize, Clone)]
pub struct UserPayload {
    username: String,
    email: String,
    password: String,
}

pub async fn handle_register(
    data: Data<AppData>,
    payload: web::Form<UserPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    req.cookie("csrf").ok_or(SsoError::CookieNotFound)?;

    let auth = &data.auth_handler;

    auth.register(&payload.username, &payload.email, &payload.password)?;
    let activation_code = auth.generate_activation_code(&payload.username)?;
    let activation_url = get_activation_url(&data.config.auth.base_url, &activation_code);

    let mailer = data.as_ref().mailer()?;
    actix_rt::spawn(send_activation_mail(
        mailer,
        data.config.auth.email_origin.to_owned(),
        payload.email.to_owned(),
        activation_url,
    ));

    Ok(HttpResponse::Ok()
        .body("Register completed! Please check your email for the activation link"))
}

pub async fn handle_form(data: Data<AppData>) -> Result<HttpResponse> {
    let template = data.templater.register_page()?;

    let cookie = Cookie::build("csrf", "")
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).body(template))
}
