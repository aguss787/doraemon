use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod redirect_by_key;

pub fn service(prefix: &str) -> impl HttpServiceFactory {
    web::scope(prefix).route("/{key}", web::get().to(redirect_by_key::handler))
}
