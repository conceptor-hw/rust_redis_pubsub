extern crate redis;
// use crate::message;
use crate::message::ProverMessage;
use crate::message::PubSubMessage;
// use crate::message_handler;
use bincode;
use redis::{ControlFlow, PubSubCommands};
use std::error::Error;

pub fn handle_message_for_binnary_channel() {}
pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let mut con = client.get_connection().unwrap();

        let _: () = con
            .subscribe(&[channel], |msg| {
                let from_channel = msg.get_channel_name();
                println!("channel name is {:?}", from_channel);
                match from_channel {
                    // from go transport pool server ProverMessage
                    "binary_channel_schedule" => {
                        let paylaod = msg.get_payload_bytes();
                        let message_obj: ProverMessage = bincode::deserialize(paylaod).unwrap();
                        println!("subcribe message 11111111111....{:?}", message_obj);
                        // message_handler::handle(message_obj);
                    }
                    // from go controller message
                    "mgt_channel_schedule" => {
                        let received: String = msg.get_payload().unwrap();
                        let message_obj = serde_json::from_str::<PubSubMessage>(&received).unwrap();
                        println!("subcribe message 22222222....{:?}", message_obj);
                        // message_handler::handle(message_obj);
                    }
                    _ => println!("something may be wrong..."),
                }

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}

// pub fn start() {
//     // start subscribe for redis
//     if let Err(error) = subscribe(String::from("go_channel")) {
//         println!("subscribe something was wrong{:?}", error);
//         panic!("{:?}", error);
//     } else {
//         println!("connected to queue");
//     }
// }
