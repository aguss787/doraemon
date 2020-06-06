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
    total: i64,
}

pub async fn handle(
    request: Query<GetUrlRequest>,
    data: Data<AppData>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let token = authenticate(&data, &req)?;

    let page = request.page.unwrap_or(0);
    let per_page = request.per_page.unwrap_or(10);

    let offset = per_page * page;
    let limit = per_page;

    let urls = data
        .url_handler
        .get_by_username(&token.username, offset, limit)?;

    let total = data.url_handler.count_by_username(&token.username)?;

    Ok(HttpResponse::Ok().json(GetUrlResponse {
        urls,
        page,
        per_page,
        total,
    }))
}

pub async fn handle_one(data: Data<AppData>, req: HttpRequest) -> Result<HttpResponse> {
    let token = authenticate(&data, &req)?;
    let key = String::from(req.match_info().get("key").unwrap());

    let url = data
        .url_handler
        .get_by_key_and_username(&key, &token.username)?;

    Ok(HttpResponse::Ok().json(url))
}
