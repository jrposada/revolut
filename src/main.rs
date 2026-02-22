use clap::{Parser, Subcommand};
use revolut::cli::PlanArgs;

#[derive(Parser)]
#[command(name = "revolut")]
#[command(author = "JR Posada")]
#[command(version = "1.0.0")]
#[command(about = "Revolut utilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get plan breakpoints
    Plan(PlanArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Plan(args) => args.run(),
    }
}
