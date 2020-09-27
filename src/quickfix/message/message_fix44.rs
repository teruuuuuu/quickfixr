use chrono::Utc;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use crate::quickfix::message::field::Field;
use crate::quickfix::message::field_key::{
    BEGIN_STRING, CLORDID, ENCRYPT_METHOD, HANDLINST, HEART_BEAT_INTERVAL, MSG_SEQ_NUM, MSG_TYPE,
    ORDER_QTY, ORDER_TYPE, RESET_SEQ_NUM_FLG, SENDER_CMP_ID, SENDING_TIME, SIDE, SYMBOL,
    TARGET_CMP_ID, TIME_IN_FORCE, TRANSACT_TIME,
};
use crate::quickfix::message::message::Message;
use crate::quickfix::message::message_factory::MessageFactory;
use crate::quickfix::message::msg_type::heart_beat;

pub fn create_factory44() -> MessageFactory {
    MessageFactory::new(logon_message, order_message)
}

pub fn logon_message(hart_beat: i32) -> Message {
    // let message = "8=FIX.4.49=6535=A34=149=BANZAI52=20200506-12:10:57.99856=EXEC98=0108=30141=Y10=228";
    let mut logon = Message::new();
    logon.add(Field::new(*BEGIN_STRING, String::from("FIX.4.4")));
    logon.add(Field::new(*MSG_TYPE, String::from("A")));
    logon.add(Field::new(
        *SENDING_TIME,
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    logon.add(Field::new(*ENCRYPT_METHOD, String::from("0")));
    logon.add(Field::new(*HEART_BEAT_INTERVAL, hart_beat.to_string()));
    logon.add(Field::new(*RESET_SEQ_NUM_FLG, String::from("Y")));
    logon
}

pub fn logout_message() -> Message {
    let mut logon = Message::new();
    logon.add(Field::new(*BEGIN_STRING, String::from("FIX.4.4")));
    logon.add(Field::new(*MSG_TYPE, String::from("5")));
    logon.add(Field::new(
        *SENDING_TIME,
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    logon
}

pub fn heart_beat_message() -> Message {
    let mut message = Message::new();
    message.add(Field::new(*BEGIN_STRING, String::from("FIX.4.4")));
    message.add(Field::new(*MSG_TYPE, heart_beat.unwrap().to_string()));
    message.add(Field::new(
        *SENDING_TIME,
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    message
}

pub fn order_message(msg_seq: i32, order_id: i64) -> Message {
    // let message = "8=FIX.4.49=12735=D34=1549=BANZAI52=20200614-12:00:13.57156=EXEC
    // 11=159213601352021=138=3440=154=155=A59=060=20200614-12:00:13.57010=105";
    let mut order = Message::new();
    order.add(Field::new(*BEGIN_STRING, String::from("FIX.4.4")));
    order.add(Field::new(*MSG_TYPE, String::from("D")));
    order.add(Field::new(*MSG_SEQ_NUM, String::from(msg_seq.to_string())));
    order.add(Field::new(*SENDER_CMP_ID, String::from("BANZAI")));
    order.add(Field::new(
        *SENDING_TIME,
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    order.add(Field::new(*TARGET_CMP_ID, String::from("EXEC")));
    order.add(Field::new(*CLORDID, String::from(order_id.to_string())));
    order.add(Field::new(*HANDLINST, String::from("1")));
    order.add(Field::new(*ORDER_QTY, String::from("34")));
    order.add(Field::new(*ORDER_TYPE, String::from("1")));
    order.add(Field::new(*SIDE, String::from("1")));
    order.add(Field::new(*SYMBOL, String::from("A")));
    order.add(Field::new(*TIME_IN_FORCE, String::from("0")));
    order.add(Field::new(
        *TRANSACT_TIME,
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    order
}
