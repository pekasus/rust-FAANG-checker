use dotenv::dotenv;
use reqwest::{Client, Error};
use serde_json;
use std::env;
use stocks::faang::{Ticker, get_ticker};
use tokio::main;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().expect(".env file not found");
    /// get api_key from alphavantage.co and store it in `.env`
    let api_key = env::var("ALPHA").expect("$ALPHA is not set");

    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=AAPL&apikey={}",
        api_key
    );
    let client = Client::new();
    let t = get_ticker(client, "GOOG", api_key).await?;
    println!("{:?}", &t);

    Ok(())
}
