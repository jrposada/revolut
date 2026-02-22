use crate::infrastructure::{format_number, logger, table, ColumnDefinition, TableOptions};
use crate::service::simulate_compound_interest;
use clap::Args;

#[derive(Args, Debug)]
pub struct SimulateArgs {
    /// Plan name (e.g. standard, plus, premium, metal, ultra)
    #[arg(short, long)]
    pub plan: String,

    /// Capital amount to simulate
    #[arg(short, long)]
    pub capital: f64,
}

impl SimulateArgs {
    pub fn run(&self) {
        let results = match simulate_compound_interest(&self.plan, self.capital) {
            Ok(r) => r,
            Err(e) => {
                logger::error(&e);
                return;
            }
        };

        let mut rows: Vec<Vec<String>> = Vec::new();

        for r in &results {
            let net_profit = r.final_balance - r.initial_capital - r.billing;
            rows.push(vec![
                r.account_type.to_string(),
                format!("{}%", r.annual_rate),
                format!("{}€", format_number(r.initial_capital.round() as u64)),
                format!("{}€", format_number(r.gross_interest.round() as u64)),
                format!("{}€", format_number(r.net_interest.round() as u64)),
                format!("{}€", format_number(r.final_balance.round() as u64)),
                format!("{}€", format_number(net_profit.round() as u64)),
            ]);
        }

        let billing = results.first().map(|r| r.billing).unwrap_or(0.0);
        let title = if billing == 0.0 {
            format!("Simulate: {} (free)", self.plan)
        } else {
            format!("Simulate: {} ({}€/yr)", self.plan, billing)
        };

        let options = TableOptions {
            title: Some(title),
            columns: vec![
                ColumnDefinition::new("Account", 10),
                ColumnDefinition::new("Rate", 8).align_right(),
                ColumnDefinition::new("Capital", 12).align_right(),
                ColumnDefinition::new("Gross Interest", 14).align_right(),
                ColumnDefinition::new("Net Interest", 14).align_right(),
                ColumnDefinition::new("Final Balance", 14).align_right(),
                ColumnDefinition::new("Net Profit", 14).align_right(),
            ],
            footer: None,
        };

        logger::log(&table(&rows, options));
    }
}
