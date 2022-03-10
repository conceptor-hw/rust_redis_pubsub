mod message;
mod message_handler;
mod redis_publisher;
mod redis_subscriber;
use std::time::Duration;
extern crate redis;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("service started");

    if let Err(error) = redis_subscriber::subscribe(String::from(message::SUB_BINARY_CHANNEL)) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!(
            "connected to queue subscribe {:?}",
            message::SUB_BINARY_CHANNEL
        );
    }

    if let Err(error) = redis_subscriber::subscribe(String::from(message::SUB_MGT_CHANNEL)) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!(
            "connected to queue subscribe {:?}",
            message::SUB_MGT_CHANNEL
        );
    }

    std::thread::sleep(Duration::from_secs(1));
    let mut i = 0;

    while i <= 10000 {
        redis_publisher::publish_message(
            message::PUB_BINARY_CHANNEL,
            message::ProverMessage::new(i as u32, i as i64, i as u64),
        )?;

        redis_publisher::publish_normal_message(message::PubSubMessage::new(message::Order::new(
            "message from rust".to_string(),
            0,
            i,
        )))?;
        std::thread::sleep(Duration::from_secs(1));
        i = i + 1;
    }

    Ok(())
}
