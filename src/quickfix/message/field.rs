use super::field_key::FieldKey;

#[derive(Debug)]
pub struct Field {
    tag: FieldKey,
    data: String,
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

    pub fn getTag(&self) -> FieldKey {
        self.tag.clone()
    }

    pub fn getLength(&self) -> i32 {
        self.to_string().len() as i32
    }
}

#[test]
fn test_message() {
    let field1 = Field::new(FieldKey::msg_type(), String::from("A"));
    println!("{:?}", field1.to_string());
    assert_eq!(5, field1.getLength());
}
