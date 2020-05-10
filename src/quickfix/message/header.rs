use std::collections::HashMap;

use super::field::Field;
use super::field_key::FieldKey;

#[derive(Debug)]
pub struct Header {
    pub fields: HashMap<FieldKey, Field>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            fields: HashMap::new(),
        }
    }

    pub fn add(&mut self, field: Field) -> Option<Field> {
        self.fields.insert(field.getTag(), field)
    }

    pub fn get_fields(&self) -> Vec<Field> {
        let a = Vec::new();
        for i in self.fields.iter() {
            println!("{:?}", i);
            // a.append(i.1);
        }
        a
    }

    pub fn to_string(&self) -> String {
        format!("")
    }
}

#[test]
fn test_header() {
    let mut a = Header::new();
    a.add(Field::new(FieldKey::msg_type(), String::from("A")));
    println!("{:?}", a);
}
