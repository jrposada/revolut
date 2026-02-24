use crate::data::revolut::PLANS;
use crate::infrastructure::{
    format_number, format_number_signed, logger, table, ColumnDefinition, TableOptions,
};
use crate::service::simulate::simulate_account;
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
        let plan = match PLANS
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(&self.plan))
        {
            Some(p) => p,
            None => {
                logger::error(&format!("Plan '{}' not found", self.plan));
                return;
            }
        };

        let results = vec![
            simulate_account("IAS", plan.instant_access_savings, self.capital),
            simulate_account("FCF", plan.flexible_cash_funds, self.capital),
        ];

        let mut rows: Vec<Vec<String>> = Vec::new();

        for r in &results {
            let net_profit = r.final_balance - r.initial_capital - plan.billing;
            rows.push(vec![
                r.label.to_string(),
                format!("{}%", r.annual_rate),
                format!("{}€", format_number(r.initial_capital.round() as u64)),
                format!("{}€", format_number(r.gross_interest.round() as u64)),
                format!("{}€", format_number(r.net_interest.round() as u64)),
                format!("{}€", format_number(r.final_balance.round() as u64)),
                format!("{}€", format_number_signed(net_profit.round() as i64)),
            ]);
        }

        let title = if plan.billing == 0.0 {
            format!("Simulate: {} (free)", self.plan)
        } else {
            format!("Simulate: {} ({}€/yr)", self.plan, plan.billing)
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
