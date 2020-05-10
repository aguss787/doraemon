use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::app_data::AppData;
use crate::core::url_shortener::utils::authenticate;
use crate::database::handler::url::Url;

#[derive(Deserialize)]
pub struct UpdateUrlRequest {
    old_key: String,
    key: String,
    target: String,
}

#[derive(Serialize)]
pub struct UpdateUrlResponse {
    url: Url,
}

pub async fn handle(
    request: Json<UpdateUrlRequest>,
    data: Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let token = authenticate(&data, req)?;

    let url = data.as_ref().url_handler().update(
        &request.old_key,
        &token.username,
        &request.key,
        &request.target,
    )?;

    Ok(HttpResponse::Ok().json(UpdateUrlResponse { url }))
}
