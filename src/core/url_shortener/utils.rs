use crate::app_data::AppData;
use crate::auth::model::TokenPayload;
use crate::auth::AuthError;
use actix_web::web::Data;
use actix_web::HttpRequest;
use regex::Regex;

pub fn authenticate(
    data: &Data<AppData>,
    req: &HttpRequest,
) -> actix_web::Result<TokenPayload> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(actix_web::error::ErrorUnauthorized(AuthError::InvalidToken))?
        .to_str()
        .map_err(|_| AuthError::InvalidToken)?
        .to_owned();

    Ok(data.as_ref().auth().inspect(&auth_header)?)
}

pub fn is_valid_url_key(key: &String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[A-Za-z0-9_\\-\\.]+$").unwrap();
    }
    RE.is_match(key)
}
