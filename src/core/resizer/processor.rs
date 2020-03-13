use std::vec::Vec;

use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{http, web, Error as AcError, HttpResponse};
use futures::StreamExt;
use image;
use image::ImageError;
use rand;
use rand::Rng;

pub async fn process(mut payload: Multipart) -> Result<HttpResponse, AcError> {
    match payload.next().await {
        None => Ok(HttpResponse::build(StatusCode::BAD_REQUEST).body("haha")),
        Some(item) => {
            let mut field = item?;
            let content_type = field.content_disposition().unwrap();

            // Field in turn is stream of *Bytes* object
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                data.extend_from_slice(&chunk.unwrap())
            }
            let resized_image = resize(&data).await.unwrap();

            let mut rng = rand::thread_rng();
            let filename = format!(
                "rsz-{}-{}",
                rng.gen::<u64>(),
                content_type.get_filename().unwrap()
            );
            let filepath = format!("./data/resizer/{}", filename);

            web::block(move || resized_image.save(filepath)).await?;

            Ok(HttpResponse::SeeOther()
                .header(http::header::LOCATION, format!("resizer/{}", filename))
                .body(""))
        }
    }
}

async fn resize(buffer: &[u8]) -> Result<image::DynamicImage, ImageError> {
    let image = image::load_from_memory(buffer)?;
    let result = image.resize(250, 250, image::imageops::FilterType::Gaussian);
    Ok(result)
}
