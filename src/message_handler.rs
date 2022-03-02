use crate::message::Message;

pub fn handle(message: Message) {
    println!("subscribe: {:?}", message);
}
