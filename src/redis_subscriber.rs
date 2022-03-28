extern crate redis;
// use crate::message;
use crate::message::ProveSpecMessage;
use crate::message::ProverMessage;
use crate::message::PubSubMessage;
use crate::{message, message_handler};
// use crate::message_handler;
use bincode;
use redis::{ControlFlow, PubSubCommands};
use std::error::Error;

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let mut con = client.get_connection().unwrap();

        let _: () = con
            .subscribe(&[channel], |msg| {
                let from_channel = msg.get_channel_name();
                println!("channel name is {:?}", from_channel);
                match from_channel {

                    // from go controller message
                    "mgt_channel_schedule" => {
                        let received: String = msg.get_payload().unwrap();
                        let message_obj = serde_json::from_str::<PubSubMessage>(&received).unwrap();
                        println!("subcribe message 22222222....{:?}", message_obj);
                        // message_handler::handle(message_obj);
                    }
                    message::SUB_BINARY_CHANNEL => {
                        let received: String = msg.get_payload().unwrap();
                        let message_obj =
                            serde_json::from_str::<ProveSpecMessage>(&received).unwrap();
                        let prov_msg:ProverMessage = serde_json::from_str(&message_obj.Info).unwrap();
                        match prov_msg{
                            ProverMessage::Notify(blockTemp,difficulty ) =>{
                                println!("message is Nofity BlockTmp{:?} diffculty{} prover_id {}",blockTemp,difficulty,message_obj.Prover_id)
                            }
                            _=>{
                                println!("some thing was wrong....")
                            }
                        }

                    }
                    _ => println!("something may be wrong..."),
                }

                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}

