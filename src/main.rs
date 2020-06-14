use chrono::{Date, DateTime, Datelike, Local, TimeZone, Utc};
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::{env, str};

use quickfixr::quickfix::message::field::Field;
use quickfixr::quickfix::message::field_key::FieldKey;
use quickfixr::quickfix::message::message::Message;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     eprintln!("Please specify address and port. Usage: ./main [address]:[port]");
    //     std::process::exit(1);
    // }
    // let remote = args[1].parse().expect("Usage: ./main [address]:[port]");
    let remote = "127.0.0.1:9880".parse().unwrap();

    let mut stream =
        TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect.");
    stream
        .set_read_timeout(Some(Duration::from_secs(600)))
        .unwrap();
    stream
        .set_write_timeout(Some(Duration::from_secs(600)))
        .unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // stream.write(input.as_bytes()).expect("failed to write");
        println!("{:?}", input);
        println!("{:?}", input.as_bytes());
        println!("{:?}", helloMessage().to_string());
        println!("{:?}", helloMessage().to_string().as_bytes());

        stream.write(helloMessage().to_string().as_bytes());
        // stream.write(&[13, 10]);
        // stream.write(&[
        //     56, 61, 70, 73, 88, 46, 52, 46, 52, 1, 57, 61, 54, 53, 1, 51, 53, 61, 65, 1, 51, 52,
        //     61, 49, 1, 52, 57, 61, 66, 65, 78, 90, 65, 73, 1, 53, 50, 61, 50, 48, 50, 48, 48, 53,
        //     48, 54, 45, 49, 50, 58, 49, 48, 58, 53, 55, 46, 57, 57, 56, 1, 53, 54, 61, 69, 88, 69,
        //     67, 1, 57, 56, 61, 48, 1, 49, 48, 56, 61, 51, 48, 1, 49, 48, 61, 50, 50, 56, 1,
        // ]);

        // let message = "8=FIX.4.49=6535=A34=149=BANZAI52=20200506-12:10:57.99856=EXEC98=0108=3010=228";
        // let message = "8=FIX.4.49=6535=A34=349=BANZAI52=20200506-12:11:21.99856=EXEC98=0108=3010=222";
        // let message = "8=FIX.4.49=6535=A34=249=BANZAI52=20200506-12:11:09.99956=EXEC98=0108=3010=228";
        // let message = "8=FIX.4.49=6635=A34=5149=BANZAI52=20200505-12:17:45.14456=EXEC98=0108=3010=012";
        // let message = "8=FIX.4.49=6335=234=2749=BANZAI52=20200506-12:14:18.08356=EXEC7=116=010=095";
        // let message = "8=FIX.4.49=5435=034=2849=BANZAI52=20200506-12:14:48.98856=EXEC10=244";
        // let message = "8=FIX.4.49=6335=134=3049=BANZAI52=20200506-12:15:48.98856=EXEC112=TEST10=001";
        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        println!("wait");
        reader
            // .read(buf: &mut [u8])
            .read_until(b'', &mut buffer)
            .expect("failed to read from the socket");
        println!("read");
        println!("result: {:?}", &buffer);
        println!("result: {:?}", str::from_utf8(&buffer).unwrap());
    }
}

fn helloMessage() -> Message {
    let mut hello = Message::new(String::from("FIX.4.4"));
    hello.add_header(Field::new(
        FieldKey::begin_string(),
        String::from("FIX.4.4"),
    ));
    hello.add_header(Field::new(FieldKey::msg_type(), String::from("A")));
    hello.add_header(Field::new(FieldKey::msg_seq_num(), String::from("7")));
    hello.add_header(Field::new(
        FieldKey::sender_cmp_id(),
        String::from("BANZAI"),
    ));
    hello.add_header(Field::new(
        FieldKey::sending_time(),
        Utc::now().format("%Y%m%d-%H:%M:%S").to_string(),
    ));
    hello.add_header(Field::new(FieldKey::target_cmp_id(), String::from("EXEC")));
    hello.add_header(Field::new(FieldKey::encrypt_method(), String::from("0")));
    hello.add_header(Field::new(
        FieldKey::heard_beat_interval(),
        String::from("30"),
    ));
    hello.add_header(Field::new(FieldKey::checksum(), String::from("222")));
    hello
}

#[test]
fn a() {
    println!("{:?}", helloMessage().to_string());
}
