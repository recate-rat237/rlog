
Example code:
```rs
use rlog::logging::{RLogger, MsgStructElement};

fn main() {
    let mut msg_struct = Vec::<MsgStructElement>::default();
    msg_struct.push(MsgStructElement::DateAndTime);
    
    msg_struct.push(MsgStructElement::Message);
    
    msg_struct.push(MsgStructElement::Mark);
    //let log_g = RLogger::new(msg_struct, 1_i32); // fast ?
    let log_g = RLogger::new(msg_struct, 1);

    for i in 0..4 {
        log_g.clone().log(0, &format!("log_level(0); Num is {}", i), "Info");
        log_g.clone().log(1, &format!("log_level(1); Num is {}", i), "Debug");
        log_g.clone().log(2, &format!("log_level(2); Num is {}", i), "Error");
    }
}
```