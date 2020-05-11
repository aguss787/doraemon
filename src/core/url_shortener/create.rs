use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use crate::app_data::AppData;
use crate::core::url_shortener::utils::authenticate;

#[derive(Deserialize)]
pub struct CreateUrlRequest {
    key: String,
    target: String,
}

pub async fn handle(
    request: Json<CreateUrlRequest>,
    data: Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let token = authenticate(&data, &req)?;

    data.as_ref()
        .url_handler()
        .insert(&request.key, &request.target, &token.username)?;
    Ok(HttpResponse::Ok().finish())
}
