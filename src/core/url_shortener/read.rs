use actix_web::web::{Data, Query};
use actix_web::{HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::app_data::AppData;
use crate::core::url_shortener::utils::authenticate;
use crate::database::handler::url::Url;

#[derive(Deserialize)]
pub struct GetUrlRequest {
    page: Option<i64>,
    per_page: Option<i64>,
}

#[derive(Serialize)]
pub struct GetUrlResponse {
    urls: Vec<Url>,
    page: i64,
    per_page: i64,
}

pub async fn handle(
    request: Query<GetUrlRequest>,
    data: Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let token = authenticate(&data, req)?;

    let page = request.page.unwrap_or(0);
    let per_page = request.per_page.unwrap_or(10);

    let offset = per_page * page;
    let limit = per_page;

    let urls = data
        .as_ref()
        .url_handler()
        .get_by_username(&token.username, offset, limit)?;

    Ok(HttpResponse::Ok().json(GetUrlResponse {
        urls,
        page,
        per_page,
    }))
}
