//pub mod rlog {
    
    #[derive(PartialEq, Clone)]
    pub enum MsgStructElement {
        DateAndTime,
        Message,
        Mark
    }

    #[derive(Clone)]
    pub struct RLogger {
        msg_struct: Vec<MsgStructElement>
    }


    impl RLogger {
        pub fn new(msg_struct: Vec<MsgStructElement>) -> RLogger {
            RLogger { msg_struct: msg_struct }
        }

        pub fn log(self, msg: &str, mark: &str) {
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
//}