use crate::{Error, BMKG_BASE_URL};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::borrow::Borrow;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Url {
    Autogempa,
    GempaTerkini,
    GempaDirasakan,
}

impl Url {
    pub fn to_str(&self) -> String {
        match self {
            Url::Autogempa => format!("{}/DataMKG/TEWS/autogempa.xml", BMKG_BASE_URL),
            Url::GempaTerkini => format!("{}/DataMKG/TEWS/gempaterkini.xml", BMKG_BASE_URL),
            Url::GempaDirasakan => format!("{}/DataMKG/TEWS/gempadirasakan.xml", BMKG_BASE_URL),
        }
    }
    pub fn from_str<T: Borrow<str>>(s: T) -> Option<Url> {
        match s.borrow().to_lowercase().as_ref() {
            "autogempa" => Some(Url::Autogempa),
            "gempaterkini" => Some(Url::GempaTerkini),
            "gempadirasakan" => Some(Url::GempaDirasakan),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gempa {
    pub tanggal: Option<String>,
    pub jam: Option<String>,
    pub coordinates: Option<String>,
    pub lintang: Option<String>,
    pub bujur: Option<String>,
    pub magnitude: Option<String>,
    pub kedalaman: Option<String>,
    pub wilayah: Option<String>,
    pub potensi: Option<String>,
    pub dirasakan: Option<String>,
    pub shakemap: Option<String>,
}

impl Gempa {
    fn new() -> Self {
        Self {
            tanggal: None,
            jam: None,
            coordinates: None,
            lintang: None,
            bujur: None,
            magnitude: None,
            kedalaman: None,
            wilayah: None,
            potensi: None,
            dirasakan: None,
            shakemap: None,
        }
    }
    fn set<T: Borrow<str>>(&mut self, k: T, v: T) -> Result<(), Error> {
        let v = v.borrow().to_owned();
        match k.borrow().to_lowercase().as_str() {
            "tanggal" => {
                self.tanggal = Some(v);
                Ok(())
            }
            "jam" => {
                self.jam = Some(v);
                Ok(())
            }
            "coordinates" => {
                self.coordinates = Some(v);
                Ok(())
            }
            "lintang" => {
                self.lintang = Some(v);
                Ok(())
            }
            "bujur" => {
                self.bujur = Some(v);
                Ok(())
            }
            "magnitude" => {
                self.magnitude = Some(v);
                Ok(())
            }
            "kedalaman" => {
                self.kedalaman = Some(v);
                Ok(())
            }
            "wilayah" => {
                self.wilayah = Some(v);
                Ok(())
            }
            "potensi" => {
                self.potensi = Some(v);
                Ok(())
            }
            "dirasakan" => {
                self.dirasakan = Some(v);
                Ok(())
            }
            "shakemap" => {
                self.shakemap = Some(v);
                Ok(())
            }
            x => {
                let msg = format!("unknown field {}", x);
                Err(Error::Others(msg))
            }
        }
    }
    pub fn to_json(self) -> JsonValue {
        json!(self)
    }
}

fn parse_data<T: Borrow<str>>(xml: T) -> Result<Vec<Gempa>, Error> {
    let mut reader = Reader::from_str(xml.borrow());
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut g = Gempa::new();
    let mut res: Result<Vec<Gempa>, Error> = Ok(Vec::new());

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"Tanggal" | b"Jam" | b"coordinates" | b"Lintang" | b"Bujur" | b"Magnitude"
                | b"Kedalaman" | b"Wilayah" | b"Potensi" | b"Dirasakan" | b"Shakemap" => {
                    let mut text = reader.read_text(e.name(), &mut Vec::new())?;
                    let v = e.unescape_and_decode(&reader)?;
                    if v == "Shakemap" {
                        text = format!("{}/DataMKG/TEWS/{}", BMKG_BASE_URL, text);
                    }
                    match g.set(v, text.clone()) {
                        _ => (),
                    };
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name() {
                b"gempa" => {
                    res = res.and_then(|mut x| {
                        x.push(g);
                        Ok(x)
                    });
                    g = Gempa::new();
                }
                _ => (),
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => {
                res = Err(Error::XmlError(e));
                break;
            }
            _ => (), // There are several other `Event`s we do not consider here
        }

        buf.clear();
    }

    // println!("res : {:?}", res);
    res
}

pub async fn get_data(url: Url) -> Result<Vec<Gempa>, Error> {
    let xml = reqwest::get(&url.to_str()).await?.text().await?;
    parse_data(xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gempa_set_test() {
        let mut data = Gempa::new();
        data.set("tanggal", "22-11-2021").unwrap();
        let expected = Some("22-11-2021".to_string());

        assert_eq!(data.tanggal, expected);
    }

    #[test]
    fn parse_data_test() {
        let data = "<gempa><Tanggal>30-Jul-20</Tanggal><Jam>09:51:20 WIB</Jam></gempa>";
        let expected = Some("30-Jul-20".to_string());

        assert_eq!(parse_data(data).unwrap()[0].tanggal, expected);
    }

    #[test]
    fn parse_data_without_gempa_tag_test() {
        let data = "<Tanggal>30-Jul-20</Tanggal><Jam>09:51:20 WIB</Jam>";

        assert_eq!(parse_data(data).unwrap().len(), 0);
    }

    #[tokio::test]
    async fn autogempa_get_data() {
        let data = get_data(Url::Autogempa).await.unwrap();

        assert!(data[0].tanggal.is_some());
    }

    #[tokio::test]
    async fn gempaterkini_get_data() {
        let data = get_data(Url::GempaTerkini).await.unwrap();

        assert!(data[0].tanggal.is_some());
    }

    #[test]
    fn url_from_str_test() {
        let data = "gempaterkini";
        let expected = Url::GempaTerkini;

        assert_eq!(Url::from_str(data).unwrap(), expected);
    }

    #[test]
    fn url_from_str_but_with_string_test() {
        let data = String::from("gempaterkini");
        let expected = Url::GempaTerkini;

        assert_eq!(Url::from_str(data).unwrap(), expected);
    }
}
