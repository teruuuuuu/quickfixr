use chrono::Utc;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::{env, str};

use std::collections::HashMap;
use crate::quickfix::message::message_fix44::{logon_message, order_message};
use crate::quickfix::message::message::Message;
use crate::quickfix::message::field_key::FieldKey;
use crate::quickfix::message::message_factory::MessageFactory;
use crate::quickfix::message::field::Field;
use crate::quickfix::message::message_read::{res_read, message_debug};
use std::sync::{mpsc, Mutex};
use async_std::sync::Arc;
use std::sync::mpsc::Sender;


#[derive(Debug, Clone)]
struct ChanelMessage { method: String, from: String }

impl ChanelMessage {
    pub fn new(method: String, from: String) -> Self {
        ChanelMessage { method, from }
    }
}


// macro_rules! send {
//     ($tx:tt, $chanelMessage:ident) => {
//     let mut count = 0;
//     loop {
//         let result = $tx.send($chanelMessage.clone());
//         match result {
//           Err (_) => {
//             println!("send error:");
//             count += 1;
//             if count > 10 {
//               println!("send error over 10 times");
//               break;
//             }
//             thread::sleep(Duration::from_millis(50));
//           }
//           Ok(_) => {
//             break;
//           }
//         }
//       }
//     }
// }


#[derive(Debug)]
pub struct Session {
    host: String,
    port: String,
    msg_seq: i32,
    hart_beat: i32,
    tcp_stream: Mutex<Arc<TcpStream>>,
    message_factory: MessageFactory,
}

impl Session {
    pub fn new(host: String, port: String, message_factory: MessageFactory) -> Session {
        let tcp_stream = Arc::new(TcpStream::connect_timeout(
            &format!("{}:{}", host, port).parse().unwrap(),
            Duration::from_secs(1),
        )
        .expect("Could not connect."));

        tcp_stream
            .set_read_timeout(Some(Duration::from_secs(600)))
            .unwrap();
        tcp_stream
            .set_write_timeout(Some(Duration::from_secs(600)))
            .unwrap();

        // let mut tcp_reader = BufReader::new(&tcp_stream);
        let session = Session {
            host: host,
            port: port,
            msg_seq: 1,
            hart_beat: 30,
            tcp_stream: Mutex::new(tcp_stream),
            message_factory: message_factory,
        };
        // println!("call read init");
        // session.read_init();
        // println!("call read end");
        session
    }

    // fn get_tcp_reader(&mut self) -> Box<BufReader<&'a TcpStream>> {
    //     let mut tcp_stream = *self.tcp_stream;
    //
    //     Box::new(BufReader::new(&tcp_stream))
    //
    // }
    pub fn start(&mut self) {
        crossbeam::scope(|scope| {
            let (tx, rx) = mpsc::channel();
            // let txc = mpsc::Sender::clone(&tx);
            // let mut tcp_reader = self.tcp_reader();
            // let mut tcp_reader = BufReader::new(&self.tcp_stream);

            // let tcp_stream = TcpStream::connect_timeout(
            //     &format!("{}:{}", self.host, self.port).parse().unwrap(),
            //     Duration::from_secs(1),
            // )
            //     .expect("Could not connect.");
            // tcp_stream
            //     .set_read_timeout(Some(Duration::from_secs(600)))
            //     .unwrap();
            // tcp_stream
            //     .set_write_timeout(Some(Duration::from_secs(600)))
            //     .unwrap();
            // let mut tcp_reader = BufReader::new(&tcp_stream);

            let mut tcp_stream = TcpStream::connect_timeout(
                &format!("{}:{}", self.host, self.port).parse().unwrap(),
                Duration::from_secs(1),
            )
                .expect("Could not connect.");

            tcp_stream
                .set_read_timeout(Some(Duration::from_secs(600)))
                .unwrap();
            tcp_stream
                .set_write_timeout(Some(Duration::from_secs(600)))
                .unwrap();

            let mut tcp_reader = BufReader::new(&tcp_stream);

            // self.read_init(scope, tx);
            // self.start_random(scope, tx);
            // self.start_input(scope, txc);

            for message in rx {
                self.receive_message(message);
            }
        });
    }

    fn scope_thread<'a, 'b>(&'a mut self, scope: &'a crossbeam::thread::Scope<'a>) {


        scope.spawn(move |s| {
            let (tx, rx) = mpsc::channel();
            // let txc = mpsc::Sender::clone(&tx);


            s.spawn(move |t| {
                let tcp_stream = self.tcp_stream.get_mut().unwrap();

                let mut tcp_reader = BufReader::new(tcp_stream.as_ref());
                t.spawn(move |u| {

                });

                t.spawn(move |u| {
                    for message in rx {
                        self.receive_message(message);
                    }
                });
            });




        });
    }
    fn test1(&mut self, scope: &crossbeam::thread::Scope<'_>,
             tx: std::sync::mpsc::Sender<ChanelMessage>,
             tcp_reader: BufReader<&TcpStream>)
    {
        scope.spawn(move |s| {
            // tcp_reade
        });
    }

    fn test2(&mut self, scope: &crossbeam::thread::Scope<'_>, tx: std::sync::mpsc::Sender<ChanelMessage>) {
        scope.spawn(move |s| {
        });
    }

    fn receive_message(&mut self, message: ChanelMessage) {
        println!("receive message: {:?}", message);
    }

    // fn tcp_reader(&mut self) -> BufReader<&'a TcpStream> { BufReader::new(&self.tcp_stream) }

    fn read_init(&mut self, scope: &crossbeam::thread::Scope<'_>,
                 tx: std::sync::mpsc::Sender<ChanelMessage>,
                 mut tcp_reader: &BufReader<&TcpStream>
    ) -> () {
        // let mut tcp_reader = self.get_tcp_reader();

        // let mut tcp_stream = *self.tcp_stream;
        // let mut tcp_reader = BufReader::new(tcp_stream);

        // let mut tcp_reader = BufReader::new(&self.tcp_stream);

        // let mut tcp_reader = BufReader::new(&self.tcp_stream);

        // let mut tcp_stream = TcpStream::connect_timeout(
        //     &format!("{}:{}", self.host, self.port).parse().unwrap(),
        //     Duration::from_secs(1),
        // )
        //     .expect("Could not connect.");
        //
        // tcp_stream
        //     .set_read_timeout(Some(Duration::from_secs(600)))
        //     .unwrap();
        // tcp_stream
        //     .set_write_timeout(Some(Duration::from_secs(600)))
        //     .unwrap();

        // let mut tcp_reader = BufReader::new(&*self.tcp_stream);

        // scope.spawn(move |s| {
        //     println!("read init start");
        //     // let mut tcp_reader = self.tcp_reader();
        //
        //
        //
        //     loop {
        //         // let mut reader = BufReader::new(&self.tcp_stream);
        //         // let response = res_read(tcp_reader);
        //         let mut vec: Vec<Field> = Vec::new();
        //         let mut length: i32 = -1;
        //         let mut current_length: i32 = 0;
        //         let mut checksum: i32 = 0;
        //
        //         let mut debug_string = String::new();
        //
        //
        //
        //         fn to_field(buf_read: &str) -> Field {
        //             let str = buf_read
        //                 .replace(&String::from_utf8(vec![1]).unwrap(), "");
        //
        //             let key_val: Vec<&str> = str.split('=').collect();
        //             let key: String = key_val[0].parse().unwrap();
        //             let value: String = key_val[1].parse().unwrap();
        //
        //             Field::new(FieldKey::new(key.parse().unwrap()), value)
        //         }
        //
        //         loop {
        //             let mut buffer = Vec::new();
        //             tcp_reader
        //                 .read_until(b'', &mut buffer)
        //                 .expect("failed to read from the socket");
        //
        //             for b in &buffer {
        //                 checksum += *b as i32;
        //                 checksum %= 256;
        //             }
        //
        //             let buf_read = str::from_utf8(&buffer).unwrap();
        //             debug_string = format!("{}{}", debug_string, buf_read);
        //             let field = to_field(&buf_read);
        //
        //             if !field.tag.eq(&FieldKey::begin_string())
        //                 && !field.tag.eq(&FieldKey::body_length()) {
        //                 current_length += buffer.len() as i32;
        //             }
        //             if field.tag.eq(&FieldKey::body_length()) {
        //                 length = field.data.parse::<i32>().unwrap();
        //             }
        //             vec.push(field);
        //             // ret.insert(field.0, field.1);
        //             if length > 0 && length <= current_length {
        //                 break;
        //             }
        //         }
        //
        //         {
        //             let mut buffer = Vec::new();
        //             tcp_reader
        //                 .read_until(b'', &mut buffer)
        //                 .expect("failed to read from the socket");
        //
        //             let buf_read = str::from_utf8(&buffer).unwrap();
        //             debug_string = format!("{}{}", debug_string, buf_read);
        //             let field = to_field(&buf_read);
        //
        //
        //             if field.tag.eq(&FieldKey::checksum())
        //                 && field.data.parse::<i32>().unwrap() != checksum {
        //                 println!("checksum invalid")
        //             }
        //             vec.push(field);
        //         }
        //
        //         let mut ret = Message::new();
        //         for field in vec.into_iter() {
        //             ret.add(field);
        //         }
        //
        //         println!("response: {:?}", debug_string);
        //         let response = ret;
        //         println!("read: {:?}", response.to_string());
        //         response.debug();
        //
        //         let chanelMessage = ChanelMessage::new(response.to_string(), String::from("response"));
        //         // send!(tx, message);
        //         tx.send(chanelMessage);
        //     }
        // });
    }

    pub fn logon(&mut self) {
        self.req_send(self.message_factory.logon(self.msg_seq, self.hart_beat));
    }

    pub fn order(&mut self) {
        self.req_send(self.message_factory.order(self.msg_seq, Self::next_orderid()) );
    }

    fn next_orderid() -> i64 {
        let utc = Utc::now();
        utc.timestamp_millis()
    }

    fn req_send(&mut self, req: Message) {
        let req_str = req.to_request_string();
        println!("request: {:?}", req_str);
        message_debug(&req_str);
        println!();
        // self.tcp_stream.write(req_str.as_bytes());
        // let response = res_read(&self.tcp_stream);
        // println!("read: {:?}", response.to_string());
        // response.debug();
        // response
    }
}

