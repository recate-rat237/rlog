mod rlog;
use std::time::Instant;

pub use crate::rlog::rlog::*;

fn main() {
    let starttime = Instant::now();
    let mut msg_struct = Vec::<MsgStructElement>::default();
    msg_struct.push(MsgStructElement::DateAndTime);
    msg_struct.push(MsgStructElement::Mark);
    msg_struct.push(MsgStructElement::Message);
    let log_g = RLogger::new(msg_struct);
    for i in 0..1024 {
        log_g.clone().log(&format!("Logged! Num is {}", i), "Debug");
    }
    println!("Time: {:?}", starttime.elapsed());
}
