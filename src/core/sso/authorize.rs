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

#[derive(Deserialize)]
pub struct LoginFormParam {
    client_id: String,
    redirect_uri: String,
}

pub async fn handle_form(query: web::Query<LoginFormParam>) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        r#"<html>
              <head><title>Login Test</title></head>
              <body>
                  <form method="post">
                      <input type="text" name="username"/>
                      <input type="password" name="password"/>
                      <input type="hidden" name="client_id" value="{}"/>
                      <input type="hidden" name="redirect_uri" value="{}"/>
                      <input type="submit" value="Submit"></button>
                  </form>
              </body>
          </html>"#,
        query.client_id, query.redirect_uri,
    ))
}
