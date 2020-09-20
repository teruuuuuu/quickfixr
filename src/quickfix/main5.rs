use std::str;
use std::thread;

use crate::quickfix::message::message_fix44::logon_message;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn main() {
    let tcp_stream_write = TcpStream::connect_timeout(
        &format!("{}:{}", "127.0.0.1", "9880").parse().unwrap(),
        Duration::from_secs(1),
    )
    .expect("Could not connect.");

    let tcp_stream_read = tcp_stream_write.try_clone().unwrap();
    for handle in vec![send(tcp_stream_write), read(tcp_stream_read)] {
        handle.join().unwrap();
    }
}

fn read(tcp_stream: TcpStream) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut tcp_reader = BufReader::new(tcp_stream);
        loop {
            let mut buffer = Vec::new();
            loop {
                tcp_reader
                    .read_until(b'', &mut buffer)
                    .expect("failed to read from the socket");

                let buf_read = str::from_utf8(&buffer).unwrap();
                println!("read: {}", buf_read);
                buffer.clear();
            }
        }
    })
}

fn send(mut tcp_stream: TcpStream) -> JoinHandle<()> {
    thread::spawn(move || loop {
        println!("input method: L(Logon)");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let input = s.trim();
        if input.eq("L") {
            let message = logon_message(5, 10);
            println!("send: {}", message.to_string());
            tcp_stream.write(message.to_request_string().as_bytes());
        }
    })
}
