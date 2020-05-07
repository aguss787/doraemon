use actix_web::{error, http, HttpResponse, Result, web};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::app_data::AppData;
use crate::auth::model::AuthCode;

#[derive(Deserialize, Clone)]
pub struct UserPayload {
    username: String,
    password: String,
    client_id: String,
    redirect_uri: String,
}

#[derive(Serialize, Clone)]
pub struct AuthCodeResponse {
    access_token: AuthCode,
}

pub async fn handle_login(
    data: Data<AppData>,
    req: web::Form<UserPayload>,
) -> Result<HttpResponse> {
    let auth_code = data.as_ref().auth().get_authorization_code(
        &req.username,
        &req.password,
        &req.client_id,
        &req.redirect_uri,
    )?;

    let redirect_uri = match Url::parse_with_params(&req.redirect_uri, &[("auth_code", auth_code)])
    {
        Ok(url) => Ok(url.into_string()),
        Err(e) => Err(error::ErrorBadRequest(e)),
    }?;

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, redirect_uri)
        .finish())
}

#[derive(Deserialize, Serialize)]
pub struct LoginFormParam {
    client_id: String,
    redirect_uri: String,
}

pub async fn handle_form(
    query: web::Query<LoginFormParam>,
    data: Data<AppData>,
) -> Result<HttpResponse> {
    let template = data
        .templater
        .login_page(&query.client_id, &query.redirect_uri)?;
    Ok(HttpResponse::Ok().body(template))
}
