use actix_web::{http, HttpRequest, HttpResponse, Responder};
use actix_web::web::Data;
use diesel::{QueryDsl, RunQueryDsl, TextExpressionMethods};

use crate::AppData;

#[derive(Queryable)]
struct Url {
    _key: String,
    target: String,
}

pub async fn handler(req: HttpRequest, data: Data<AppData>) -> impl Responder {
    let name = req.match_info().get("key").unwrap();

    use crate::schema::url::dsl::*;

    let app_data = data.get_ref();
    let url_entry_result = url
        .filter(key.like(name))
        .first::<Url>(&app_data.connection);

    match url_entry_result {
        Err(_) => HttpResponse::NotFound().body("not found"),
        Ok(url_entry) => HttpResponse::PermanentRedirect()
            .header(http::header::LOCATION, url_entry.target)
            .finish(),
    }
}
