use std::str;
use std::thread;

use crate::quickfix::message::message::Message;
use async_std::sync::Arc;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

pub trait OnMessage {
    fn receive(&self, message: Message);
}

pub struct Session {
    host: String,
    port: String,
    tcp_stream: Option<Arc<Mutex<TcpStream>>>,
}

// impl Copy for Session {
//     fn copy(&mut self) -> Self {
//         Self::new(self.host, self.port, Op)
//     }
// }

impl Session {
    pub fn new(host: String, port: String) -> Self {
        Self {
            host: host,
            port: port,
            tcp_stream: Option::None,
        }
    }

    pub fn start(&mut self, on_message: Box<dyn OnMessage + Send>) -> JoinHandle<()> {
        let mut tcp_stream = TcpStream::connect_timeout(
            &format!("{}:{}", self.host, self.port).parse().unwrap(),
            Duration::from_secs(1),
        )
        .expect("Could not connect.");

        self.tcp_stream = Option::from(Arc::new(Mutex::new(tcp_stream.try_clone().unwrap())));
        self.read(tcp_stream.try_clone().unwrap(), on_message)
    }

    pub fn send(&mut self, message: Message) -> Result<usize, String> {
        Result::Ok(
            self.tcp_stream
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .write(message.to_request_string().as_bytes())
                .unwrap(),
        )
    }

    fn read(
        &mut self,
        tcp_stream_read: TcpStream,
        on_message: Box<dyn OnMessage + Send>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut tcp_reader = BufReader::new(tcp_stream_read);
            loop {
                let mut buffer = Vec::new();
                loop {
                    tcp_reader
                        .read_until(b'', &mut buffer)
                        .expect("failed to read from the socket");
                    let buf_read = str::from_utf8(&buffer).unwrap();
                    println!("read: {}", buf_read);
                    buffer.clear();
                    let message = Message::new();

                    on_message.receive(message);
                }
            }
        })
    }
}
