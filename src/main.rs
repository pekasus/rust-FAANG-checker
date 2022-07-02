use dotenv::dotenv;
use reqwest::{Client, Error};
use serde_json;
use std::env;
use stocks::faang::{get_ticker, Ticker};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().expect(".env file not found");
    // get api_key from alphavantage.co and store it in `.env`
    let api_key = env::var("ALPHA").expect("$ALPHA is not set");
    let client = Client::new();
    let stocks: Vec<&str> = 

    let t = get_ticker(client, "GOOG", api_key).await?;
    println!("{:?}", &t);

    Ok(())
}
