#![allow(unused)]
#![feature(async_closure)]
use dotenv::dotenv;
use rayon::prelude::*;
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
    //let client = Client::new();
    let stocks: Vec<&str> = vec!["META", "AMZN", "AAPL", "NFLX", "GOOG"];
    
    // This was too dificult, hence we wrote the below steps.
    /*let results_before_await = stocks
        .par_iter()
        .map(async move |&t| get_ticker(Client::new(), t, api_key.clone()));
    */
    let entries_before_async = stocks.iter().map(|&t| (t, api_key.clone())).collect::<Vec<_>>();

    let results_before_await = entries_before_async
        .par_iter()
        .map(async move |(t, key)| get_ticker(Client::new(), t, key.clone()));

    let results_after_await = results_before_await.map(async move |result| result.await);

    //results_after_await.for_each(|result| println!("{}", result));

    //let t = get_ticker(client, "GOOG", api_key).await     ?;

    //println!("{:?}", &t);

    Ok(())
}
