extern crate redis;
use crate::message::ProverMessage;
use crate::message::PubSubMessage;
use bincode;
use redis::Commands;
use std::error::Error;

pub fn publish_message(channel: &str, message: ProverMessage) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;
    println!("publishing message to go channel {:?}", message);
    let serial_data = bincode::serialize(&message).unwrap();
    con.publish(channel, serial_data)?;

    Ok(())
}

pub fn publish_normal_message(message: PubSubMessage) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    let json = serde_json::to_string(&message)?;

    con.publish(message.channel, json)?;

    Ok(())
}
