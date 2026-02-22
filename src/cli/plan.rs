use crate::infrastructure::{format_number, logger, table, ColumnDefinition, TableOptions};
use crate::service::calculate_breakpoints;
use clap::Args;

#[derive(Args, Debug)]
pub struct PlanArgs {}

impl PlanArgs {
    pub fn run(&self) {
        let breakpoints = calculate_breakpoints();

        if breakpoints.is_empty() {
            logger::warn("No matching plan found.");
            return;
        }

        let mut rows: Vec<Vec<String>> = Vec::new();

        for bp in &breakpoints {
            let ias_breakpoint = match bp.instant_access_savings {
                Some(v) => format_number(v),
                None => "N/A".to_string(),
            };

            let fcf_breakpoint = match bp.flexible_cash_funds {
                Some(v) => format_number(v),
                None => "N/A".to_string(),
            };

            rows.push(vec![
                bp.name.to_string(),
                if bp.billing == 0.0 {
                    "free".to_string()
                } else {
                    format!("{}€/yr", bp.billing)
                },
                ias_breakpoint,
                fcf_breakpoint,
            ]);
        }

        let options = TableOptions {
            title: Some("Plan Breakpoints".to_string()),
            columns: vec![
                ColumnDefinition::new("Plan", 10),
                ColumnDefinition::new("Billing", 12).align_right(),
                ColumnDefinition::new("IAS Brkpt (100k)", 16).align_right(),
                ColumnDefinition::new("FCF Brkpt (22k)", 16).align_right(),
            ],
            footer: None,
        };

        logger::log(&table(&rows, options));
    }
}
