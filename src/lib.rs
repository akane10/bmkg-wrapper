use std::fmt;

pub mod cuaca;
pub mod gempa;

const BMKG_BASE_URL: &str = "https://data.bmkg.go.id";

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    XmlError(quick_xml::Error),
    Others(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::ReqwestError(ref x) => write!(f, "{}", x),
            Error::XmlError(ref x) => write!(f, "{}", x),
            Error::Others(ref x) => write!(f, "{}", x),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! error_wrap {
    ($f:ty, $e:expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error {
                $e(f)
            }
        }
    };
}

error_wrap!(reqwest::Error, Error::ReqwestError);
error_wrap!(quick_xml::Error, Error::XmlError);
