mod message;
mod message_handler;
mod redis_publisher;
mod redis_subscriber;
use std::time::Duration;
extern crate redis;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("service started");

    if let Err(error) = redis_subscriber::subscribe(String::from("go_channel")) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }
    std::thread::sleep(Duration::from_secs(1));
    let mut i = 0;
    while i <= 10000 {
        redis_publisher::publish_message(message::Message::new(
            message::Order::new("message from rust".to_string(), 0, i),
            "rust_channel".to_string(),
        ))?;

        std::thread::sleep(Duration::from_secs(1));
        i = i + 1;
    }

    Ok(())
}
