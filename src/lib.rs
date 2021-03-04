pub mod cuaca;
pub mod gempa;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    XmlError(quick_xml::Error),
    BmkgError(String),
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

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::BmkgError(error)
    }
}
