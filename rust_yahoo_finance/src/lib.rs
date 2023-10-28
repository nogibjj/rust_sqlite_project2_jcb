/* A library that provides a structure with two methods:
- download financial data using yahoo_finance_api get_quote_range with 3 inputs: ticker, interval, period and return a DataFrame
- print summary statistics using polars
 */

use polars::prelude::*;
// use std::time::{Duration, UNIX_EPOCH};
// use tokio_test;
use yahoo_finance_api as yahoo;
use rusqlite::{params, Connection, Result};

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
}

// adding interaction with sqlite database

pub fn create_connection() -> Result<Connection> {
    let conn = Connection::open("findata.db")?;
    Ok(conn)
}

pub fn create_table(table_name: &str, conn: &Connection) -> Result<()> {
    let table = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            date TEXT,
            open FLOAT,
            high FLOAT,
            low FLOAT,
            close FLOAT,
            adjclose FLOAT,
            volume FLOAT
        )",
        table_name
    );
    conn.execute(table.as_str(), [])?;
    Ok(())
}

pub fn insert_to_table(table_name: &str, df: &DataFrame, conn: &Connection) -> Result<()> {
    let insert_query = format!(
        "INSERT INTO {} (date, open, high, low, close, adjclose, volume) VALUES (?, ?, ?, ?, ?, ?, ?)",
        table_name
    );
    let date_col = df.column("date").unwrap();
    let date_str_col = date_col
        .as_date()
        .unwrap()
        .to_string("%Y-%m-%d %H:%M:%S");
    let df = df
        .with_column(date_str_col)
        .rename(date_col.name(), "date");
    let open_col = df.column("open").unwrap();
    let high_col = df.column("high").unwrap();
    let low_col = df.column("low").unwrap();
    let close_col = df.column("close").unwrap();
    let adjclose_col = df.column("adjclose").unwrap();
    let volume_col = df.column("volume").unwrap();
    let rows = open_col
        .into_iter()
        .zip(high_col)
        .zip(low_col)
        .zip(close_col)
        .zip(adjclose_col)
        .zip(volume_col)
        .map(|((((open, high), low), close), adjclose), volume)| {
            params![
                date_str_col.as_ref(),
                open.as_float64().unwrap(),
                high.as_float64().unwrap(),
                low.as_float64().unwrap(),
                close.as_float64().unwrap(),
                adjclose.as_float64().unwrap(),
                volume.as_float64().unwrap(),
            ]
        });
    let mut stmt = conn.prepare(insert_query.as_str())?;
    for row in rows {
        stmt.execute(row)?;
    }
    Ok(())
}