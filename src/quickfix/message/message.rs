use super::field::Field;
use super::header::Header;
use super::message_factory::message_factory;

#[derive(Debug)]
pub struct Message {
    pub version: String,
    pub header: Header,
}

impl Message {
    pub fn new(version: String) -> Message {
        Message {
            version: version,
            header: Header::new(),
        }
    }

    pub fn add_header(&mut self, field: Field) {
        self.header.add(field);
    }

    pub fn to_string(&self) -> String {
        // for i in self.header.fields {
        //     println!("{:?}", i);
        // }
        self.header.to_string()
    }
}
