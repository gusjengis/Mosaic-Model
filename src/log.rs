use std::fmt::Display;

use crate::{deserialization::Deserialize, log_data::{Data, Datum, Interval}, log_type::Type};

pub struct Log {
    pub timestamp: i64,
    pub data_type: Type,
    pub data: Vec<Datum>,
}

pub struct Interval {
    pub start: i64,
    pub end: i64,
    pub data: Vec<Datum>,
}

pub struct Series {
    logs: Vec<Log>
}

impl Interval {
    from_logs(start: Log, end, Log) {
        Self {
            start: start.timestamp,
            end: end.timestamp,
            
}
    }
}
