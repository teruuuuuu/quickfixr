extern crate crossbeam;

use rand::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use actix_rt::System;
use std::sync::mpsc::SendError;

#[derive(Debug, Clone)]
struct Message { method: String, from: String }

impl Message {
    pub fn new(method: String, from: String) -> Self {
        Message { method, from }
    }
}

#[derive(Debug)]
struct Executor { count: i16 }

macro_rules! send {
    ($tx:tt, $message:ident) => {
    let mut count = 0;
    loop {
        let result = $tx.send($message.clone());
        match result {
          Err (_) => {
            println!("send error:");
            count += 1;
            if count > 10 {
              println!("send error over 10 times");
              break;
            }
            thread::sleep(Duration::from_millis(50));
          }
          Ok(_) => {
            break;
          }
        }
      }
    }
}

impl Executor {
    fn new() -> Self {
        Self { count: 0 }
    }

    pub fn start(&mut self) {
        crossbeam::scope(|scope| {
            let (tx, rx) = mpsc::channel();
            let txc = mpsc::Sender::clone(&tx);
            self.start_random(scope, tx);
            self.start_input(scope, txc);

            for message in rx {
                self.receive_message(message);
            }
        });
    }

    fn start_random(&mut self, scope: &crossbeam::thread::Scope<'_>, tx: std::sync::mpsc::Sender<Message>) -> () {
        scope.spawn(move |s| {
            loop {
                let mut rng = rand::thread_rng();
                let random_val: f64 = rng.gen();
                let message = if random_val > 0.5 {
                    Message::new(String::from("up"), String::from("random"))
                } else {
                    Message::new(String::from("down"), String::from("random"))
                };
                send!(tx, message);
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    fn start_input(&mut self, scope: &crossbeam::thread::Scope<'_>, tx: std::sync::mpsc::Sender<Message>) -> () {
        scope.spawn(move |s| {
            loop {
                let tx1 = tx.clone();
                println!("input: U(Up) or D(Down)");
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).ok();
                let input = s.trim();
                let messageOp = match input {
                    "U" => Some(Message::new(String::from("up"), String::from("input"))),
                    "D" => Some(Message::new(String::from("dodn"), String::from("input"))),
                    _ => {
                        println!("invalid input: {:?}", input);
                        None
                    }
                };
                match messageOp {
                    Some(message) => {send!(tx, message);}
                    _ => {}
                }
            }
        });
    }

    fn send_message(self, tx: std::sync::mpsc::Sender<Message>, message: Message) {
        tx.send(message);
    }

    fn receive_message(&mut self, message: Message) {
        println!("receive: {:?}", message);
        if message.method.eq("up") {
            self.count += 1;
        } else if message.method.eq("down") {
            self.count -= 1;
        }
        println!("count: {:?}", self.count);
    }
}

pub fn main() {
    let mut executor = Executor::new();
    executor.start();
}
