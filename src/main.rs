#[macro_use]
extern crate diesel;

use std::fmt::Error;

use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use diesel::PgConnection;

use crate::config::{Config, get_config};
use crate::database::establish_connection;

mod config;

mod core;

mod database;
mod schema;

pub struct AppData {
    connection: PgConnection,
}

fn init(config: Config) -> Result<AppData, Error> {
    Ok(AppData {
        connection: establish_connection(&config),
    })
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
            .default_service(web::to(|| HttpResponse::NotFound().body("404")))
    })
    .bind("0.0.0.0:8000")?
    .run();

    println!("Server started");

    server.await
}
