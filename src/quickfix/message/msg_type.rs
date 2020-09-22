use crate::quickfix::message::message::Message;
#[macro_use]
use lazy_static::lazy_static;
use crate::quickfix::message::message_fix44::logon_message;
use std::borrow::Borrow;

pub struct MsgType(String);

impl MsgType {
    pub fn unwrap(&self) -> &String {
        &self.0
    }
}

lazy_static! {
    pub static ref heart_beat: MsgType = MsgType(String::from("0"));
    pub static ref test_request: MsgType = MsgType(String::from("1"));
    pub static ref reject: MsgType = MsgType(String::from("3"));
    pub static ref sequence_reset: MsgType = MsgType(String::from("4"));
    pub static ref logout: MsgType = MsgType(String::from("5"));
    pub static ref logon: MsgType = MsgType(String::from("A"));
    pub static ref quote_request: MsgType = MsgType(String::from("R"));
    pub static ref quote: MsgType = MsgType(String::from("S"));
}

pub fn is_heart_beat(message: Message) -> bool {
    message.get_msg_type().unwrap().data.eq(heart_beat.unwrap())
}
pub fn is_test_request(message: Message) -> bool {
    message
        .get_msg_type()
        .unwrap()
        .data
        .eq(test_request.unwrap())
}
pub fn is_reject(message: Message) -> bool {
    message.get_msg_type().unwrap().data.eq(reject.unwrap())
}
pub fn is_sequence_reset(message: Message) -> bool {
    message
        .get_msg_type()
        .unwrap()
        .data
        .eq(sequence_reset.unwrap())
}
pub fn is_logout(message: Message) -> bool {
    message.get_msg_type().unwrap().data.eq(logout.unwrap())
}
pub fn is_logon(message: Message) -> bool {
    message.get_msg_type().unwrap().data.eq(logon.unwrap())
}
pub fn is_quote_request(message: Message) -> bool {
    message
        .get_msg_type()
        .unwrap()
        .data
        .eq(quote_request.unwrap())
}
pub fn is_quote(message: Message) -> bool {
    message.get_msg_type().unwrap().data.eq(quote.unwrap())
}

#[test]
fn test() {
    let message = logon_message(1);
    assert!(is_logon(message));
}
