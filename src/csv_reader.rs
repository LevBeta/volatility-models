use std::error::Error;
use csv;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct Kline {
    #[serde(rename = "open", deserialize_with = "from_str")]
    open: f64,
    #[serde(rename = "high", deserialize_with = "from_str")]
    high: f64,
    #[serde(rename = "low", deserialize_with = "from_str")]
    low: f64,
    #[serde(rename = "close", deserialize_with = "from_str")]
    close: f64
}

pub fn read_csv(path: &str) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    // let headers = reader.headers()?;

    let mut open = Vec::new();
    let mut high = Vec::new();
    let mut low = Vec::new();
    let mut close = Vec::new();

    for result in reader.deserialize() {
        let record: Kline = result?;
    
        open.push(record.open);
        high.push(record.high);
        low.push(record.low);
        close.push(record.close);
        
    }

    Ok((open, high, low, close))
}


fn from_str<'de, D>(s: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(s)?;
    s.parse().map_err(serde::de::Error::custom)
}