extern crate redis;

use crate::message::Message;
use crate::message_handler;
use redis::{ControlFlow, PubSubCommands};
use std::error::Error;

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let mut con = client.get_connection().unwrap();

        let _: () = con
            .subscribe(&[channel], |msg| {
                let received: String = msg.get_payload().unwrap();
                let message_obj = serde_json::from_str::<Message>(&received).unwrap();

                message_handler::handle(message_obj);

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}
