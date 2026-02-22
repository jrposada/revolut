use clap::{Parser, Subcommand};
use revolut::cli::{IncomeArgs, PlanArgs, SimulateArgs};

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
    /// Calculate capital needed for target passive income
    Income(IncomeArgs),
    /// Get plan breakpoints
    Plan(PlanArgs),
    /// Simulate compound interest for a plan
    Simulate(SimulateArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Income(args) => args.run(),
        Commands::Plan(args) => args.run(),
        Commands::Simulate(args) => args.run(),
    }
}
