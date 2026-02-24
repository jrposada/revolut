use clap::{Parser, Subcommand};
use jrp_finance::cli::RevolutArgs;

#[derive(Parser)]
#[command(name = "jrp-finance")]
#[command(author = "JR Posada")]
#[command(version = "1.0.0")]
#[command(about = "Finance utilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Revolut-specific finance utilities
    Revolut(RevolutArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Revolut(args) => args.run(),
    }
}
