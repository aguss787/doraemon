use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::app_data::AppData;
use crate::core::sso::utils::{get_activation_url, send_activation_mail};

#[derive(Deserialize, Clone)]
pub struct ActivationCodePayload {
    code: Option<String>,
}

pub async fn handle(
    data: Data<AppData>,
    req: web::Query<ActivationCodePayload>,
) -> Result<HttpResponse> {
    if req.code.is_none() {
        display_resend_email_form(data, None)
    } else {
        activate_user(data, &req.code.to_owned().unwrap())
    }
}

fn display_resend_email_form(data: Data<AppData>, message: Option<String>) -> Result<HttpResponse> {
    let view = data
        .as_ref()
        .templater
        .resend_activation_page(&message.unwrap_or("".to_owned()))?;
    Ok(HttpResponse::Ok().body(view))
}

fn activate_user(data: Data<AppData>, activation_code: &String) -> Result<HttpResponse> {
    data.auth_handler.activate(activation_code)?;
    Ok(HttpResponse::Ok().body("Activated!"))
}

#[derive(Deserialize, Clone)]
pub struct ResendPayload {
    username: String,
}

pub async fn handle_resend(
    data: Data<AppData>,
    req: web::Form<ResendPayload>,
) -> Result<HttpResponse> {
    let request_result = data
        .auth_handler
        .get_activation_code_with_email(&req.username);

    match request_result {
        Err(e) => display_resend_email_form(data, Some(e.to_string())),
        Ok((email, activation_code)) => {
            let activation_url = get_activation_url(&data.config.auth.base_url, &activation_code);

            let mailer = data.as_ref().mailer()?;
            actix_rt::spawn(send_activation_mail(
                mailer,
                data.config.auth.email_origin.to_owned(),
                email.to_owned(),
                activation_url,
            ));

            display_resend_email_form(data, Some("Success!".to_owned()))
        }
    }
}
