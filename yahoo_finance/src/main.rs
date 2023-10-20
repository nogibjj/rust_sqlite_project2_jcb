use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use tokio_test;

fn main() {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range("AAPL", "1d", "1mo")).unwrap();
    let quotes = response.quotes().unwrap();
    println!("Apple's quotes of the last month: {:?}", quotes);
}