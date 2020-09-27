use std::io::{BufRead, BufReader, Read};
use std::str;

use crate::quickfix::message::field::Field;
use crate::quickfix::message::field_key::FieldKey;
use crate::quickfix::message::field_key::{BEGIN_STRING, BODY_LENGTH, CHECKSUM};
use crate::quickfix::message::message::Message;
use log::debug;

pub fn res_read<R: Read>(mut reader: BufReader<R>) -> Message {
    let mut vec: Vec<Field> = Vec::new();
    let mut length: i32 = -1;
    let mut current_length: i32 = 0;
    let mut checksum: i32 = 0;

    fn to_field(buf_read: &str) -> Field {
        let str = buf_read.replace(&String::from_utf8(vec![1]).unwrap(), "");

        let key_val: Vec<&str> = str.split('=').collect();
        let key: String = key_val[0].parse().unwrap();
        let value: String = key_val[1].parse().unwrap();
        Field::new(FieldKey::new(key.parse().unwrap()), value)
    }

    loop {
        let mut buffer = Vec::new();
        if reader
            .read_until(b'', &mut buffer)
            .expect("failed to read from the socket")
            == 0
        {
            // disconnect
            // 9=010=167
            return Message::new();
        }

        for b in &buffer {
            checksum += *b as i32;
            checksum %= 256;
        }

        let buf_read = str::from_utf8(&buffer).unwrap();
        debug!("read_tcp_stream: {:?}", buf_read);
        let field = to_field(&buf_read);

        if !field.tag.eq(&*BEGIN_STRING) && !field.tag.eq(&*BODY_LENGTH) {
            current_length += buffer.len() as i32;
        }
        if field.tag.eq(&*BODY_LENGTH) {
            length = field.data.parse::<i32>().unwrap();
        }
        vec.push(field);
        if length > 0 && length <= current_length {
            break;
        }
    }

    {
        let mut buffer = Vec::new();
        if reader
            .read_until(b'', &mut buffer)
            .expect("failed to read from the socket")
            == 0
        {
            // disconnect
            // 9=010=167
            return Message::new();
        }

        let buf_read = str::from_utf8(&buffer).unwrap();
        debug!("read_tcp_stream: {:?}", buf_read);
        let field = to_field(&buf_read);

        if field.tag.eq(&*CHECKSUM) && field.data.parse::<i32>().unwrap() != checksum {
            println!("checksum invalid")
        }
        vec.push(field);
    }

    vec.into_iter().fold(Message::new(), |mut acc, cur| {
        acc.add(cur);
        acc
    })
}

pub fn message_debug(str: &String) {
    let key_vals: Vec<&str> = str.split(&String::from_utf8(vec![1]).unwrap()).collect();
    for field in key_vals.into_iter() {
        let key_val: Vec<&str> = field.split(&String::from("=")).collect();
        if key_val.len() > 1 {
            let key = FieldKey::new(key_val[0].parse::<i32>().unwrap());
            let val = key_val[1];
            println!("{:?}: {:?}", key.to_string(), val);
        }
    }
}

#[test]
fn test_read() {
    use crate::quickfix::message::field_key::{
        BEGIN_STRING, MSG_SEQ_NUM, MSG_TYPE, SENDER_CMP_ID, SENDING_TIME, TARGET_CMP_ID,
    };
    {
        let mut message = Message::new();
        message.add(Field::new(*BEGIN_STRING, String::from("FIX.4.4")));
        message.add(Field::new(*MSG_TYPE, String::from("0")));
        message.add(Field::new(*MSG_SEQ_NUM, String::from("5")));
        message.add(Field::new(*SENDER_CMP_ID, String::from("EXEC")));
        message.add(Field::new(
            *SENDING_TIME,
            String::from("20200926-07:50:55.325"),
        ));
        message.add(Field::new(*TARGET_CMP_ID, String::from("BANZAI")));

        let heart_beat = res_read(BufReader::new(message.to_request_string().as_bytes()));
        println!("{:?}", message.to_request_string());
        println!("{:?}", heart_beat.to_request_string());
        assert_eq!(message, heart_beat);
    }

    {
        let disconnect_bytes = "".as_bytes();
        let message = res_read(BufReader::new(disconnect_bytes));
        assert_eq!(Message::new(), message);
    }
}
