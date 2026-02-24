use crate::infrastructure::TAX_WITHHOLDING;

pub struct SimulationResult {
    pub label: &'static str,
    pub annual_rate: f64,
    pub initial_capital: f64,
    pub gross_interest: f64,
    pub net_interest: f64,
    pub final_balance: f64,
}

pub fn simulate_account(
    label: &'static str,
    annual_rate: f64,
    capital: f64,
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
        label,
        annual_rate,
        initial_capital: capital,
        gross_interest,
        net_interest,
        final_balance,
    }
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
