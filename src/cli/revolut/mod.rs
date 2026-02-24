pub mod income;
pub mod plan;
pub mod simulate;

use clap::{Args, Subcommand};
use income::IncomeArgs;
use plan::PlanArgs;
use simulate::SimulateArgs;

#[derive(Args, Debug)]
pub struct RevolutArgs {
    #[command(subcommand)]
    command: RevolutCommands,
}

#[derive(Subcommand, Debug)]
enum RevolutCommands {
    /// Calculate capital needed for target passive income
    Income(IncomeArgs),
    /// Get plan breakpoints
    Plan(PlanArgs),
    /// Simulate compound interest for a plan
    Simulate(SimulateArgs),
}

impl RevolutArgs {
    pub fn run(&self) {
        match &self.command {
            RevolutCommands::Income(args) => args.run(),
            RevolutCommands::Plan(args) => args.run(),
            RevolutCommands::Simulate(args) => args.run(),
        }
    }
}
