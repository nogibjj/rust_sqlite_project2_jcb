/* Command-line interface for FinData */
use clap::Parser;
use yahoo_finance_demo::{FinData, insert_data};

// Command-line interface for FinData

#[derive(Parser)]
#[clap(
    author = "Javier Cervantes",
    version = "v1.0",
    about = "Command-line interface for FinData"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(name = "summary")]
    Summary(SummaryCommand),
}

#[derive(Parser)]
struct SummaryCommand {
    #[clap(short, long)]
    ticker: String,
    #[clap(short, long)]
    interval: String,
    #[clap(short, long)]
    period: String,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Summary(summary_command)) => {
            let fin_data = FinData::new(
                &summary_command.ticker,
                &summary_command.interval,
                &summary_command.period,
            );
            fin_data.print_summary();
            let conn = yahoo_finance_demo::create_connection().unwrap();
            let table_name = summary_command.ticker.to_lowercase();
            yahoo_finance_demo::create_table(&table_name, &conn).unwrap();
            insert_data(&fin_data.df, &table_name, &conn).unwrap();
        }
        None => {
            println!("No subcommand specified");
        }
    }
}