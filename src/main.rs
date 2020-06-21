use quickfixr::quickfix::session::Session;
use quickfixr::quickfix::message::message_fix44::{create_factory44, hello_message, order_message};
use std::borrow::Borrow;
use quickfixr::quickfix::message::message_factory::MessageFactory;

fn main() {
    let mut session = Session::new(String::from("127.0.0.1"), String::from("9880"),
                                   create_factory44());
    loop {
        println!("input method: H(Hello), O(Order)");

        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let input = s.trim();
        if input.eq("H") {
            println!("send hello");
            session.hello();
        } else if input.eq("O") {
            println!("send order");
            session.order();
        } else {
            println!("send fail");
        }
        println!();
    }
}
