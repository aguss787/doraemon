use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use crate::app_data::AppData;
use crate::core::url_shortener::utils::authenticate;

#[derive(Deserialize)]
pub struct DeleteUrlRequest {
    key: String,
}

pub async fn handle(
    request: Json<DeleteUrlRequest>,
    data: Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let token = authenticate(&data, &req)?;

    data.url_handler
        .delete_at_least_one(&request.key, &token.username)?;

    Ok(HttpResponse::Ok().finish())
}
