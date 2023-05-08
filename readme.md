
Example code:
```rs
use rlog::logging::{RLogger, MsgStructElement};

fn main() {
    let mut msg_struct = Vec::<MsgStructElement>::default();
    msg_struct.push(MsgStructElement::DateAndTime);
    
    msg_struct.push(MsgStructElement::Message);
    
    msg_struct.push(MsgStructElement::Mark);
    
    let log_g = RLogger::new(msg_struct, 1);

    for i in 0..4 {
        log_g.log(0, &format!("log_level(0); Num is {}", i), "Info");
        log_g.log(1, &format!("log_level(1); Num is {}", i), "Debug");
        log_g.log(2, &format!("log_level(2); Num is {}", i), "Error");
    }
}
```
Here is output 
```cmd
[YYYY-MM-DD HH:MM:SS.:9 UTC]log_level(0); Num is 0[Info]
[YYYY-MM-DD HH:MM:SS.:9 UTC]log_level(1); Num is 0[Debug]
[YYYY-MM-DD HH:MM:SS.:9 UTC]log_level(0); Num is 1[Info]
[YYYY-MM-DD HH:MM:SS.:9 UTC]log_level(1); Num is 1[Debug]
[YYYY-MM-DD HH:MM:SS.:9 UTC]log_level(0); Num is 2[Info]
[YYYY-MM-DD HH:MM:SS.:9 UTC]log_level(1); Num is 2[Debug]
[YYYY-MM-DD HH:MM:SS.:9 UTC]log_level(0); Num is 3[Info]
[YYYY-MM-DD HH:MM:SS.:9 UTC]log_level(1); Num is 3[Debug]
```
#
In this example, msg_struct is the message structure: \[DateAndTime\]Message\[Mark\]

#
```rs
log_g.log(2, &format!("log_level(2); Num is {}", i), "Error");
```
Is not working because log_level is 1.

