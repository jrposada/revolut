use crate::infrastructure::{format_number, logger, table, ColumnDefinition, TableOptions};
use crate::service::revolut::calculate_required_capital;
use clap::Args;

#[derive(Args, Debug)]
pub struct IncomeArgs {
    /// Target monthly passive income in EUR
    #[arg(short, long, default_value_t = 1000.0)]
    pub income: f64,

    /// Filter by plan name (e.g. standard, plus, premium, metal, ultra)
    #[arg(short, long)]
    pub plan: Option<String>,
}

impl IncomeArgs {
    pub fn run(&self) {
        let results = match calculate_required_capital(self.income, self.plan.as_deref()) {
            Ok(r) => r,
            Err(e) => {
                logger::error(&e);
                return;
            }
        };

        let mut rows: Vec<Vec<String>> = Vec::new();

        for r in &results {
            rows.push(vec![
                r.name.to_string(),
                if r.billing == 0.0 {
                    "free".to_string()
                } else {
                    format!("{}€/yr", r.billing)
                },
                format!("{:.2}%", r.ias_rate),
                format!("{}€", format_number(r.ias_capital.round() as u64)),
                format!("{:.2}%", r.fcf_rate),
                format!("{}€", format_number(r.fcf_capital.round() as u64)),
            ]);
        }

        let options = TableOptions {
            title: Some(format!(
                "Capital required for {}€/month passive income",
                format_number(self.income.round() as u64)
            )),
            columns: vec![
                ColumnDefinition::new("Plan", 10),
                ColumnDefinition::new("Billing", 12).align_right(),
                ColumnDefinition::new("IAS Rate", 10).align_right(),
                ColumnDefinition::new("IAS Capital", 14).align_right(),
                ColumnDefinition::new("FCF Rate", 10).align_right(),
                ColumnDefinition::new("FCF Capital", 14).align_right(),
            ],
            footer: Some(
                "Tax withholding: 19% | Simple annual interest (no reinvestment)".to_string(),
            ),
        };

        logger::log(&table(&rows, options));
    }
}
