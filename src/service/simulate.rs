use crate::data::PLANS;

const TAX_WITHHOLDING: f64 = 0.19;

pub struct SimulationResult {
    pub account_type: &'static str,
    pub annual_rate: f64,
    pub initial_capital: f64,
    pub gross_interest: f64,
    pub net_interest: f64,
    pub final_balance: f64,
    pub billing: f64,
}

pub fn simulate_compound_interest(
    plan_name: &str,
    capital: f64,
) -> Result<Vec<SimulationResult>, String> {
    let plan = PLANS
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(plan_name))
        .ok_or_else(|| format!("Plan '{}' not found", plan_name))?;

    let results = vec![
        simulate_account("IAS", plan.instant_access_savings, capital, plan.billing),
        simulate_account("FCF", plan.flexible_cash_funds, capital, plan.billing),
    ];

    Ok(results)
}

/// Calculate net interest from daily compounding over 365 days with tax withholding.
pub fn calculate_net_interest(annual_rate: f64, capital: f64) -> f64 {
    let daily_rate = annual_rate / 100.0 / 365.0;
    let mut balance = capital;
    let mut net_interest = 0.0;
    for _ in 0..365 {
        let daily_interest = balance * daily_rate;
        let daily_net = daily_interest * (1.0 - TAX_WITHHOLDING);
        net_interest += daily_net;
        balance += daily_net;
    }
    net_interest
}

fn simulate_account(
    account_type: &'static str,
    annual_rate: f64,
    capital: f64,
    billing: f64,
) -> SimulationResult {
    let daily_rate = annual_rate / 100.0 / 365.0;
    let mut balance = capital;
    let mut gross_interest = 0.0;
    let mut net_interest = 0.0;
    for _ in 0..365 {
        let daily_interest = balance * daily_rate;
        let daily_net = daily_interest * (1.0 - TAX_WITHHOLDING);
        gross_interest += daily_interest;
        net_interest += daily_net;
        balance += daily_net;
    }
    let final_balance = balance;

    SimulationResult {
        account_type,
        annual_rate,
        initial_capital: capital,
        gross_interest,
        net_interest,
        final_balance,
        billing,
    }
}
