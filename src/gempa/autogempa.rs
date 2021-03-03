use crate::*;
use quick_xml;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::borrow::Borrow;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Autogempa {
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

impl Autogempa {
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
    fn set<T: Borrow<str>>(&mut self, k: T, v: T) {
        let v = v.borrow().to_owned();
        match k.borrow().to_lowercase().as_str() {
            "tanggal" => {
                self.tanggal = Some(v);
            }
            "jam" => {
                self.jam = Some(v);
            }
            "coordinates" => {
                self.coordinates = Some(v);
            }
            "lintang" => {
                self.lintang = Some(v);
            }
            "bujur" => {
                self.bujur = Some(v);
            }
            "magnitude" => {
                self.magnitude = Some(v);
            }
            "kedalaman" => {
                self.kedalaman = Some(v);
            }
            "wilayah" => {
                self.wilayah = Some(v);
            }
            "potensi" => {
                self.potensi = Some(v);
            }
            "dirasakan" => {
                self.dirasakan = Some(v);
            }
            "shakemap" => {
                self.shakemap = Some(v);
            }
            _ => (),
        }
    }
    pub fn to_json(self) -> JsonValue {
        json!(self)
    }
}

fn parse_data(xml: &str) -> Result<Autogempa, Box<dyn Error>> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut g = Autogempa::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"Tanggal" | b"Jam" | b"coordinates" | b"Lintang" | b"Bujur" | b"Magnitude"
                | b"Kedalaman" | b"Wilayah" | b"Potensi" | b"Dirasakan" | b"Shakemap" => {
                    let text = reader.read_text(e.name(), &mut Vec::new());
                    let v = e.unescape_and_decode(&reader).expect("Error!");
                    g.set(v, text.unwrap().clone());
                }
                _ => (),
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        buf.clear();
    }

    Ok(g)
}

pub async fn get_data() -> Result<Autogempa, Box<dyn Error>> {
    let xml = fetch_data(Url::Autogempa.to_str()).await?;
    parse_data(&xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autogempa_set_test() {
        let mut data = Autogempa::new();
        data.set("tanggal", "22-11-2021");
        let expected = Some("22-11-2021".to_string());

        assert_eq!(data.tanggal, expected);
    }

    #[test]
    fn parse_data_test() {
        let data = "<Tanggal>30-Jul-20</Tanggal><Jam>09:51:20 WIB</Jam>";
        let expected = Some("30-Jul-20".to_string());

        assert_eq!(parse_data(data).unwrap().tanggal, expected);
    }
}
