extern crate redis;
use redis::Commands;
use tokio::time::{sleep, Duration};

pub fn check_and_update_plate(redis_client: &redis::Client, plate_number: &str, confidence: f64) -> redis::RedisResult<()> {
    let mut con = redis_client.get_connection()?;
    
    // Check if the plate_number exists and get its confidence
    let stored_confidence: Option<f64> = con.get(plate_number)?;

    match stored_confidence {
        Some(stored) => {
            // Update only if new confidence is higher
            if confidence > stored {
                con.set(plate_number, confidence)?;
                println!("Updated: {} with confidence {}", plate_number, confidence);
            }
        },
        None => {
            // Insert if plate_number does not exist
            con.set(plate_number, confidence)?;
            println!("Inserted: {} with confidence {}", plate_number, confidence);
        },
    }

    // Reset the expiration time for the plate_number to 30 seconds
    con.expire(plate_number, 60)?;
    
    Ok(())
}


pub async fn check_for_expired_keys(redis_client: &redis::Client) {
    loop {
        sleep(Duration::from_secs(5)).await; // Check every 5 seconds

        let mut con = match redis_client.get_connection() {
            Ok(conn) => conn,
            Err(_) => continue,
        };

        // Fetch all keys related to plate_numbers
        // (assuming keys are stored with a pattern like "plate_number:XXX")
        let keys: Vec<String> = con.keys("plate_number:*").unwrap_or_else(|_| vec![]);

        for key in keys {
            // Check the TTL for each key
            let ttl: i64 = con.ttl(&key).unwrap_or(-1);

            if ttl <= 0 {
                // Trigger "OK" or your desired action when a plate_number expires
                println!("OK: {} has expired or does not exist.", key);
            }
        }
    }
}