use env_logger as logger;
use log::{debug, error, info, warn};
use quickfixr::quickfix::application::Controller;
use quickfixr::quickfix::message::message::Message;
use quickfixr::quickfix::message::message_fix44;
use std::env;
use std::sync::mpsc::Sender;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    logger::init();
    let mut controller = Controller::new(String::from("127.0.0.1"), String::from("9880"));
    controller.start(sends);
}

fn sends(tx: Sender<Message>) {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        info!("send logout");
        tx.send(message_fix44::logout_message());
    }
}
