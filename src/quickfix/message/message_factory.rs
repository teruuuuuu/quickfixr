use super::field::Field;
use super::field_key::FieldKey;
use super::message::Message;
use std::net::TcpStream;

#[derive(Debug)]
pub struct MessageFactory {
    logon_message: fn(i32) -> Message,
    order_message: fn(i32, i64) -> Message,
}

impl MessageFactory {
    pub fn new(
        logon_message: fn(i32) -> Message,
        order_message: fn(i32, i64) -> Message,
    ) -> MessageFactory {
        MessageFactory {
            logon_message,
            order_message,
        }
    }

    pub fn logon(&self, hart_beat: i32) -> Message {
        (self.logon_message)(hart_beat)
    }
    pub fn order(&self, msg_seq: i32, hart_beat: i64) -> Message {
        (self.order_message)(msg_seq, hart_beat)
    }
}
