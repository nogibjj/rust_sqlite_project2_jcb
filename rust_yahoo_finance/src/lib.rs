/* A library that provides two functions:
- download financial data using yahoo_finance_api get_quote_range with 3 inputs: ticker, interval, period and return a DataFrame
- print summary statistics using polars
 */

use polars::prelude::*;
use rusqlite::{params, Connection, Result, ToSql};
use std::error::Error;
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
}

// create a  function to create findata.db using rusqlite
pub fn create_connection() -> Result<rusqlite::Connection, rusqlite::Error> {
    let conn = rusqlite::Connection::open("findata.db")?;
    Ok(conn)
}

// create a function to create a table in findata.db
pub fn create_table(table_name: &str, conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    let table = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            date TEXT, 
            open REAL, 
            high REAL, 
            low REAL, 
            close REAL, 
            adjclose REAL, 
            volume REAL
        )",
        table_name
    );
    conn.execute(table.as_str(), [])?;
    Ok(())
}

// create a function to insert data into the previous table that takes a dataframe, table_name and Connection as input
pub fn insert_data(
    df: &mut DataFrame,
    table_name: &str,
    conn: &Connection,
) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare(&format!("INSERT INTO {} (date, open, high, low, close, adjclose, volume) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", table_name))?;

    // Bing
    df.as_single_chunk_par();
    // create vector of iterators: one for each column
    let mut iters = df
        .columns(df.get_column_names())?
        .iter()
        .map(|s| s.iter())
        .collect::<Vec<_>>();

    for row in 0..df.height() {
        // create a tuple of parameters to pass to the statement. Set each parameter to 0
        let mut params: (
            Box<dyn ToSql>,
            Box<dyn ToSql>,
            Box<dyn ToSql>,
            Box<dyn ToSql>,
            Box<dyn ToSql>,
            Box<dyn ToSql>,
            Box<dyn ToSql>,
        ) = (
            Box::new(0),
            Box::new(0),
            Box::new(0),
            Box::new(0),
            Box::new(0),
            Box::new(0),
            Box::new(0),
        );
        // update the tuple with the values from the dataframe
        for (i, iter) in iters.iter_mut().enumerate() {
            let value = iter.next().expect("should have as many iterations as rows");
            // add the value to the params tuple
            match i {
                0 => {
                    params.0 = match value {
                        AnyValue::UInt64(val) => Box::new(val as i64),
                        AnyValue::Float64(val) => Box::new(val),
                        _ => panic!("Expected UInt64 or Float64"),
                    }
                }
                1 => {
                    params.1 = match value {
                        AnyValue::UInt64(val) => Box::new(val as i64),
                        AnyValue::Float64(val) => Box::new(val),
                        _ => panic!("Expected UInt64 or Float64"),
                    }
                }
                2 => {
                    params.2 = match value {
                        AnyValue::UInt64(val) => Box::new(val as i64),
                        AnyValue::Float64(val) => Box::new(val),
                        _ => panic!("Expected UInt64 or Float64"),
                    }
                }
                3 => {
                    params.3 = match value {
                        AnyValue::UInt64(val) => Box::new(val as i64),
                        AnyValue::Float64(val) => Box::new(val),
                        _ => panic!("Expected UInt64 or Float64"),
                    }
                }
                4 => {
                    params.4 = match value {
                        AnyValue::UInt64(val) => Box::new(val as i64),
                        AnyValue::Float64(val) => Box::new(val),
                        _ => panic!("Expected UInt64 or Float64"),
                    }
                }
                5 => {
                    params.5 = match value {
                        AnyValue::UInt64(val) => Box::new(val as i64),
                        AnyValue::Float64(val) => Box::new(val),
                        _ => panic!("Expected UInt64 or Float64"),
                    }
                }
                6 => {
                    params.6 = match value {
                        AnyValue::UInt64(val) => Box::new(val as i64),
                        AnyValue::Float64(val) => Box::new(val),
                        _ => panic!("Expected UInt64 or Float64"),
                    }
                }
                _ => (),
            };
        }
        // execute the statement with the tuple of parameters
        stmt.execute(params)?;
    }
    Ok(())
}
