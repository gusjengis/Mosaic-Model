use std::fmt::Display;

use crate::{
    log::Log,
    log_data::{Data, Interval},
};

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.timestamp, self.data)
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Interval(activity) => write!(f, "Activity: {}", activity),
            Data::ActiveProgram(program, title) => {
                write!(f, "ActiveProgram: {}, {}", program, title)
            }
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Start(label) => write!(f, "{}", label),
            Interval::Stop => write!(f, "STOP"),
        }
    }
}
