use crate::infrastructure::{format_number, logger, table, ColumnDefinition, TableOptions};
use crate::service::revolut::{calculate_breakpoints, recommend_plan};
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct PlanArgs {
    #[command(subcommand)]
    command: PlanCommand,
}

#[derive(Subcommand, Debug)]
enum PlanCommand {
    /// Show plan breakpoints
    View,
    /// Recommend the best plan for given capitals
    Recommend(RecommendArgs),
}

#[derive(Args, Debug)]
struct RecommendArgs {
    /// Capital in Instant Access Savings account
    #[arg(long)]
    ias: f64,
    /// Capital in Flexible Cash Funds account
    #[arg(long)]
    fcf: f64,
}

impl PlanArgs {
    pub fn run(&self) {
        match &self.command {
            PlanCommand::View => run_view(),
            PlanCommand::Recommend(args) => run_recommend(args),
        }
    }
}

fn run_view() {
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

fn run_recommend(args: &RecommendArgs) {
    let recommendations = recommend_plan(args.ias, args.fcf);

    if recommendations.is_empty() {
        logger::warn("No plans available.");
        return;
    }

    let rows: Vec<Vec<String>> = recommendations
        .iter()
        .map(|r| {
            vec![
                r.name.to_string(),
                if r.billing == 0.0 {
                    "free".to_string()
                } else {
                    format!("{:.2}€/yr", r.billing)
                },
                format!("{:.2}€", r.ias_net_interest),
                format!("{:.2}€", r.fcf_net_interest),
                format!("{:.2}€", r.net_profit),
            ]
        })
        .collect();

    let best = recommendations.first().unwrap();

    let options = TableOptions {
        title: Some("Plan Recommendation".to_string()),
        columns: vec![
            ColumnDefinition::new("Plan", 10),
            ColumnDefinition::new("Billing", 12).align_right(),
            ColumnDefinition::new("IAS Interest", 14).align_right(),
            ColumnDefinition::new("FCF Interest", 14).align_right(),
            ColumnDefinition::new("Net Profit", 14).align_right(),
        ],
        footer: Some(format!("Recommended: {}", best.name)),
    };

    logger::log(&table(&rows, options));
}
