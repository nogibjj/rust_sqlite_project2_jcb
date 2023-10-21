[![Clippy](https://github.com/nogibjj/rust_mini_project7_JCB/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rust_mini_project7_JCB/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/rust_mini_project7_JCB/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rust_mini_project7_JCB/actions/workflows/tests.yml)

# IDS 706 Week 7 Mini - project


## Setting up a command-line tool using Rust to print out financial data using the Yahoo Finance API

### The project in Rust

https://github.com/nogibjj/rust_mini_project7_JCB

In this project I've set up a command-line tool to print out summary statistics on any financial instrument found on yahoo_finance_api.

I've created a Rust structure called FinData that takes 3 arguments:

- ticker | the financial instrument's ticker (as found in yahoo finance) ex: AAPL or USDMXN=X
- interval | the tick interval for the data. This defines the OHLC bar size ex: 1d, 60m, 1m
- period | the time range for which you wish to download the data ex: 1mo, 6mo, 1y

The structure then has a method print_summary to print out each OHLC bar as well as summary statistics. 

### Command-line tool

Use `main.rs` to call the handle CLI and `lib.rs` to handle logic and import `clap` in `Cargo.toml` as shown in this project.

### Implementation

This project is ideal for quick implementation using GitHub Codespaces. 

1) Navigate to https://github.com/nogibjj/rust_mini_project7_JCB
2) Create a new Codespace
3) `cd rust_yahoo_finance` to cd into the Rust yahoo finance project
4) `cargo run -- summary --ticker AAPL --interval 1d --period 1mo` to print out summary statistics for AAPL daily data for the last 1 month. Please feel free to play around with these inputs and report back any issues

![output](https://github.com/nogibjj/rust_mini_project7_JCB/assets/33461065/3afeb207-a3ce-4911-939e-a3911f3f8c90)
