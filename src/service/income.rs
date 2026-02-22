use crate::{data::PLANS, infrastructure::TAX_WITHHOLDING};

const NET_FACTOR: f64 = 1.0 - TAX_WITHHOLDING;

pub struct IncomeResult {
    pub name: &'static str,
    pub billing: f64,
    pub ias_rate: f64,
    pub fcf_rate: f64,
    pub ias_capital: f64,
    pub fcf_capital: f64,
    pub annual_target: f64,
    pub monthly_target: f64,
}

pub fn calculate_required_capital(
    monthly_income: f64,
    plan_filter: Option<&str>,
) -> Result<Vec<IncomeResult>, String> {
    if monthly_income <= 0.0 {
        return Err("Monthly income must be a positive number.".to_string());
    }

    let annual_target = monthly_income * 12.0;

    let plans: Vec<_> = match plan_filter {
        Some(filter) => {
            let matched: Vec<_> = PLANS
                .iter()
                .filter(|p| p.name.eq_ignore_ascii_case(filter))
                .collect();
            if matched.is_empty() {
                return Err(format!("Plan '{}' not found.", filter));
            }
            matched
        }
        None => PLANS.iter().collect(),
    };

    let results = plans
        .into_iter()
        .map(|plan| {
            let ias_capital =
                required_capital(annual_target, plan.billing, plan.instant_access_savings);
            let fcf_capital =
                required_capital(annual_target, plan.billing, plan.flexible_cash_funds);

            IncomeResult {
                name: plan.name,
                billing: plan.billing,
                ias_rate: plan.instant_access_savings,
                fcf_rate: plan.flexible_cash_funds,
                ias_capital,
                fcf_capital,
                annual_target,
                monthly_target: monthly_income,
            }
        })
        .collect();

    Ok(results)
}

fn required_capital(annual_target: f64, billing: f64, annual_rate: f64) -> f64 {
    (annual_target + billing) / (annual_rate / 100.0 * NET_FACTOR)
}
