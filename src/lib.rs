pub mod gempa;

use std::borrow::Borrow;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Url {
    Autogempa,
    GempaTerkini,
    GempaDirasakan,
}

impl Url {
    fn to_str(&self) -> &str {
        match self {
            Url::Autogempa => "https://data.bmkg.go.id/DataMKG/TEWS/autogempa.xml",
            Url::GempaTerkini => "https://data.bmkg.go.id/DataMKG/TEWS/gempaterkini.xml",
            Url::GempaDirasakan => "https://data.bmkg.go.id/DataMKG/TEWS/gempadirasakan.xml",
        }
    }
    fn from_str<T: Borrow<str>>(s: T) -> Option<Url> {
        match s.borrow().to_lowercase().as_ref() {
            "autogempa" => Some(Url::Autogempa),
            "gempaterkini" => Some(Url::GempaTerkini),
            "gempadirasakan" => Some(Url::GempaDirasakan),
            _ => None,
        }
    }
}

async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn autogempa_get_data() {
        let data = gempa::autogempa::get_data().await.unwrap();

        assert!(data.tanggal.is_some());
    }

    #[tokio::test]
    async fn gempaterkini_get_data() {
        let data = gempa::gempaterkini::get_data().await.unwrap();

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
