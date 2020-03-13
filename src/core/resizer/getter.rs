use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use actix_web::{HttpRequest, HttpResponse};

pub async fn getter(req: HttpRequest) -> std::io::Result<HttpResponse> {
    match req.match_info().get("name") {
        None => Ok(HttpResponse::NotFound().body("not found")),
        Some(name) => {
            let file = File::open(format!("./data/resizer/{}", name))?;
            let mut buf_reader = BufReader::new(file);
            let mut bytes = Vec::new();
            buf_reader.read_to_end(&mut bytes)?;
            Ok(HttpResponse::Ok().body(bytes))
        }
    }
}
