use std::fmt::{Display, write};

use crate::deserialization::Deserialize;

pub enum Datum {
    Label(String),
    Location(f64, f64),
    URL(String),
}

pub enum Interval {
    Start(String),
    Stop,
}
