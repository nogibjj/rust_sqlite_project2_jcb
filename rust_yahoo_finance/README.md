[![Clippy](https://github.com/nogibjj/rust_sqlite_project2_jcb/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rust_sqlite_project2_jcb/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/rust_sqlite_project2_jcb/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rust_sqlite_project2_jcb/actions/workflows/tests.yml)

# IDS 706 Individual Project 2


## Rust CLI Binary with SQLite


### The project in Rust

https://github.com/nogibjj/rust_sqlite_project2_jcb

### Yahoo finance API & summary statistics

In this project I've set up a command-line tool to print out summary statistics on any financial instrument found on yahoo_finance_api and perform various operations on an SQLite database.

I've created a Rust structure called FinData that takes 3 arguments:

- ticker | the financial instrument's ticker (as found in yahoo finance) ex: AAPL or USDMXN=X
- interval | the tick interval for the data. This defines the OHLC bar size ex: 1d, 60m, 1m
- period | the time range for which you wish to download the data ex: 1mo, 6mo, 1y

The structure then has a method print_summary to print out each OHLC bar as well as summary statistics for the given inputs. 

### SQLite database

The project additionally allows the following CRUD operations on an SQLite database:

- CREATE: creat_table | creates a table in the database with the given table_name
- READ: 
    - list_tables | lists all tables in the database
    - ** update!! add read_table to read the data from the table with the given table_name
- UPDATE: insert_data | inserts the data from the FinData structure into the table with the given table_name
- DELETE: ** update!! add delete interface to delete data from the database

### Command-line tool

Use `main.rs` to call the handle CLI and `lib.rs` to handle logic and import `clap` in `Cargo.toml` as shown in this project.

** update system architecture **

### Implementation

This project is ideal for quick implementation using GitHub Codespaces. 

1) Navigate to https://github.com/nogibjj/rust_mini_project7_JCB
2) Create a new Codespace
3) `cd rust_yahoo_finance` to cd into the Rust yahoo finance project
4) `cargo run -- summary --ticker AAPL --interval 1d --period 1mo` to print out summary statistics for AAPL daily data for the last 1 month. Please feel free to play around with these inputs and report back any issues
5) `cargo run -- list-tables` to list all tables in the database

** .exe binary **

** add some screenshots of the output **
