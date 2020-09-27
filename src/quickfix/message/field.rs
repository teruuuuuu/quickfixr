use super::field_key::FieldKey;

#[derive(Debug, Clone)]
pub struct Field {
    pub tag: FieldKey,
    pub data: String,
}

impl Field {
    pub fn new(tag: FieldKey, data: String) -> Self {
        Self {
            tag: tag,
            data: data,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}={}{}", self.tag.key_val(), self.data, char::from(1))
    }

    pub fn get_tag(&self) -> FieldKey {
        self.tag.clone()
    }

    pub fn length(&self) -> i32 {
        self.to_string().len() as i32
    }
}

#[test]
fn test_message() {
    use crate::quickfix::message::field_key::MSG_TYPE;

    let field1 = Field::new(*MSG_TYPE, String::from("A"));
    println!("{:?}", field1.to_string());
    assert_eq!(5, field1.length());
}
