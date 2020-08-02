// use std::collections::HashMap;
use serde_json::{json, Value as ValueJson};
use xml::reader::{EventReader, XmlEvent};

// https://data.bmkg.go.id/autogempa.xml
// https://data.bmkg.go.id/gempadirasakan.xml (mutiple)

pub enum Url {
  Autogempa,
  GempaTerkini,
  GempaDirasakan,
  LastTsunami,
  EnAutogempa,
  EnGempaTerkini,
}

pub struct Gempa;

impl Gempa {
  pub fn get_data(url: Url) -> Result<Vec<Vec<(String, String)>>, Box<dyn std::error::Error>> {
    let url = match url {
      Url::Autogempa => String::from("https://data.bmkg.go.id/autogempa.xml"),
      Url::GempaTerkini => String::from("https://data.bmkg.go.id/gempaterkini.xml"),
      Url::GempaDirasakan => String::from("https://data.bmkg.go.id/gempadirasakan.xml"),
      Url::LastTsunami => String::from("https://data.bmkg.go.id/lasttsunami.xml"),
      Url::EnAutogempa => String::from("https://data.bmkg.go.id/en_autogempa.xml"),
      Url::EnGempaTerkini => String::from("https://data.bmkg.go.id/en_gempaterkini.xml"),
    };

    let data = get_data(url);
    data
  }

  pub fn to_json(
    data: Vec<Vec<(String, String)>>,
  ) -> Result<Vec<ValueJson>, Box<dyn std::error::Error>> {
    let mut vec = Vec::new();

    for i in 0..data.len() {
      let y = data[i].iter().fold(json!({}), |mut acc, (k, v)| {
        acc[k] = ValueJson::String(v.to_string());
        acc
      });
      vec.push(y)
    }

    Ok(vec)
  }
}

type Key = String;
type Value = String;

async fn get_xml(url: &String) -> Result<String, Box<dyn std::error::Error>> {
  let resp = reqwest::get(url).await?.text().await?;
  Ok(resp)
}

fn parsing_data(data: String) -> Vec<(Key, Value)> {
  let parser = EventReader::from_str(&data);
  let mut tags: Vec<String> = Vec::new();
  let mut res: Vec<(Key, Value)> = Vec::new();

  for e in parser {
    match e {
      Ok(XmlEvent::StartElement { name, .. }) => {
        // name.local_name
        tags.push(name.local_name);
      }
      Ok(XmlEvent::EndElement { .. }) => {
        tags.pop();
      }
      Ok(XmlEvent::Characters(car)) => {
        let tag = &tags[tags.len() - 1];
        res.push((tag.to_string(), car));
      }
      Err(e) => {
        println!("Error: {}", e);
        break;
      }
      _ => {}
    }
  }

  res
}

fn separate_data(data: Vec<(Key, Value)>) -> Vec<Vec<(Key, Value)>> {
  let start = data[0].0.clone();
  let mut final_r: Vec<Vec<(Key, Value)>> = Vec::new();

  for i in data {
    let last = match final_r.len() {
      0 => 0,
      _ => final_r.len() - 1,
    };

    if i.0 == start {
      let x: Vec<(Key, Value)> = [i].to_vec();
      final_r.push(x);
    } else {
      final_r[last].push(i);
    }
  }

  final_r
}

#[tokio::main]
async fn get_data(url: String) -> Result<Vec<Vec<(String, String)>>, Box<dyn std::error::Error>> {
  let xml = get_xml(&url).await;

  match xml {
    Ok(v) => {
      let res: Vec<(Key, Value)> = parsing_data(v);
      let final_r: Vec<Vec<(Key, Value)>> = separate_data(res);
      Ok(final_r)
    }

    Err(e) => {
      println!("error: {:?}", e);
      Err(e)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn parsing_data_test() {
    let data = String::from(
      "<Tanggal>30-Jul-20</Tanggal>
       <Jam>09:51:20 WIB</Jam>",
    );

    let result = [
      ("Tanggal".to_string(), "30-Jul-20".to_string()),
      ("Jam".to_string(), "09:51:20 WIB".to_string()),
    ];

    assert_eq!(parsing_data(data), result);
  }

  #[test]
  fn separate_data_test() {
    let data = [
      ("Tanggal".to_string(), "30-Jul-20".to_string()),
      ("Jam".to_string(), "09:51:20 WIB".to_string()),
      ("Tanggal".to_string(), "30-Jul-21".to_string()),
      ("Jam".to_string(), "05:51:20 WIB".to_string()),
    ]
    .to_vec();

    let result = [
      [
        ("Tanggal".to_string(), "30-Jul-20".to_string()),
        ("Jam".to_string(), "09:51:20 WIB".to_string()),
      ],
      [
        ("Tanggal".to_string(), "30-Jul-21".to_string()),
        ("Jam".to_string(), "05:51:20 WIB".to_string()),
      ],
    ];

    assert_eq!(separate_data(data), result);
  }
}
