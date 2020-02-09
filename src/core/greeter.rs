use actix_web::{web, HttpRequest, HttpResponse, Responder, Scope};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body(format!("Hello {}!", &name))
}

pub fn service(prefix: &str) -> Scope {
    web::scope(prefix)
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
}
