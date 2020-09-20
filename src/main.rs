use quickfixr::quickfix::message::message_factory::MessageFactory;
use quickfixr::quickfix::message::message_fix44::{create_factory44, logon_message, order_message};
use quickfixr::quickfix::session::Session;
use quickfixr::quickfix::{main3, main4, main5};
use std::borrow::Borrow;

fn main() {
    // let mut session = Session::new(String::from("127.0.0.1"), String::from("9880"),
    //                                create_factory44());
    // session.start();

    main5::main();
}
