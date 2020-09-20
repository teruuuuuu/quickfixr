use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::quickfix::message::message_fix44::create_factory44;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread::JoinHandle;
use std::time::Duration;

pub fn main() {
    let mut tcp_stream_write = TcpStream::connect_timeout(
        &format!("{}:{}", "127.0.0.1", "9880").parse().unwrap(),
        Duration::from_secs(1),
    )
    .expect("Could not connect.");
    tcp_stream_write
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    tcp_stream_write
        .set_write_timeout(Some(Duration::from_secs(20)))
        .unwrap();
    let tcp_stream_arc = Arc::new(Mutex::new(tcp_stream_write));

    for handle in vec![
        send(Arc::clone(&tcp_stream_arc)),
        read(Arc::clone(&tcp_stream_arc)),
    ] {
        handle.join().unwrap();
    }
}

fn read(tcp_stream_arc: Arc<Mutex<TcpStream>>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let mut tcp_stream = tcp_stream_arc.lock().unwrap();
            let mut tcp_reader = BufReader::new(&*tcp_stream);

            println!("read start");
            // let mut buffer = Vec::new();
            let mut buffer = [0; 100];

            let result = tcp_reader.read(&mut buffer);
            println!("read result: {:?}", result);
            if result.is_ok() {
                let buf_read = std::str::from_utf8(&buffer).unwrap();
                println!("receved: {:?}", buf_read);
            }
            println!("read end");

            std::mem::drop(tcp_reader);
            std::mem::drop(tcp_stream);
            thread::sleep(Duration::from_secs(1));
        }
    })
}

fn send(tcp_stream_arc: Arc<Mutex<TcpStream>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        println!("send start");
        let mut tcp_stream = tcp_stream_arc.lock().unwrap();
        let message = create_factory44().logon(1, 30).to_request_string();
        println!("message: {:?}", message);
        tcp_stream.write(message.as_bytes());
        println!("send end");
        std::mem::drop(tcp_stream);
        thread::sleep(Duration::from_secs(1));
    })
}
