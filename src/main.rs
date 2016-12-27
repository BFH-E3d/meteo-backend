extern crate glob;
extern crate regex;

use std::str::FromStr;
use std::path::PathBuf;
use regex::Regex;
use std::fs::File;
use std::io::Read;

use std::error;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fmt;

#[derive(Debug)]
enum TmpError {
    IOError(std::io::Error),
    RegexError(regex::Error),
    FloatError(std::num::ParseFloatError),
    Utf8Error(std::str::Utf8Error),
    Unknow,
}

impl From<std::io::Error> for TmpError {
    fn from(err: std::io::Error) -> TmpError {
        TmpError::IOError(err)
    }
}

impl From<regex::Error> for TmpError {
    fn from(err: regex::Error) -> TmpError {
        TmpError::RegexError(err)
    }
}

impl From<std::num::ParseFloatError> for TmpError {
    fn from(err: std::num::ParseFloatError) -> TmpError {
        TmpError::FloatError(err)
    }
}

impl From<std::str::Utf8Error> for TmpError {
    fn from(err: std::str::Utf8Error) -> TmpError {
        TmpError::Utf8Error(err)
    }
}

impl Error for TmpError {
    /// A short description of the error.
    fn description(&self) -> &str {
        match *self {
            TmpError::IOError(ref err) => err.description(),
            TmpError::FloatError(ref err) => err.description(),
            TmpError::RegexError(ref err) => err.description(),
            TmpError::Utf8Error(ref err) => err.description(),
            TmpError::Unknow => "Unknow error.",
        }
    }

    /// The lower level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            TmpError::IOError(ref err) => Some(err),
            TmpError::FloatError(ref err) => Some(err),
            TmpError::RegexError(ref err) => Some(err),
            TmpError::Utf8Error(ref err) => Some(err),
            TmpError::Unknow => None,
        }
    }
}

impl Display for TmpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            TmpError::IOError(ref err) => write!(f, "IO error: {}", err),
            TmpError::FloatError(ref err) => write!(f, "Parse error: {}", err),
            TmpError::RegexError(ref err) => write!(f, "Parse error: {}", err),
            TmpError::Utf8Error(ref err) => write!(f, "Parse error: {}", err),
            TmpError::Unknow => write!(f, "Unknow error"),
        }
    }
}


fn read_sensor() -> Result<f32, TmpError> {

    let data = search_sensors();
    for path in data {
        let mut file = File::open(path)?;
        let mut text: Vec<u8> = Vec::with_capacity(75);

        let re_crc = Regex::new(r"(YES)")?;

        file.read_to_end(&mut text)?;

        if re_crc.is_match(std::str::from_utf8(&text)?) {
            let re_temp = Regex::new(r"t=(-?\d+)")?;

            match re_temp.captures(std::str::from_utf8(&text)?).unwrap().at(1) {
                Some(tmp) => {
                    let temperature = f32::from_str(tmp)?;
                    return Ok(temperature / 1000.0);
                }
                None => {}
            };
        }
    }

    Err(TmpError::Unknow)

}
fn search_sensors() -> Vec<PathBuf> {

    glob::glob("/sys/bus/w1/devices/28-*/w1_slave").unwrap().filter_map(Result::ok).collect()
}


fn main() {
    println!("{}", read_sensor().unwrap());
}
