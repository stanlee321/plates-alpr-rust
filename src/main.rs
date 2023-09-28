use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use serde::Deserialize;
use actix_web::web::Bytes;

// Import module from another file
mod models;

async fn webhook_endpoint(data: Bytes) -> impl Responder {
    let raw_data_str = String::from_utf8_lossy(&data);
    println!("Received data: {}", raw_data_str);
    
    HttpResponse::Ok().body("Webhook received")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/webhook", web::post().to(webhook_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
