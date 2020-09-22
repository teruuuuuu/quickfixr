use chrono::Utc;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::{env, str};

use crate::quickfix::message::field::Field;
use crate::quickfix::message::field_key::FieldKey;
use crate::quickfix::message::message::Message;
use crate::quickfix::message::message_factory::MessageFactory;
use crate::quickfix::message::msg_type::heart_beat;

pub fn create_factory44() -> MessageFactory {
    MessageFactory::new(logon_message, order_message)
}

pub fn logon_message(hart_beat: i32) -> Message {
    // let message = "8=FIX.4.49=6535=A34=149=BANZAI52=20200506-12:10:57.99856=EXEC98=0108=30141=Y10=228";
    let mut logon = Message::new();
    logon.add(Field::new(
        FieldKey::begin_string(),
        String::from("FIX.4.4"),
    ));
    logon.add(Field::new(FieldKey::msg_type(), String::from("A")));
    logon.add(Field::new(FieldKey::msg_seq_num(), String::from("1")));
    // logon.add(Field::new(
    //     FieldKey::sender_cmp_id(),
    //     String::from("BANZAI"),
    // ));
    logon.add(Field::new(
        FieldKey::sending_time(),
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    // logon.add(Field::new(FieldKey::target_cmp_id(), String::from("EXEC")));
    logon.add(Field::new(FieldKey::encrypt_method(), String::from("0")));
    logon.add(Field::new(
        FieldKey::heart_beat_interval(),
        hart_beat.to_string(),
    ));
    logon.add(Field::new(
        FieldKey::reset_seq_num_flag(),
        String::from("Y"),
    ));
    logon
}

pub fn heart_beat_message() -> Message {
    let mut message = Message::new();
    message.add(Field::new(
        FieldKey::begin_string(),
        String::from("FIX.4.4"),
    ));
    message.add(Field::new(
        FieldKey::msg_type(),
        heart_beat.unwrap().to_string(),
    ));
    message.add(Field::new(
        FieldKey::sending_time(),
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    message
}

pub fn order_message(msg_seq: i32, order_id: i64) -> Message {
    // let message = "8=FIX.4.49=12735=D34=1549=BANZAI52=20200614-12:00:13.57156=EXEC
    // 11=159213601352021=138=3440=154=155=A59=060=20200614-12:00:13.57010=105";
    let mut order = Message::new();
    order.add(Field::new(
        FieldKey::begin_string(),
        String::from("FIX.4.4"),
    ));
    order.add(Field::new(FieldKey::msg_type(), String::from("D")));
    order.add(Field::new(
        FieldKey::msg_seq_num(),
        String::from(msg_seq.to_string()),
    ));
    order.add(Field::new(
        FieldKey::sender_cmp_id(),
        String::from("BANZAI"),
    ));
    order.add(Field::new(
        FieldKey::sending_time(),
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    order.add(Field::new(FieldKey::target_cmp_id(), String::from("EXEC")));
    order.add(Field::new(
        FieldKey::cl_ord_id(),
        String::from(order_id.to_string()),
    ));
    order.add(Field::new(FieldKey::handl_inst(), String::from("1")));
    order.add(Field::new(FieldKey::order_qty(), String::from("34")));
    order.add(Field::new(FieldKey::order_type(), String::from("1")));
    order.add(Field::new(FieldKey::side(), String::from("1")));
    order.add(Field::new(FieldKey::symbol(), String::from("A")));
    order.add(Field::new(FieldKey::time_in_force(), String::from("0")));
    order.add(Field::new(
        FieldKey::transact_time(),
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    order
}
