use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::{env, str};

use crate::quickfix::message::field::Field;
use crate::quickfix::message::field_key::FieldKey;
use crate::quickfix::message::message::Message;
use std::collections::HashMap;

pub fn res_read(mut reader: BufReader<&TcpStream>) -> Message {
    let mut vec: Vec<Field> = Vec::new();
    let mut length: i32 = -1;
    let mut current_length: i32 = 0;
    let mut checksum: i32 = 0;

    let mut debug_string = String::new();

    fn to_field(buf_read: &str) -> Field {
        let str = buf_read.replace(&String::from_utf8(vec![1]).unwrap(), "");

        let key_val: Vec<&str> = str.split('=').collect();
        let key: String = key_val[0].parse().unwrap();
        let value: String = key_val[1].parse().unwrap();

        Field::new(FieldKey::new(key.parse().unwrap()), value)
    }

    loop {
        let mut buffer = Vec::new();
        reader
            .read_until(b'', &mut buffer)
            .expect("failed to read from the socket");

        for b in &buffer {
            checksum += *b as i32;
            checksum %= 256;
        }

        let buf_read = str::from_utf8(&buffer).unwrap();
        debug_string = format!("{}{}", debug_string, buf_read);
        let field = to_field(&buf_read);

        if !field.tag.eq(&FieldKey::begin_string()) && !field.tag.eq(&FieldKey::body_length()) {
            current_length += buffer.len() as i32;
        }
        if field.tag.eq(&FieldKey::body_length()) {
            length = field.data.parse::<i32>().unwrap();
        }
        vec.push(field);
        // ret.insert(field.0, field.1);
        if length > 0 && length <= current_length {
            break;
        }
    }

    {
        let mut buffer = Vec::new();
        reader
            .read_until(b'', &mut buffer)
            .expect("failed to read from the socket");

        let buf_read = str::from_utf8(&buffer).unwrap();
        debug_string = format!("{}{}", debug_string, buf_read);
        let field = to_field(&buf_read);

        if field.tag.eq(&FieldKey::checksum()) && field.data.parse::<i32>().unwrap() != checksum {
            println!("checksum invalid")
        }
        vec.push(field);
    }

    let mut ret = Message::new();
    for field in vec.into_iter() {
        ret.add(field);
    }

    // println!("response: {:?}", debug_string);
    ret
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
