mod message;
mod message_handler;
mod redis_publisher;
mod redis_subscriber;
use std::time::Duration;
extern crate redis;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("service started");

    if let Err(error) =  redis_subscriber::subscribe(String::from(message::SUB_PROVER_SPEC_MESSAGE)) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue subscribe {:?}", message::SUB_PROVER_SPEC_MESSAGE);
    }

    std::thread::sleep(Duration::from_secs(1));
    let mut i = 0;

    while i <= 10000 {
        redis_publisher::publish_submit_result_message(
            "test prover id from pool server".to_string(),
            message::ProverMessage::new(i as u32, i as i64, i as u64),
        )?;

        std::thread::sleep(Duration::from_secs(1));
        i = i + 1;
    }

    Ok(())
}
