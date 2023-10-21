/* A library that provides two functions:
- download financial data using yahoo_finance_api get_quote_range with 3 inputs: ticker, interval, period and return a DataFrame
- print summary statistics using polars
 */

use polars::prelude::*;
// use std::time::{Duration, UNIX_EPOCH};
use tokio_test;
use yahoo_finance_api as yahoo;

pub struct FinData {
    pub df: DataFrame,
}

impl FinData {
    pub fn new(ticker: &str, interval: &str, period: &str) -> Self {
        let provider = yahoo::YahooConnector::new();
        let response =
            tokio_test::block_on(provider.get_quote_range(ticker, interval, period)).unwrap();
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
        Self { df }
    }
    // print summary statistics for the dataframe similar to pandas describe()
    pub fn print_summary(&self) {
        println!("{}", self.df);
        println!("Summary Statistics:");
        println!("{}", self.df.describe(None).unwrap());
    }

    // pub fn print_summary(&self) {
    //     println!("{}", self.df);
    //     println!("Summary Statistics:");
    //     println!("{}", self.df.describe().unwrap());
    // }
}
