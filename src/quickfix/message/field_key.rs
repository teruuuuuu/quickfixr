// use std::cmp::Eq;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldKey(i32);

impl FieldKey {
    pub fn new(key: i32) -> FieldKey {
        FieldKey(key)
    }
    pub fn begin_string() -> FieldKey {
        FieldKey(8)
    }

    pub fn msg_type() -> FieldKey {
        FieldKey(35)
    }

    pub fn key_val(&self) -> i32 {
        self.0
    }

    pub fn to_string(&self) -> String {
        match self {
            FieldKey(8) => String::from("BeginString"),
            FieldKey(35) => String::from("MsgType"),
            _ => String::from(""),
        }
    }
}

// impl Eq for FieldKey {}

#[test]
fn test_field_key() {
    println!("{:?}", FieldKey::begin_string().to_string());
    println!("{:?}", FieldKey::msg_type().to_string());
}
