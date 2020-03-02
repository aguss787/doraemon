use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod inspect;
mod login;
mod register;

pub fn service(prefix: &str) -> impl HttpServiceFactory {
    web::scope(prefix)
        .route("/login", web::post().to(login::handle))
        .route("/register", web::post().to(register::handle))
        .route("/inspect", web::post().to(inspect::handle))
}
