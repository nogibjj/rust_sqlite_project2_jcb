/* Command-line interface for FinData */
use clap::Parser;
use yahoo_finance_demo::{insert_data, list_tables, FinData};

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
    #[clap(name = "list-tables")]
    ListTables,
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
            let mut fin_data = FinData::new(
                &summary_command.ticker,
                &summary_command.interval,
                &summary_command.period,
            );
            fin_data.print_summary();
            let conn = yahoo_finance_demo::create_connection().unwrap();
            let table_name = summary_command.ticker.to_lowercase();
            yahoo_finance_demo::create_table(&table_name, &conn).unwrap();
            insert_data(&mut fin_data.df, &table_name, &conn).unwrap();
        }
        // create new subcommand that uses list_tables() and then prints all tables in findata.db
        Some(Commands::ListTables) => {
            let conn = yahoo_finance_demo::create_connection().unwrap();
            let tables = list_tables(&conn).unwrap();
            println!("Tables in findata.db:");
            // use enumerate to print tables with index
            for (i, table) in tables.iter().enumerate() {
                println!("Table{}: {}", i + 1, table);
            }
        }

        None => {
            println!("No subcommand specified");
        }
    }
}
