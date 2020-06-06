use actix_web::web::Data;
use actix_web::{http, HttpRequest, HttpResponse, Result};

use crate::app_data::AppData;

pub async fn handle(req: HttpRequest, data: Data<AppData>) -> Result<HttpResponse> {
    let name = String::from(req.match_info().get("key").unwrap());

    let url_entry = data.url_handler.get_by_key(&name)?;

    Ok(HttpResponse::PermanentRedirect()
        .header(http::header::LOCATION, url_entry.target)
        .finish())
}
