use actix_web::{web};
use actix_web::dev::HttpServiceFactory;

mod form;
mod getter;
mod processor;

pub fn service(prefix: &str) -> impl HttpServiceFactory {
  web::scope(prefix)
    .route("", web::get().to(form::form))
    .route("", web::post().to(processor::process))
    .route("/{name}", web::get().to(getter::getter))
}

