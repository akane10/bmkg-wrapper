use crate::{Error, BMKG_BASE_URL};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::borrow::Borrow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub name: String,
    pub value: String,
    pub url_param: String,
}

impl Domain {
    pub fn get_data() -> Vec<Domain> {
        let bytes = include_bytes!("../domain_list.json");
        let data = String::from_utf8_lossy(bytes);
        let data_domain: Vec<Domain> = serde_json::from_str(&data).unwrap();

        data_domain
    }
}

pub enum Province {
    Aceh,
    Bali,
    BangkaBelitung,
    Banten,
    Bengkulu,
    DIY,
    DKI,
    Gorontalo,
    Jambi,
    JawaBarat,
    JawaTengah,
    JawaTimur,
    Kalbar,
    Kalsel,
    Kalteng,
    Kaltim,
    Kaltara,
    KepulauanRiau,
    Lampung,
    Maluku,
    MalukuUtara,
    NTB,
    NTT,
    Papua,
    PapuaBarat,
    Riau,
    SulawesiBarat,
    SulawesiSelatan,
    SulawesiTengah,
    SulawesiTenggara,
    SulawesiUtara,
    SumateraBarat,
    SumateraSelatan,
    SumateraUtara,
    Indonesia,
}

impl Province {
    fn build_url(s: &str) -> String {
        let s = Domain::get_data()
            .into_iter()
            .find(|x| x.value == s)
            .unwrap();

        format!(
            "{}/DataMKG/MEWS/DigitalForecast/{}",
            BMKG_BASE_URL, s.url_param
        )
    }
    pub fn to_url(&self) -> String {
        match self {
            Self::Aceh => Self::build_url("aceh"),
            Self::Bali => Self::build_url("bali"),
            Self::BangkaBelitung => Self::build_url("bangka_belitung"),
            Self::Banten => Self::build_url("banten"),
            Self::Bengkulu => Self::build_url("bengkulu"),
            Self::DIY => Self::build_url("diy"),
            Self::DKI => Self::build_url("dki"),
            Self::Gorontalo => Self::build_url("gorontalo"),
            Self::Jambi => Self::build_url("jambi"),
            Self::JawaBarat => Self::build_url("jabar"),
            Self::JawaTengah => Self::build_url("jateng"),
            Self::JawaTimur => Self::build_url("jatim"),
            Self::Kalbar => Self::build_url("kalbar"),
            Self::Kalsel => Self::build_url("kalsel"),
            Self::Kalteng => Self::build_url("kalteng"),
            Self::Kaltim => Self::build_url("kaltim"),
            Self::Kaltara => Self::build_url("kaltara"),
            Self::KepulauanRiau => Self::build_url("kepri"),
            Self::Lampung => Self::build_url("lampung"),
            Self::Maluku => Self::build_url("maluku"),
            Self::MalukuUtara => Self::build_url("maluku_utara"),
            Self::NTB => Self::build_url("ntb"),
            Self::NTT => Self::build_url("ntt"),
            Self::Papua => Self::build_url("papua"),
            Self::PapuaBarat => Self::build_url("papua_barat"),
            Self::Riau => Self::build_url("riau"),
            Self::SulawesiBarat => Self::build_url("sulawesi_barat"),
            Self::SulawesiSelatan => Self::build_url("sulawesi_selatan"),
            Self::SulawesiTengah => Self::build_url("sulawesi_tengah"),
            Self::SulawesiTenggara => Self::build_url("sulawesi_tenggara"),
            Self::SulawesiUtara => Self::build_url("sulawesi_utara"),
            Self::SumateraBarat => Self::build_url("sumatera_barat"),
            Self::SumateraSelatan => Self::build_url("sumatera_selatan"),
            Self::SumateraUtara => Self::build_url("sumatera_utara"),
            Self::Indonesia => Self::build_url("indonesia"),
        }
    }
    pub fn from_str<T: Borrow<str>>(s: T) -> Option<Self> {
        match s.borrow().to_lowercase().as_ref() {
            "aceh" => Some(Self::Aceh),
            "bali" => Some(Self::Bali),
            "bangka_belitung" => Some(Self::BangkaBelitung),
            "banten" => Some(Self::Banten),
            "bengkulu" => Some(Self::Bengkulu),
            "diy" => Some(Self::DIY),
            "dki" => Some(Self::DKI),
            "gorontalo" => Some(Self::Gorontalo),
            "jambi" => Some(Self::Jambi),
            "jabar" => Some(Self::JawaBarat),
            "jateng" => Some(Self::JawaTengah),
            "jatim" => Some(Self::JawaTimur),
            "kalbar" => Some(Self::Kalbar),
            "kalsel" => Some(Self::Kalsel),
            "kalteng" => Some(Self::Kalteng),
            "kaltim" => Some(Self::Kaltim),
            "kaltara" => Some(Self::Kaltara),
            "kepri" => Some(Self::KepulauanRiau),
            "lampung" => Some(Self::Lampung),
            "maluku_utara" => Some(Self::MalukuUtara),
            "maluku" => Some(Self::Maluku),
            "ntb" => Some(Self::NTB),
            "ntt" => Some(Self::NTT),
            "papua" => Some(Self::Papua),
            "papua_barat" => Some(Self::PapuaBarat),
            "riau" => Some(Self::Riau),
            "sulawesi_barat" => Some(Self::SulawesiBarat),
            "sulawesi_selatan" => Some(Self::SulawesiSelatan),
            "sulawesi_tengah" => Some(Self::SulawesiTengah),
            "sulawesi_tenggara" => Some(Self::SulawesiTenggara),
            "sulawesi_utara" => Some(Self::SulawesiUtara),
            "sumatera_barat" => Some(Self::SumateraUtara),
            "sumatera_selatan" => Some(Self::SumateraSelatan),
            "sumatera_utara" => Some(Self::SumateraUtara),
            "indonesia" => Some(Self::Indonesia),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub unit: Option<String>,
    pub value: Option<String>,
}

impl Value {
    fn new() -> Self {
        Self {
            unit: None,
            value: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    #[serde(rename = "type", default)]
    pub typ: Option<String>,
    pub h: Option<String>,
    pub datetime: Option<String>,
    pub values: Vec<Value>,
    // value: Option<String>,
}

impl TimeRange {
    fn new() -> Self {
        Self {
            typ: None,
            h: None,
            datetime: None,
            values: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub id: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type", default)]
    pub typ: Option<String>,
    pub timeranges: Vec<TimeRange>,
}

impl Parameter {
    fn new() -> Self {
        Self {
            id: None,
            description: None,
            typ: None,
            timeranges: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub lang: Option<String>,
    pub text: Option<String>,
}

impl Name {
    fn new() -> Self {
        Self {
            lang: None,
            text: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub id: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub coordinate: Option<String>,
    #[serde(rename = "type", default)]
    pub typ: Option<String>,
    pub region: Option<String>,
    pub level: Option<String>,
    pub description: Option<String>,
    pub domain: Option<String>,
    pub tags: Option<String>,
    pub names: Vec<Name>,
    pub parameters: Vec<Parameter>,
}

impl Area {
    fn new() -> Self {
        Self {
            id: None,
            latitude: None,
            longitude: None,
            coordinate: None,
            typ: None,
            region: None,
            level: None,
            description: None,
            domain: None,
            tags: None,
            names: Vec::new(),
            parameters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub timestamp: Option<String>,
    pub year: Option<String>,
    pub month: Option<String>,
    pub day: Option<String>,
    pub hour: Option<String>,
    pub minute: Option<String>,
    pub second: Option<String>,
}

impl Issue {
    fn new() -> Self {
        Self {
            timestamp: None,
            year: None,
            month: None,
            day: None,
            hour: None,
            minute: None,
            second: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forecast {
    pub domain: Option<String>,
    pub issue: Option<Issue>,
    pub areas: Vec<Area>,
}

impl Forecast {
    fn new() -> Self {
        Self {
            domain: None,
            issue: None,
            areas: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub source: Option<String>,
    pub production_center: Option<String>,
    pub forecast: Option<Forecast>,
}

impl Data {
    fn new() -> Self {
        Self {
            source: None,
            production_center: None,
            forecast: None,
        }
    }

    pub fn to_json(self) -> JsonValue {
        json!(self)
    }
}

fn parse_data<T: Borrow<str>>(xml: T) -> Result<Data, Error> {
    let mut reader = Reader::from_str(xml.borrow());
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut data = Data::new();
    let mut forecast = Forecast::new();
    let mut issue = Issue::new();
    let mut area = Area::new();
    let mut name = Name::new();
    let mut parameter = Parameter::new();
    let mut time_range = TimeRange::new();
    let mut value = Value::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"data" => {
                    let values = e
                        .attributes()
                        .map(|a| a.unwrap().unescape_and_decode_value(&reader).unwrap())
                        .collect::<Vec<_>>();
                    let keys = e
                        .attributes()
                        .map(|a| std::str::from_utf8(a.unwrap().key).unwrap())
                        .collect::<Vec<_>>();
                    let kvs: Vec<(_, _)> = keys.iter().zip(values.iter()).collect();
                    for (k, v) in kvs {
                        match k.as_ref() {
                            "source" => data.source = Some(v.to_string()),
                            "productioncenter" => data.production_center = Some(v.to_string()),
                            _ => (),
                        }
                    }
                }
                b"forecast" => {
                    let values = e
                        .attributes()
                        .map(|a| a.unwrap().unescape_and_decode_value(&reader).unwrap())
                        .collect::<Vec<_>>();

                    for v in values {
                        forecast.domain = Some(v.to_string())
                    }
                }
                b"timestamp" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    issue.timestamp = Some(text);
                }
                b"year" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    issue.year = Some(text);
                }
                b"month" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    issue.month = Some(text);
                }
                b"day" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    issue.day = Some(text);
                }
                b"hour" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    issue.hour = Some(text);
                }
                b"minute" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    issue.minute = Some(text);
                }
                b"second" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    issue.second = Some(text);
                }
                b"area" => {
                    let values = e
                        .attributes()
                        .map(|a| a.unwrap().unescape_and_decode_value(&reader).unwrap())
                        .collect::<Vec<_>>();
                    let keys = e
                        .attributes()
                        .map(|a| std::str::from_utf8(a.unwrap().key).unwrap())
                        .collect::<Vec<_>>();
                    let kvs: Vec<(_, _)> = keys.iter().zip(values.iter()).collect();
                    for (k, v) in kvs {
                        match k.as_ref() {
                            "id" => area.id = Some(v.to_string()),
                            "latitude" => area.latitude = Some(v.to_string()),
                            "longitude" => area.longitude = Some(v.to_string()),
                            "coordinate" => area.coordinate = Some(v.to_string()),
                            "type" => area.typ = Some(v.to_string()),
                            "region" => area.region = Some(v.to_string()),
                            "level" => area.level = Some(v.to_string()),
                            "description" => area.description = Some(v.to_string()),
                            "domain" => area.domain = Some(v.to_string()),
                            "tags" => area.tags = Some(v.to_string()),
                            _ => (),
                        }
                    }
                }
                b"name" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    name.text = Some(text);
                    let values = e
                        .attributes()
                        .map(|a| a.unwrap().unescape_and_decode_value(&reader).unwrap())
                        .collect::<Vec<_>>();

                    for v in values {
                        name.lang = Some(v.to_string())
                    }

                    area.names.push(name);
                    name = Name::new();
                }
                b"parameter" => {
                    let values = e
                        .attributes()
                        .map(|a| a.unwrap().unescape_and_decode_value(&reader).unwrap())
                        .collect::<Vec<_>>();
                    let keys = e
                        .attributes()
                        .map(|a| std::str::from_utf8(a.unwrap().key).unwrap())
                        .collect::<Vec<_>>();
                    let kvs: Vec<(_, _)> = keys.iter().zip(values.iter()).collect();
                    for (k, v) in kvs {
                        match k.as_ref() {
                            "id" => parameter.id = Some(v.to_string()),
                            "type" => parameter.typ = Some(v.to_string()),
                            "description" => parameter.description = Some(v.to_string()),
                            _ => (),
                        }
                    }
                }
                b"timerange" => {
                    let values = e
                        .attributes()
                        .map(|a| a.unwrap().unescape_and_decode_value(&reader).unwrap())
                        .collect::<Vec<_>>();
                    let keys = e
                        .attributes()
                        .map(|a| std::str::from_utf8(a.unwrap().key).unwrap())
                        .collect::<Vec<_>>();
                    let kvs: Vec<(_, _)> = keys.iter().zip(values.iter()).collect();
                    for (k, v) in kvs {
                        match k.as_ref() {
                            "h" => time_range.h = Some(v.to_string()),
                            "type" => time_range.typ = Some(v.to_string()),
                            "datetime" => time_range.datetime = Some(v.to_string()),
                            _ => (),
                        }
                    }
                }
                b"value" => {
                    let text = reader.read_text(e.name(), &mut Vec::new())?;
                    value.value = Some(text);
                    let values = e
                        .attributes()
                        .map(|a| a.unwrap().unescape_and_decode_value(&reader).unwrap())
                        .collect::<Vec<_>>();

                    for v in values {
                        value.unit = Some(v.to_string())
                    }

                    time_range.values.push(value);
                    value = Value::new();
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name() {
                b"data" => {}
                b"forecast" => {
                    data.forecast = Some(forecast.clone());
                }
                b"issue" => {
                    forecast.issue = Some(issue.clone());
                }
                b"area" => {
                    forecast.areas.push(area);
                    area = Area::new();
                }
                b"parameter" => {
                    area.parameters.push(parameter);
                    parameter = Parameter::new();
                }
                b"timerange" => {
                    parameter.timeranges.push(time_range);
                    time_range = TimeRange::new();
                }
                _ => (),
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        buf.clear();
    }

    // println!("data {:#?}", data);
    Ok(data)
}

pub async fn get_data(p: Province) -> Result<Data, Error> {
    let xml = reqwest::get(&p.to_url()).await?.text().await?;
    let data = parse_data(xml)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_data_cuaca_test() {
        let res = get_data(Province::DKI).await;

        assert!(res.is_ok());
    }

    #[test]
    fn domain_list_get_data_test() {
        let data = Domain::get_data();

        assert!(data.len() == 35);
    }

    #[test]
    fn build_url_test() {
        let data = Province::build_url("bengkulu");
        let expected = format!(
            "{}/DataMKG/MEWS/DigitalForecast/{}",
            BMKG_BASE_URL, "DigitalForecast-Bengkulu.xml"
        );

        assert!(data == expected);
    }
}
