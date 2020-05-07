#[macro_use]
extern crate diesel;
#[macro_use]
extern crate magic_crypt;

use std::fmt::Error;

use actix_web::{App, HttpResponse, HttpServer, middleware, web};

use crate::app_data::AppData;
use crate::config::{Config, get_config};
use crate::database::establish_connection;

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
