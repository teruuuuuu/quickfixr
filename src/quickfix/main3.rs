use std::sync::{Mutex, Arc, mpsc};
use std::thread;

extern crate crossbeam;

use rand::prelude::*;
use std::time::Duration;
use actix_rt::System;
use std::sync::mpsc::{SendError, Sender, Receiver};
use std::thread::JoinHandle;

#[derive(Debug, Clone)]
struct Message { method: String, from: String }

impl Message {
    pub fn new(method: String, from: String) -> Self {
        Message { method, from }
    }
}

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_arc = Arc::new(Mutex::new(tx));
    let rx_arc = Arc::new(Mutex::new(rx));
    let mut count = Arc::new(Mutex::new(0));

    for handle in vec![
        start_random(Arc::clone(&tx_arc)),
        start_input(Arc::clone(&tx_arc)),
        start_receive(rx_arc, count)
    ] {
        handle.join().unwrap();
    }
}

fn start_random(tx_arc: Arc<Mutex<Sender<Message>>>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            let random_val: f64 = rng.gen();
            let message = if random_val > 0.5 {
                Message::new(String::from("up"), String::from("random"))
            } else {
                Message::new(String::from("down"), String::from("random"))
            };
            tx_arc.lock().unwrap().send(message);
            thread::sleep(Duration::from_secs(1));
        }
    })
}

fn start_input(tx_arc: Arc<Mutex<Sender<Message>>>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
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
                Some(message) => {
                    tx_arc.lock().unwrap().send(message);
                }
                _ => {}
            }
        }
    })
}

fn start_receive(rx_arc: Arc<Mutex<Receiver<Message>>>, count_arc: Arc<Mutex<i32>>) -> JoinHandle<()> {
    let mut count = Arc::clone(&count_arc);
    thread::spawn(move || {
        loop {
            let rx = Arc::clone(&rx_arc);
            match rx.lock().unwrap().recv() {
                Ok(message) => {
                    println!("receive: {:?}", message);
                    let mut num = count.lock().unwrap();
                    if message.method.eq("up") {
                        *num += 1;
                    } else if message.method.eq("down") {
                        *num -= 1;
                    }
                    println!("count: {:?}", num);
                }
                _ => { println!("not") }
            };


        }
    })
}