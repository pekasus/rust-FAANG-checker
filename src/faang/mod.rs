use chrono::NaiveDate;
use serde;
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;

fn float_from_percent<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;
    s.pop();
    Ok(s.parse::<f64>().unwrap())
}

fn float_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.parse::<f64>().unwrap())
}

fn int_from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.parse::<i32>().unwrap())
}

fn date_from_str<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let naivedate = NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap();
    Ok(naivedate)
}

#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(rename = "02. open", deserialize_with = "float_from_str")]
    open: f64,
    #[serde(rename = "03. high", deserialize_with = "float_from_str")]
    high: f64,
    #[serde(rename = "04. low", deserialize_with = "float_from_str")]
    low: f64,
    #[serde(rename = "05. price", deserialize_with = "float_from_str")]
    price: f64,
    #[serde(rename = "06. volume", deserialize_with = "int_from_str")]
    volume: i32,
    #[serde(rename = "07. latest trading day", deserialize_with = "date_from_str")]
    latest_day: NaiveDate,
    #[serde(rename = "08. previous close", deserialize_with = "float_from_str")]
    previous_close: f64,
    #[serde(rename = "09. change", deserialize_with = "float_from_str")]
    change: f64,
    #[serde(rename = "10. change percent", deserialize_with = "float_from_percent")]
    change_percent: f64,
}
