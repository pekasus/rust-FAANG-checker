use reqwest::{Client, Error};
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

use dotenv::dotenv;
use serde_json::Value;
use std::env;
use std::process;

// #[derive(Deserialize, Debug)]
// struct Json {

//     Global Quote": {
//         "01. symbol": "AAPL",
//         "02. open": "174.4800",
//         "03. high": "176.2399",
//         "04. low": "172.1200",
//         "05. price": "172.9000",
//         "06. volume": "89418074",
//         "07. latest trading day": "2022-02-03",
//         "08. previous close": "175.8400",
//         "09. change": "-2.9400",
//         "10. change percent": "-1.6720%"
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().expect(".env file not found");
    let api_key = env::var("ALPHA").expect("$ALPHA is not set");
    println!("{:?}", api_key);

    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=AAPL&apikey={}",
        api_key
    );
    let client = Client::new();
    let response = client.get(url).send().await?;

    println!("Status: {}", &response.status());
    let json_text = response.text().await?;
    let json_ser_ok = serde_json::from_str(&json_text);
    if !json_ser_ok.is_ok() {
        panic!("Something wrong with Json deserialize.")
    }
    let json_ser: Value = json_ser_ok.unwrap();

    println!("{:?}", json_ser["Global Quote"]);
    // let result = &response.json::<Json>().await?;
    // println!("{:?}", &result);
    // println!("{:?}", &result.data);
    // println!("{:?}", &result.data[0]);

    Ok(())
}
