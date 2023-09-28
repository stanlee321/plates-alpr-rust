extern crate rdkafka;

use std::env;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use futures_util::stream::StreamExt;
use rdkafka::Message;

mod models;
mod check_redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv::dotenv().ok();
    let kafka_server_address = env::var("KAFKA_SERVER_ADDRESS").expect("KAFKA_SERVER_ADDRESS must be set");
    let topic = env::var("TOPIC").expect("TOPIC must be set");
    println!("Kafka Address: {}", kafka_server_address);
    println!("Topic: {}", topic);

    let client = redis::Client::open("redis://127.0.0.1/")?;

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_server_address)
        .set("group.id", "rust-consumer-group")
        .create()?;
    
    consumer.subscribe(&[&topic])?;

     // Clone the client for use in the spawned task
     let client_clone = client.clone();

    tokio::spawn(async move {
        check_redis::check_for_expired_keys(&client).await;
    });

    let mut message_stream = consumer.stream();
    while let Some(message) = message_stream.next().await {
        match message {
            Err(_) => println!("Error while receiving from Kafka"),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(_)) => {
                        println!("Could not deserialize message payload");
                        continue;
                    }
                };

                // println!("Received message: {}", payload);
                match serde_json::from_str::<models::MainStruct>(payload) {
                    Ok(deserialized_data) => {
                        // println!("Deserialized data: {:?}", deserialized_data);
                        let best_confidence = deserialized_data.best_confidence;
                        let best_plate_number = deserialized_data.best_plate_number.clone(); // Assuming it's a String, otherwise you don't need to clone

                        println!("Best Confidence: {}", best_confidence);
                        println!("Best Plate Number: {}", best_plate_number);

                        check_redis::check_and_update_plate(&client_clone, &best_plate_number, *&best_confidence)?;

                    }
                    Err(e) => {
                        println!("Couldn't deserialize payload: {}", e);
                    }
                }
                // Here, you can process the payload as per your requirement
            }
        };
    }

    Ok(())
}
