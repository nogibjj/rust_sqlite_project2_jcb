use polars::prelude::*;
// use std::time::{Duration, UNIX_EPOCH};
use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range("AAPL", "1d", "1mo")).unwrap();
    let quotes = response.quotes().unwrap();
    // Create a DataFrame from the quotes data
    let df = DataFrame::new(vec![
        Series::new(
            "date",
            &quotes.iter().map(|q| q.timestamp).collect::<Vec<_>>(),
        ),
        Series::new("open", &quotes.iter().map(|q| q.open).collect::<Vec<_>>()),
        Series::new("high", &quotes.iter().map(|q| q.high).collect::<Vec<_>>()),
        Series::new("low", &quotes.iter().map(|q| q.low).collect::<Vec<_>>()),
        Series::new("close", &quotes.iter().map(|q| q.close).collect::<Vec<_>>()),
        Series::new(
            "adjclose",
            &quotes.iter().map(|q| q.adjclose).collect::<Vec<_>>(),
        ),
        Series::new(
            "volume",
            &quotes.iter().map(|q| q.volume).collect::<Vec<_>>(),
        ),
    ])
    .unwrap();

    // Print the DataFrame
    println!("{}", df);
}
