use super::field::Field;
use std::collections::HashMap;
use crate::quickfix::message::field_key::FieldKey;

#[derive(Debug)]
pub struct Message {
    pub fields: HashMap<FieldKey, Field>,
}

impl Message {
    pub fn new() -> Message {
        Message {
            fields: HashMap::new(),
        }
    }

    pub fn add(&mut self, field: Field) -> Option<Field> {
        self.fields.insert(field.get_tag(), field)
    }

    pub fn get_fields(&self) -> Vec<Field> {
        let mut vec: Vec<Field> = self.fields.iter().map(|a| a.1.clone()).collect();
        vec.sort_by(|a, b| a.get_tag().key_val().cmp(&b.get_tag().key_val()));
        vec
    }

    pub fn debug(&self) {
        for field in self.get_fields() {
            println!("{:?}: {:?}", field.tag.to_string(), field.data);
        }
    }

    pub fn to_request_string(&self) -> String {
        let mut result = String::from("");
        let mut msgtype = String::from("");
        let mut result2 = String::from("");
        for field in self.get_fields() {
            if field.tag == FieldKey::begin_string() {
                result.push_str(&field.to_string());
            } else if field.tag == FieldKey::msg_type() {
                msgtype.push_str(&field.to_string());
            } else if field.tag == FieldKey::checksum() {
            } else {
                result2.push_str(&field.to_string());
            }
        }
        result.push_str(
            &Field::new(
                FieldKey::body_length(),
                ((result2.len() + msgtype.len()) as i32).to_string(),
            )
                .to_string(),
        );
        result.push_str(&msgtype);
        result.push_str(&result2);
        let mut check: i32 = 0;
        for b in result.as_bytes() {
            check += *b as i32;
            check %= 256;
        }
        let checksum = Field::new(FieldKey::checksum(), format!("{:03}", check));
        result.push_str(&checksum.to_string());
        result
    }

    pub fn to_string(&self) -> String {
        let mut result = String::from("");
        for field in self.get_fields() {
            result.push_str(&field.to_string());
        }
        result
    }
}

#[test]
fn test_message() {
    let mut a = Message::new();
    let field9 = Field::new(FieldKey::msg_type(), String::from("A"));
    let field8 = Field::new(
        FieldKey::body_length(),
        field9.clone().to_string().len().to_string(),
    );
    a.add(field8);
    a.add(field9);
    println!("{:?}", a.get_fields());
    println!("{:?}", a.to_string());
}

