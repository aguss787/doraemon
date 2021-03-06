use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod activate;
mod authorize;
mod inspect;
mod login;
mod register;
mod token;

mod error;
mod utils;

pub fn service(prefix: &str) -> impl HttpServiceFactory {
    web::scope(prefix)
        .route("/login", web::post().to(login::handle))
        .route("/activate", web::get().to(activate::handle))
        .route("/activate", web::post().to(activate::handle_resend))
        .route("/authorize", web::post().to(authorize::handle_login))
        .route("/authorize", web::get().to(authorize::handle_form))
        .route("/token", web::post().to(token::handle))
        .route("/register", web::post().to(register::handle_register))
        .route("/register", web::get().to(register::handle_form))
        .route("/inspect", web::post().to(inspect::handle))
}
