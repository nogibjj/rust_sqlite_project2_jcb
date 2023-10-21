/* Command-line interface for FinData */
use clap::Parser;

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
            let fin_data = yahoo_finance_demo::FinData::new(
                &summary_command.ticker,
                &summary_command.interval,
                &summary_command.period,
            );
            fin_data.print_summary();
        }
        None => {
            println!("No subcommand specified");
        }
    }
}
