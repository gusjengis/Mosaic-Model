use crate::{
    deserialization_errors::DeserializeError,
    error_handling::handle_deserialize_error,
    log::Log,
    log_data::{Data, Interval},
};

pub trait Serialize {
    fn to_string(&self) -> Self;
}

pub trait Deserialize: Sized {
    fn from_string(string: String) -> Result<Self, DeserializeError>;
}

impl Deserialize for Log {
    fn from_string(string: String) -> Result<Self, DeserializeError> {
        let mut iter = string.split(",");
        let timestamp_string = iter
            .next()
            .ok_or(DeserializeError::MissingField("Timestamp"))
            .unwrap();

        let timestamp = timestamp_string
            .parse()
            .map_err(|e| DeserializeError::ParseError("Timestamp"))
            .unwrap();

        let remaining_data = iter.collect::<Vec<_>>().join(",");

        if remaining_data.is_empty() {
            return Err(DeserializeError::MissingField("Log Data"));
        }

        let data = Data::from_string(remaining_data);

        if let Err(e) = &data {
            handle_deserialize_error(e);
        }

        Ok(Self {
            timestamp,
            data: data.unwrap(),
        })
    }
}

impl Deserialize for Data {
    fn from_string(string: String) -> Result<Self, DeserializeError> {
        let mut iter = string.split(",");
        let id_string = iter
            .next()
            .ok_or(DeserializeError::MissingField("Variant ID"))
            .unwrap();

        let id: u32 = id_string
            .parse()
            .map_err(|e| DeserializeError::ParseError("Variant ID"))
            .unwrap();

        let remaining_data = iter.collect::<Vec<_>>().join(",");

        if remaining_data.is_empty() {
            return match id {
                _ => Err(DeserializeError::MissingField("Data_Data")),
            };
        }

        match id {
            0 => Ok(Data::Activity(
                Interval::from_string(remaining_data).unwrap(),
            )),
            1 => {
                let mut iter = remaining_data.split(",");
                let program = iter
                    .next()
                    .ok_or(DeserializeError::MissingField("Active Program"))
                    .unwrap()
                    .to_string();

                let title = iter
                    .next()
                    .ok_or(DeserializeError::MissingField("Active Program Title"))
                    .unwrap()
                    .to_string();

                Ok(Data::ActiveProgram(program, title))
            }
            _ => Err(DeserializeError::ParseError("variant_id out of range")),
        }
    }
}

impl Deserialize for Interval {
    fn from_string(string: String) -> Result<Self, DeserializeError> {
        if string == "\u{001E}" {
            return Ok(Interval::Stop);
        }

        return Ok(Interval::Start(string));
    }
}
