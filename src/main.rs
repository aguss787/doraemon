use actix_web::{App, HttpServer, middleware};

mod core;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath)
            .wrap(middleware::Logger::default())
            .service(core::greeter::service("/greet"))
            .service(core::resizer::service("/resizer"))
    })
    .bind("0.0.0.0:8000")?
    .run();

    println!("Server started");

    server.await
}
