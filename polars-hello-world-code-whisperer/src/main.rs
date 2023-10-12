/*
Polars hello world script that uses AWS Code Catalyst and Code Whisperer
*/
use polars::prelude::*;
use std::fs::File;

pub fn calculate() -> Result<DataFrame, PolarsError> {
    // Open the CSV file
    let file = File::open("data/iris.csv")?;

    // Read the CSV data using CsvReader
    let df = CsvReader::new(file)
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;
    Ok(df)
}

fn main() {
    let df = calculate().unwrap();
    println!("{}", df);
}
