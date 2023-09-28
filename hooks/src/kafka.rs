use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

use std::time::Duration;
use rdkafka::util::Timeout;

// "192.168.1.3:9092"
pub async fn send_to_kafka(data: String, hook: String, server: String, timeout: String, topic: String) -> Result<(), rdkafka::error::KafkaError> {
    // Debug print
    // println!("Sending data to Kafka...");
    // println!("Hook: {}", hook);
    // println!("Server: {}", server);
    // println!("Timeout: {}", timeout);
    // println!("Topic: {}", topic);

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &server)
        .set("message.timeout.ms", &timeout)
        .create()?;
    
    let delivery_status = producer
        .send(
            FutureRecord::to(&topic)
                .payload(&data)
                .key(&hook), 
            Timeout::After(Duration::from_secs(20))
        )
        .await;
    match delivery_status {
        Err((e, _)) => Err(e),
        Ok(_) => Ok(()),
    }
}
