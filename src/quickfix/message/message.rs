use super::field::Field;
use crate::quickfix::message::field_key::FieldKey;
use crate::quickfix::message::field_key::{
    BODY_LENGTH, BODY_LENGTH_KEY, CHECKSUM, CHECKSUM_KEY, MSG_TYPE, MSG_TYPE_KEY, SENDER_CMP_ID,
    TARGET_CMP_ID,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    pub fields: HashMap<FieldKey, Field>,
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.to_request_string().eq(&other.to_request_string())
    }
}
impl Eq for Message {}

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
            println!("{:?}:{:?}", field.tag.to_string(), field.data);
        }
    }

    pub fn to_request_string(&self) -> String {
        let mut result = String::from("");
        let mut msgtype = String::from("");
        let mut result2 = String::from("");
        for field in self.get_fields() {
            match field.tag.key_val() {
                BEGIN_STRING_KEY => result.push_str(&field.to_string()),
                MSG_TYPE_KEY => msgtype.push_str(&field.to_string()),
                CHECKSUM_KEY | BODY_LENGTH_KEY => {}
                _ => result2.push_str(&field.to_string()),
            };
        }
        result.push_str(
            &Field::new(
                *BODY_LENGTH,
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
        let checksum = Field::new(*CHECKSUM, format!("{:03}", check));
        result.push_str(&checksum.to_string());
        result
    }

    pub fn to_debug_string(&self) -> String {
        let sender = match self.fields.get(&*SENDER_CMP_ID) {
            Option::Some(v) => v.data.to_string(),
            _ => "".to_string(),
        };
        let target = match self.fields.get(&*TARGET_CMP_ID) {
            Option::Some(v) => v.data.to_string(),
            _ => "".to_string(),
        };
        format!("{}=>{}: {}", sender, target, self.to_request_string())
    }

    pub fn get_msg_type(&self) -> Option<&Field> {
        self.fields.get(&*MSG_TYPE)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::from("");
        for field in self.get_fields() {
            result.push_str(&field.to_string());
        }
        result
    }

    pub fn get(&self, field_key: FieldKey) -> Option<&Field> {
        self.fields.get(&field_key)
    }
}

#[test]
fn test_message() {
    let mut a = Message::new();
    let field9 = Field::new(*MSG_TYPE, String::from("A"));
    let field8 = Field::new(*BODY_LENGTH, field9.clone().to_string().len().to_string());
    a.add(field8);
    a.add(field9);
    println!("{:?}", a.get_fields());
    println!("{:?}", a.to_string());
}
