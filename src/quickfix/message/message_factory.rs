use super::field::Field;
use super::field_key::FieldKey;
use super::message::Message;
use std::net::TcpStream;

#[derive(Debug)]
pub struct MessageFactory {
    hello_message: fn(i32) -> Message,
    order_message: fn(i64) -> Message
}

impl MessageFactory {
    pub fn new(hello_message: fn(i32) -> Message,
               order_message: fn(i64) -> Message) -> MessageFactory {
        MessageFactory {
            hello_message,
            order_message
        }
    }

    pub fn hello(&self, hart_beat: i32) -> Message {
        (self.hello_message)(hart_beat)
    }
    pub fn order(&self, hart_beat: i64) -> Message {
        (self.order_message)(hart_beat)
    }
}
