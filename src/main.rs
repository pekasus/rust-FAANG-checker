#![allow(unused)]
#![feature(async_closure)]
use dotenv::dotenv;
use rayon::prelude::*;
use reqwest::{Client, Error};
use serde_json;
use std::{env, thread};
use std::thread::Thread;
use stocks::faang::{get_ticker, Ticker};
use tokio;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().expect(".env file not found");
    // get api_key from alphavantage.co and store it in `.env`
    let api_key = env::var("ALPHA").expect("$ALPHA is not set");
    //let client = Client::new();
    let stocks: Vec<&str> = vec!["META", "AMZN", "AAPL", "NFLX", "GOOG"];

    let mut vec_futures: Vec<_> = Vec::with_capacity(5);
    for idx in 0..5 {
        let stock = stocks[idx];
        let future = get_ticker(Client::new(), stock, api_key);
        vec_futures.push(future);
    }
    let results = join_all(vec_futures);
    // This was too dificult, hence we wrote the below steps.
    /*let results_before_await = stocks
        .par_iter()
        .map(async move |&t| get_ticker(Client::new(), t, api_key.clone()));
    */
    // let entries_before_async = stocks.iter().map(|&t| (t, api_key.clone())).collect::<Vec<_>>();

    // let results_before_await = entries_before_async
    //     .par_iter()
    //     .map(async move |(t, key)| get_ticker(Client::new(), t, key.clone()));

    // let results_after_await = results_before_await.map(async move |result| result.await);

    //results_after_await.for_each(|result| println!("{}", result));

    //let t = get_ticker(client, "GOOG", api_key).await     ?;

    //println!("{:?}", &t);

    Ok(())
}

// thread_pool = Vec<Thread>

// for idx in 0..5 {
//     get_ticker(stocks[idx]).await;
//     append thread to thread_pool
// }

// join thread_pool
