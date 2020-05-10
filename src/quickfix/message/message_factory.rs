use super::field::Field;
use super::field_key::FieldKey;
use super::header::Header;
use super::message::Message;

#[derive(Debug)]
pub struct MessageFcactory44 {}
impl MessageFcactory44 {
    fn new() -> MessageFcactory44 {
        MessageFcactory44 {}
    }
}

pub trait MessageFactoryTrait {
    fn create(&self, msgType: String) -> Message;
}

impl MessageFactoryTrait for MessageFcactory44 {
    fn create(&self, msgType: String) -> Message {
        let mut message = Message::new(String::from("FIX.4.4"));
        let field8 = Field::new(FieldKey::begin_string(), String::from("FIX.4.4"));
        message.add_header(field8);
        let field9 = Field::new(FieldKey::msg_type(), msgType);
        message.add_header(field9);

        let mut length = 0;
        // &field_key::begin_string;
        for field in &message.header.fields {
            // if field.0 != 3 {
            // }
            // println!("{:?}", i);
        }
        // let field9 = Field::new(9, length.to_string());
        // message.add_header(field9);

        message
    }
}

#[macro_use]
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref message_factory: MessageFcactory44 = MessageFcactory44::new();
}

#[test]
fn message_factory_test() {
    // message_factory
    let a = &message_factory;
    println!(
        "{:?}",
        message_factory.create(String::from("A")).to_string()
    );

    // message_factory.create(String::from("FIX.4.4"), String::from(""), String::from(""));
}
