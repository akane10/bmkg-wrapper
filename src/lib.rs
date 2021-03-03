pub mod gempa;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    XmlError(quick_xml::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(error: quick_xml::Error) -> Self {
        Error::XmlError(error)
    }
}
