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
    pub fn body_length() -> FieldKey {
        FieldKey(9)
    }
    pub fn msg_seq_num() -> FieldKey {
        FieldKey(34)
    }
    pub fn msg_type() -> FieldKey {
        FieldKey(35)
    }
    pub fn sender_cmp_id() -> FieldKey {
        FieldKey(49)
    }
    pub fn sending_time() -> FieldKey {
        FieldKey(52)
    }
    pub fn target_cmp_id() -> FieldKey {
        FieldKey(56)
    }
    pub fn encrypt_method() -> FieldKey {
        FieldKey(98)
    }
    pub fn heard_beat_interval() -> FieldKey {
        FieldKey(108)
    }
    pub fn checksum() -> FieldKey {
        FieldKey(10)
    }

    pub fn key_val(&self) -> i32 {
        self.0
    }

    pub fn to_string(&self) -> String {
        match self {
            FieldKey(8) => String::from("BeginString"),
            FieldKey(9) => String::from("BodyLength"),
            FieldKey(35) => String::from("MsgType"),
            _ => String::from(""),
        }
    }
}

// impl Eq for FieldKey {}

#[test]
fn test_field_key() {
    assert_eq!(8, FieldKey::begin_string().key_val());
    assert_eq!(9, FieldKey::body_length().key_val());
    assert_eq!(35, FieldKey::msg_type().key_val());
}
