#[derive(PartialEq, Clone)]
pub enum MsgStructElement {
    DateAndTime,
    Message,
    Mark
}

#[derive(Clone)]
pub struct RLogger {
    msg_struct: Vec<MsgStructElement>,
    log_level: i32
}


impl RLogger {
    pub fn new(msg_struct: Vec<MsgStructElement>, log_level: i32) -> RLogger {
        RLogger { msg_struct: msg_struct, log_level: log_level }
    }

    pub fn log(self, log_level: i32, msg: &str, mark: &str) {
        if log_level >= self.log_level {
            for i in self.msg_struct {
                if i == MsgStructElement::Mark {
                    print!("[{}]", mark);
                } else if i == MsgStructElement::Message {
                    print!("{}", msg);
                } else if i == MsgStructElement::DateAndTime {
                    print!("[{}]", chrono::Utc::now());
                } 
            }
            println!();
        }
    }

}