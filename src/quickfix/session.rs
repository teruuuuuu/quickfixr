use chrono::Utc;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::{env, str};

use std::collections::HashMap;
use crate::quickfix::message::message_fix44::{hello_message, order_message};
use crate::quickfix::message::message::Message;
use crate::quickfix::message::field_key::FieldKey;
use crate::quickfix::message::message_factory::MessageFactory;
use crate::quickfix::message::field::Field;
use crate::quickfix::message::message_read::{res_read, message_debug};

#[derive(Debug)]
pub struct Session {
    host: String,
    port: String,
    msg_seq: i32,
    hart_beat: i32,
    tcp_stream: TcpStream,
    message_factory: MessageFactory,
}

impl Session {
    pub fn new(host: String, port: String, message_factory: MessageFactory) -> Session {
        let tcp_stream = TcpStream::connect_timeout(
            &format!("{}:{}", host, port).parse().unwrap(),
            Duration::from_secs(1),
        )
        .expect("Could not connect.");
        tcp_stream
            .set_read_timeout(Some(Duration::from_secs(600)))
            .unwrap();
        tcp_stream
            .set_write_timeout(Some(Duration::from_secs(600)))
            .unwrap();

        Session {
            host: host,
            port: port,
            msg_seq: 1,
            hart_beat: 30,
            tcp_stream: tcp_stream,
            message_factory: message_factory,
        }
    }

    pub fn hello(&mut self) -> Message {
        self.req_send(self.message_factory.hello(self.hart_beat))
    }

    pub fn order(&mut self) -> Message {
        self.req_send(self.message_factory.order(Self::next_orderid()) )
    }

    fn next_orderid() -> i64 {
        let utc = Utc::now();
        utc.timestamp_millis()
    }

    fn req_send(&mut self, req: Message) -> Message {
        let req_str = req.to_request_string();
        println!("request: {:?}", req_str);
        message_debug(&req_str);
        println!();
        self.tcp_stream.write(req_str.as_bytes());
        let response = res_read(&self.tcp_stream);
        println!("read: {:?}", response.to_string());
        response.debug();
        response
    }
}

