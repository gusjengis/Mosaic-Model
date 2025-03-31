use crate::{
    log::Log,
    log_data::{Data, Interval},
};

pub trait Serialize {
    fn to_string(&self) -> String;
}

impl Serialize for Log {
    fn to_string(&self) -> String {
        format!("{},{}", self.timestamp, Serialize::to_string(&self.data))
    }
}

impl Serialize for Data {
    fn to_string(&self) -> String {
        match self {
            Data::Interval(activity) => format!("0,{}", Serialize::to_string(activity)),
            Data::ActiveProgram(program, title) => format!("1,{},{}", program, title),
        }
    }
}

impl Serialize for Interval {
    fn to_string(&self) -> String {
        match self {
            Interval::Start(label) => format!("{}", label),
            Interval::Stop => "\u{001E}".to_string(),
        }
    }
}
