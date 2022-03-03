use crate::message::Message;

pub fn handle(message: Message) {
    println!(
        "subscribe: id{} channel {} desciption {} index:{}",
        message.id, message.channel, message.payload.description, message.payload.index,
    );
}
