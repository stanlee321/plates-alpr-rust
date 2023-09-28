use actix_web::web::Json;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

mod kafka;
mod models;

pub async fn process_payload(data: models::MainStruct) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let hook = env::var("HOOK").expect("HOOK must be set");
    let topic = env::var("TOPIC").expect("TOPIC must be set");
    let kafka_server_address =
        env::var("KAFKA_SERVER_ADDRESS").expect("KAFKA_SERVER_ADDRESS must be set");
    let timeout = env::var("TIMEOUT").expect("TIMEOUT must be set");

    let serialized_data = serde_json::to_string(&data)?;
    match kafka::send_to_kafka(serialized_data, hook, kafka_server_address, timeout, topic).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

async fn webhook_endpoint(payload: Json<models::MainStruct>) -> impl Responder {
    println!("I received a webhook call...");
    
    match process_payload(payload.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Webhook processed"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let hook = env::var("HOOK").expect("HOOK must be set");
    let server_ip: String = env::var("SERVER_IP").expect("SERVER_IP must be set");
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");

    let server_address = server_ip + ":" + &server_port;
    let webhook_path = "/".to_owned() + &hook;

    println!("Server is running at {}", server_address);  // Add this line to print the server address

    HttpServer::new(move || {
        App::new()
            .route(
                &webhook_path,
                web::post()
                    .to(webhook_endpoint)
                    .guard(actix_web::guard::Post()),
            )
            .route("/health_check", web::get().to(health_check))
    })
    .bind(&server_address)?
    .run()
    .await
}
