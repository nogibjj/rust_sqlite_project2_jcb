/* A library that provides two functions:
- download financial data using yahoo_finance_api get_quote_range with 3 inputs: ticker, interval, period and return a DataFrame
- print summary statistics using polars
 */

 use polars::prelude::*;
 use yahoo_finance_api as yahoo;
 use rusqlite::{Result, Connection, params};
 use std::error::Error;

 
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
 pub fn insert_data(df: &DataFrame, table_name: &str, conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare(&format!("INSERT INTO {} (date, open, high, low, close, adjclose, volume) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", table_name))?;

    // Iterate over DataFrame rows
    for row in df.iter() {
        // Insert the data into the database
        stmt.execute(params![
            match row.get(0).unwrap() {
                AnyValue::UInt64(val) => val as i64,
                _ => panic!("Expected UInt64"),
            },
            match row.get(1).unwrap() {
                AnyValue::Float64(val) => val,
                _ => panic!("Expected Float64"),
            },
            match row.get(2).unwrap() {
                AnyValue::Float64(val) => val,
                _ => panic!("Expected Float64"),
            },
            match row.get(3).unwrap() {
                AnyValue::Float64(val) => val,
                _ => panic!("Expected Float64"),
            },
            match row.get(4).unwrap() {
                AnyValue::Float64(val) => val,
                _ => panic!("Expected Float64"),
            },
            match row.get(5).unwrap() {
                AnyValue::Float64(val) => val,
                _ => panic!("Expected Float64"),
            },
            match row.get(6).unwrap() {
                AnyValue::UInt64(val) => val as i64,
                _ => panic!("Expected UInt64"),
            },
        ])?;
    }

    Ok(())
}