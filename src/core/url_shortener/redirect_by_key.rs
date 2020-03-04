use actix_web::{http, HttpRequest, HttpResponse, Result};
use actix_web::web::Data;
use diesel::result::Error as DieselError;

use crate::app_data::AppData;

pub async fn handle(req: HttpRequest, data: Data<AppData>) -> Result<HttpResponse> {
    let name = String::from(req.match_info().get("key").unwrap());

    let url_entry_result = data.get_ref().url_handler().get_by_key(&name);

    match url_entry_result {
        Err(e @ DieselError::NotFound) => Err(actix_web::error::ErrorNotFound(e)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
        Ok(url_entry) => Ok(HttpResponse::PermanentRedirect()
            .header(http::header::LOCATION, url_entry.target)
            .finish()),
    }
}
