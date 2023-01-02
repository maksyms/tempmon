use derive_more::Display;
use std::{io, num};

#[derive(Debug, Display)]
pub enum W1Error {
    Io(io::Error),
    Parse(num::ParseIntError),
    BadSerialConnection,
    NoSensorsFound,
    NoTemperatureFound,
}

impl From<io::Error> for W1Error {
    fn from(err: io::Error) -> W1Error {
        W1Error::Io(err)
    }
}

impl From<num::ParseIntError> for W1Error {
    fn from(err: num::ParseIntError) -> W1Error {
        W1Error::Parse(err)
    }
}

impl PartialEq for W1Error {
    fn eq(&self, other: &W1Error) -> bool {
        match (self, other) {
            (W1Error::Io(_), W1Error::Io(_)) => true,
            (W1Error::Parse(_), W1Error::Parse(_)) => true,
            (W1Error::BadSerialConnection, W1Error::BadSerialConnection) => true,
            (W1Error::NoSensorsFound, W1Error::NoSensorsFound) => true,
            (W1Error::NoTemperatureFound, W1Error::NoTemperatureFound) => true,
            _ => false,
        }
    }
}
