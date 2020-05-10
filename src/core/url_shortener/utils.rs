use crate::app_data::AppData;
use crate::auth::model::TokenPayload;
use crate::auth::AuthError;
use actix_web::web::Data;
use actix_web::HttpRequest;

pub(crate) fn authenticate(
    data: &Data<AppData>,
    req: HttpRequest,
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
