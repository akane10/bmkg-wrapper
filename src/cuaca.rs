use crate::Error;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::borrow::Borrow;

pub enum Url {
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

impl Url {
    pub fn to_str(&self) -> &str {
        match self {
            Url::Aceh => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Aceh.xml",
            Url::Bali => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Bali.xml",
            Url::BangkaBelitung => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-BangkaBelitung.xml",
            Url::Banten => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Banten.xml",
            Url::Bengkulu => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Bengkulu.xml",
            Url::DIY => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-DIYogyakarta.xml",
            Url::DKI => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-DKIJakarta.xml",
            Url::Gorontalo => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Gorontalo.xml",
            Url::Jambi => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Jambi.xml",
            Url::JawaBarat => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-JawaBarat.xml",
            Url::JawaTengah => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-JawaTengah.xml",
            Url::JawaTimur => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-JawaTimur.xml",
            Url::Kalbar => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-KalimantanBarat.xml",
            Url::Kalsel => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-KalimantanSelatan.xml",
            Url::Kalteng => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-KalimantanTengah.xml",
            Url::Kaltim => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-KalimantanTimur.xml",
            Url::Kaltara => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-KalimantanUtara.xml",
            Url::KepulauanRiau => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-KepulauanRiau.xml",
            Url::Lampung => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Lampung.xml",
            Url::Maluku => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Maluku.xml",
            Url::MalukuUtara => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-MalukuUtara.xml",
            Url::NTB => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-NusaTenggaraBarat.xml",
            Url::NTT => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-NusaTenggaraTimur.xml",
            Url::Papua => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Papua.xml",
            Url::PapuaBarat => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-PapuaBarat.xml",
            Url::Riau => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Riau.xml",
            Url::SulawesiBarat => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-SulawesiBarat.xml",
            Url::SulawesiSelatan => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-SulawesiSelatan.xml",
            Url::SulawesiTengah => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-SulawesiTengah.xml",
            Url::SulawesiTenggara => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-SulawesiTenggara.xml",
            Url::SulawesiUtara => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-SulawesiUtara.xml",
            Url::SumateraBarat => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-SumateraBarat.xml",
            Url::SumateraSelatan => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-SumateraSelatan.xml",
            Url::SumateraUtara => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-SumateraUtara.xml",
            Url::Indonesia => "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast-Indonesia.xml",
        }
    }
    pub fn from_str<T: Borrow<str>>(s: T) -> Option<Url> {
        match s.borrow().to_lowercase().as_ref() {
            "aceh" => Some(Url::Aceh),
            "bali" => Some(Url::Bali),
            "bangka_belitung" => Some(Url::BangkaBelitung),
            "banten" => Some(Url::Banten),
            "bengkulu" => Some(Url::Bengkulu),
            "dyi" => Some(Url::DIY),
            "dki" => Some(Url::DKI),
            "gorontalo" => Some(Url::Gorontalo),
            "jambi" => Some(Url::Jambi),
            "jabar" => Some(Url::JawaBarat),
            "jateng" => Some(Url::JawaTengah),
            "jatim" => Some(Url::JawaTimur),
            "kalbar" => Some(Url::Kalbar),
            "kalsel" => Some(Url::Kalsel),
            "kalteng" => Some(Url::Kalteng),
            "kaltim" => Some(Url::Kaltim),
            "kaltara" => Some(Url::Kaltara),
            "kepri" => Some(Url::KepulauanRiau),
            "lampung" => Some(Url::Lampung),
            "maluku_utara" => Some(Url::MalukuUtara),
            "maluku" => Some(Url::Maluku),
            "ntb" => Some(Url::NTB),
            "ntt" => Some(Url::NTT),
            "papua" => Some(Url::Papua),
            "papua_barat" => Some(Url::PapuaBarat),
            "riau" => Some(Url::Riau),
            "sulawesi_barat" => Some(Url::SulawesiBarat),
            "sulawesi_selatan" => Some(Url::SulawesiSelatan),
            "sulawesi_tengah" => Some(Url::SulawesiTengah),
            "sulawesi_tenggara" => Some(Url::SulawesiTenggara),
            "sulawesi_utara" => Some(Url::SulawesiUtara),
            "sumatera_barat" => Some(Url::SumateraUtara),
            "sumatera_selatan" => Some(Url::SumateraSelatan),
            "sumatera_utara" => Some(Url::SumateraUtara),
            "indonesia" => Some(Url::Indonesia),
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

pub async fn get_data(url: Url) -> Result<Data, Error> {
    let xml = reqwest::get(url.to_str()).await?.text().await?;
    let data = parse_data(xml)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_data_cuaca_test() {
        let res = get_data(Url::DKI).await;

        assert!(res.is_ok());
    }
}
