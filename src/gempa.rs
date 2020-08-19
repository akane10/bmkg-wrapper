use serde_json::{json, Value as ValueJson};
use xml::reader::{EventReader, XmlEvent};

pub enum Url {
  Autogempa,
  GempaTerkini,
  GempaDirasakan,
  LastTsunami,
  EnAutogempa,
  EnGempaTerkini,
}

type Key = String;
type Value = String;

async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
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
  match data.len() {
    0 => [].to_vec(),
    _ => {
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
  }
}

#[tokio::main]
pub async fn get_data(url: Url) -> Result<Vec<Vec<(String, String)>>, Box<dyn std::error::Error>> {
  let url = match url {
    Url::Autogempa => "https://data.bmkg.go.id/autogempa.xml",
    Url::GempaTerkini => "https://data.bmkg.go.id/gempaterkini.xml",
    Url::GempaDirasakan => "https://data.bmkg.go.id/gempadirasakan.xml",
    Url::LastTsunami => "https://data.bmkg.go.id/lasttsunami.xml",
    Url::EnAutogempa => "https://data.bmkg.go.id/en_autogempa.xml",
    Url::EnGempaTerkini => "https://data.bmkg.go.id/en_gempaterkini.xml",
  };
  let xml = fetch_data(url).await;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parsing_data_test() {
    let data = String::from(
      "<Tanggal>30-Jul-20</Tanggal>
       <Jam>09:51:20 WIB</Jam>",
    );

    let expected = [
      ("Tanggal".to_string(), "30-Jul-20".to_string()),
      ("Jam".to_string(), "09:51:20 WIB".to_string()),
    ];

    assert_eq!(parsing_data(data), expected);
  }

  #[test]
  fn parsing_data_empty_string_test() {
    let data = String::from("");
    let expected = [];

    assert_eq!(parsing_data(data), expected);
  }

  #[test]
  fn parsing_data_invalid_xml_test() {
    let data = String::from("<Tanggal>30-Jul-20");
    let expected = [];

    assert_eq!(parsing_data(data), expected);
  }

  #[test]
  fn parsing_data_without_tag_xml_test() {
    let data = String::from("30-Jul-20");
    let expected = [];

    assert_eq!(parsing_data(data), expected);
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

    let expected = [
      [
        ("Tanggal".to_string(), "30-Jul-20".to_string()),
        ("Jam".to_string(), "09:51:20 WIB".to_string()),
      ],
      [
        ("Tanggal".to_string(), "30-Jul-21".to_string()),
        ("Jam".to_string(), "05:51:20 WIB".to_string()),
      ],
    ];

    assert_eq!(separate_data(data), expected);
  }

  #[test]
  fn separate_data_empty_vec_test() {
    let data = [].to_vec();
    let expected: Vec<Vec<(Key, Value)>> = [].to_vec();

    assert_eq!(separate_data(data), expected);
  }

  #[test]
  fn to_json_test() {
    let vec = [
      ("Tanggal".to_string(), "30-Jul-20".to_string()),
      ("Jam".to_string(), "09:51:20 WIB".to_string()),
    ]
    .to_vec();

    let vec1 = [
      ("Tanggal".to_string(), "30-Jul-21".to_string()),
      ("Jam".to_string(), "05:51:20 WIB".to_string()),
    ]
    .to_vec();

    let i = serde_json::from_str(r#"{ "Tanggal": "30-Jul-20", "Jam": "09:51:20 WIB"}"#).unwrap();
    let ii = serde_json::from_str(r#"{ "Tanggal": "30-Jul-21", "Jam": "05:51:20 WIB"}"#).unwrap();

    let data = [vec, vec1].to_vec();
    let expected: Vec<ValueJson> = [i, ii].to_vec();
    assert_eq!(to_json(data).unwrap(), expected);
  }

  #[test]
  fn to_json_empty_test() {
    let data = [].to_vec();
    let expected: Vec<ValueJson> = [].to_vec();
    assert_eq!(to_json(data).unwrap(), expected);
  }
}
