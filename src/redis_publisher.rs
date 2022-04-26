extern crate redis;
use crate::message;
use crate::message::ProveSpecMessage;
use crate::message::ProverMessage;
use crate::message::PubSubMessage;
use bincode;
use redis::Commands;
use std::error::Error;

// pub fn publish_message(channel: &str, message: ProverMessage) -> Result<(), Box<dyn Error>> {
//     let client = redis::Client::open("redis://localhost:6379")?;
//     let mut con = client.get_connection()?;
//     println!("publishing message to go channel {:?}", message);
//     let serial_data = bincode::serialize(&message).unwrap();
//     con.publish(channel, serial_data)?;

//     Ok(())
// }

pub fn publish_normal_message(message: PubSubMessage) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    let json = serde_json::to_string(&message)?;

    con.publish(message.channel, json)?;

    Ok(())
}

pub fn publist_prover_message() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let mut con = client.get_connection()?;

    let spe_msg: String =
        ProverMessage::Notify(message::BlockTemplate::new(110, 110, 1220), (0)).to_string();
    println!("povemessage to string is{}", spe_msg);

    let serial_data = bincode::serialize(&spe_msg).unwrap();
    con.publish(message::PUB_BINARY_CHANNEL, serial_data)?;

    Ok(())
}

// pub fn publish_submit_result_message(prover_id: String, msg: ProverMessage) -> Result<(), Box<dyn Error>> {
//     println!("publishing prover:{} submit result message to channel",prover_id);
//     let client = redis::Client::open("redis://localhost:6379")?;
//     let mut con = client.get_connection()?;
//     let _info = bincode::serialize(&msg).unwrap();
//     let pub_msg =ProveSpecMessage::new(prover_id, &_info);
//     let serial_data = bincode::serialize(&pub_msg).unwrap();
//     con.publish(message::PUB_PROVER_SPEC_MESSAGE, serial_data)?;

//     Ok(())
// }
