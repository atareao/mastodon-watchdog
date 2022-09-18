mod twitter;

use dotenv::dotenv;
use std::{thread, time, env};
use tokio;
use crate::twitter::Twitter;

#[tokio::main]
async fn main() {
    dotenv().expect("Not found environment");
    let sleep_time_in_seconds = env::var("SLEEP_TIME")
        .expect("Not found SLEEP_TIME")
        .parse::<u64>()
        .unwrap();
    let consumer_key = env::var("TW_CONSUMER_KEY").expect("Not foun consumer key");
    let consumer_secret = env::var("TW_CONSUMER_SECRET").expect("Not foun consumer secret");
    let access_token = env::var("TW_ACCESS_TOKEN").expect("Not found access token");
    let access_token_secret = env::var("TW_ACCESS_TOKEN_SECRET").expect("Not found access token secret");
    let sleep_time = time::Duration::from_secs(sleep_time_in_seconds);
    let twitter = Twitter::new(&consumer_key, &consumer_secret, &access_token, &access_token_secret);
    twitter.tweet("Hi from rust!!").await;
    loop {
        thread::sleep(sleep_time);
        println!("Esto es una prueba");
    }
}
