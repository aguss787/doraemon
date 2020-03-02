use actix_web::{http, HttpRequest, HttpResponse, Result};
use actix_web::web::Data;
use diesel::result::Error as DieselError;

use crate::AppData;
use crate::database::handler::url::get_by_key;

pub async fn handle(req: HttpRequest, data: Data<AppData>) -> Result<HttpResponse> {
    let name = String::from(req.match_info().get("key").unwrap());

    let app_data = data.get_ref();
    let url_entry_result = get_by_key(&app_data.connection, &name);

    match url_entry_result {
        Err(e @ DieselError::NotFound) => Err(actix_web::error::ErrorNotFound(e)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
        Ok(url_entry) => Ok(HttpResponse::PermanentRedirect()
            .header(http::header::LOCATION, url_entry.target)
            .finish()),
    }
}
