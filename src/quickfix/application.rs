use crate::quickfix::message::message::Message;
use crate::quickfix::session::{OnMessage, Session};

use crate::quickfix::message::field::Field;
use crate::quickfix::message::field_key::FieldKey;
use crate::quickfix::message::message_fix44::{heart_beat_message, logon_message};
use crate::quickfix::message::message_read;
use async_std::sync::Arc;
use env_logger as logger;
use log::{debug, error, info, warn};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
pub struct Controller {
    host: String,
    port: String,
    is_init_app: bool,
}

impl Controller {
    pub fn new(host: String, port: String) -> Self {
        Self {
            host,
            port,
            is_init_app: false,
        }
    }

    fn init_app(&mut self) -> Arc<Mutex<Application>> {
        info!("init stream start");
        let mut tcp_stream = TcpStream::connect_timeout(
            &format!("{}:{}", self.host, self.port).parse().unwrap(),
            Duration::from_secs(1),
        )
        .expect("Could not connect.");
        info!("init stream end");

        info!("init app start");
        let heart_beat_interval = 10;
        let mut app = Arc::new(Mutex::new(Application::new(
            tcp_stream,
            "BANZAI".to_string(),
            "EXEC".to_string(),
            heart_beat_interval,
        )));
        self.is_init_app = true;
        info!("init app end");
        app
    }

    pub fn start(&mut self, f: fn(Sender<Message>)) {
        info!("start");
        let mut app = self.init_app();
        let mut tcp_stream = app.lock().unwrap().stream_clone();
        let (tx, rx) = mpsc::channel::<Message>();
        let mut read_handler = self.read(tcp_stream, Arc::clone(&app));
        let mut send_handler_tx = self.send_tx(tx.clone(), f);
        let mut send_handler_rx = self.send_rx(Arc::clone(&app), rx);
        let mut heart_beat_handler =
            self.send_heart_beat(app.lock().unwrap().heart_beat as u64, tx.clone());

        app.lock().unwrap().start();
        for handle in vec![
            read_handler,
            send_handler_tx,
            send_handler_rx,
            heart_beat_handler,
        ] {
            handle.join().unwrap();
        }
    }

    fn read(
        &mut self,
        tcp_stream: TcpStream,
        application: Arc<Mutex<Application>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut tcp_reader = BufReader::new(&tcp_stream);
            loop {
                let message = message_read::res_read(BufReader::new(&tcp_stream));
                application.lock().unwrap().receive(message);
            }
        })
    }

    fn send_tx(&mut self, tx: Sender<Message>, f: fn(Sender<Message>)) -> JoinHandle<()> {
        thread::spawn(move || f(tx))
    }

    fn send_rx(
        &mut self,
        application: Arc<Mutex<Application>>,
        rx: Receiver<Message>,
    ) -> JoinHandle<()> {
        thread::spawn(move || loop {
            let message = rx.recv().unwrap();
            application.lock().unwrap().send(message);
        })
    }

    fn send_heart_beat(&mut self, heart_beat_interval: u64, tx: Sender<Message>) -> JoinHandle<()> {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(heart_beat_interval));
            tx.send(heart_beat_message());
        })
    }
}

struct Application {
    tcp_stream: TcpStream,
    sender_comp_id: String,
    target_comp_id: String,
    heart_beat: i32,
    seq_num_sender: i32,
    seq_num_target: i32,
}

impl Application {
    fn new(
        tcp_stream: TcpStream,
        sender_comp_id: String,
        target_comp_id: String,
        heart_beat: i32,
    ) -> Self {
        Self {
            tcp_stream,
            sender_comp_id,
            target_comp_id,
            heart_beat,
            seq_num_sender: 0,
            seq_num_target: 0,
        }
    }

    fn stream_clone(&mut self) -> TcpStream {
        self.tcp_stream.try_clone().unwrap()
    }

    fn start(&mut self) {
        let message = logon_message(self.heart_beat);
        self.send(message);
    }

    fn receive(&mut self, message: Message) {
        info!("{:?}", message.to_debug_string());
        let seq_num = message.get(FieldKey::msg_seq_num()).unwrap();
        self.seq_num_target += 1;
        if !seq_num.data.eq(&self.seq_num_target.to_string()) {
            debug!("target seq num is wrog");
        }
    }

    fn send(&mut self, message: Message) {
        let message = self.to_send_message(message);
        info!("{:?}", message.to_debug_string());
        self.tcp_stream
            .try_clone()
            .unwrap()
            .write(message.to_request_string().as_bytes());
    }

    fn to_send_message(&mut self, mut message: Message) -> Message {
        self.seq_num_sender = self.seq_num_sender + 1;
        message.add(Field::new(
            FieldKey::msg_seq_num(),
            self.seq_num_sender.to_string(),
        ));

        message.add(Field::new(
            FieldKey::sender_cmp_id(),
            self.sender_comp_id.to_string(),
        ));
        message.add(Field::new(
            FieldKey::target_cmp_id(),
            self.target_comp_id.to_string(),
        ));
        message
    }
}
