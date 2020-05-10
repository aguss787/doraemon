use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod utils;

mod create;
mod delete;
mod read;
mod update;

mod exchange;
mod redirect_by_key;

pub fn service(prefix: &str) -> impl HttpServiceFactory {
    web::scope(prefix)
        .route("/redirect/{key}", web::get().to(redirect_by_key::handle))
        .route("/exchange", web::post().to(exchange::handle))
        .route("/", web::post().to(create::handle))
        .route("/", web::get().to(read::handle))
        .route("/", web::delete().to(delete::handle))
        .route("/", web::patch().to(update::handle))
}
