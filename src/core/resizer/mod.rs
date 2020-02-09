use actix_web::{web, Scope};

mod form;
mod getter;
mod processor;

pub fn service(prefix: &str) -> Scope {
  web::scope(prefix)
    .route("", web::get().to(form::form))
    .route("", web::post().to(processor::process))
    .route("/{name}", web::get().to(getter::getter))
}

