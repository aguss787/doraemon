use actix_web::HttpResponse;

pub async fn form() -> HttpResponse {
    let html = r#"<html>
      <head><title>Upload Test</title></head>
      <body>
          <form method="post" enctype="multipart/form-data">
              <input type="file" multiple name="file"/>
              <input type="submit" value="Submit"></button>
          </form>
      </body>
  </html>"#;

    HttpResponse::Ok().body(html)
}
