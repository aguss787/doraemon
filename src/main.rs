#[macro_use]
extern crate diesel;
#[macro_use]
extern crate magic_crypt;

use std::fmt::Error;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use crate::app_data::AppData;
use crate::config::{get_config, Config};
use crate::database::establish_connection;
use actix_cors::Cors;

mod config;

mod app_data;
mod auth;
mod core;

mod database;
mod schema;

mod templater;

fn init(config: Config) -> Result<AppData, Error> {
    let connection = establish_connection(&config);

    Ok(AppData::new(connection, &config))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_config();

    let server = HttpServer::new(move || {
        App::new()
            .data(init(config.clone()).unwrap())
            .wrap(middleware::NormalizePath)
            .wrap(middleware::Logger::default())
            .wrap(Cors::new().supports_credentials().max_age(3600).finish())
            .service(core::greeter::service("/greet"))
            .service(core::resizer::service("/resizer"))
            .service(core::url_shortener::service("/url"))
            .service(core::sso::service("/sso"))
            .default_service(web::to(|| HttpResponse::NotFound().body("404")))
    })
    .bind("0.0.0.0:8000")?
    .run();

    println!("Server started");

    server.await
}
