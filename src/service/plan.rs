use crate::data::{Plan, PLANS};
use crate::infrastructure::TAX_WITHHOLDING;
use crate::service::simulate::calculate_net_interest;

pub struct PlanBreakpoint {
    pub name: &'static str,
    pub billing: f64,
    pub instant_access_savings: Option<u64>,
    pub flexible_cash_funds: Option<u64>,
}

pub fn calculate_breakpoints() -> Vec<PlanBreakpoint> {
    PLANS
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, plan)| {
            let cheaper_plans = &PLANS[..i];
            breakpoint(plan, cheaper_plans)
        })
        .collect()
}

fn breakpoint(plan: &Plan, cheaper_plans: &[Plan]) -> PlanBreakpoint {
    let net_factor = 1.0 - TAX_WITHHOLDING;

    let ias = account_breakpoint(
        plan.billing,
        plan.instant_access_savings,
        cheaper_plans
            .iter()
            .map(|p| (p.billing, p.instant_access_savings)),
        net_factor,
    );

    let fcf = account_breakpoint(
        plan.billing,
        plan.flexible_cash_funds,
        cheaper_plans
            .iter()
            .map(|p| (p.billing, p.flexible_cash_funds)),
        net_factor,
    );

    PlanBreakpoint {
        name: plan.name,
        billing: plan.billing,
        instant_access_savings: ias,
        flexible_cash_funds: fcf,
    }
}

fn account_breakpoint(
    billing: f64,
    rate: f64,
    cheaper: impl Iterator<Item = (f64, f64)>,
    net_factor: f64,
) -> Option<u64> {
    let mut max_bp: Option<f64> = Some(0.0);

    for (cheaper_billing, cheaper_rate) in cheaper {
        let rate_diff = rate - cheaper_rate;
        if rate_diff <= 0.0 {
            return None;
        }
        let billing_diff = billing - cheaper_billing;
        let bp = billing_diff / (rate_diff / 100.0 * net_factor);
        max_bp = Some(max_bp.unwrap().max(bp));
    }

    max_bp.filter(|&v| v > 0.0).map(|v| v.ceil() as u64)
}

pub struct PlanRecommendation {
    pub name: &'static str,
    pub billing: f64,
    pub ias_net_interest: f64,
    pub fcf_net_interest: f64,
    pub net_profit: f64,
}

pub fn recommend_plan(ias_capital: f64, fcf_capital: f64) -> Vec<PlanRecommendation> {
    let mut recommendations: Vec<PlanRecommendation> = PLANS
        .iter()
        .map(|plan| {
            let ias_net_interest = calculate_net_interest(plan.instant_access_savings, ias_capital);
            let fcf_net_interest = calculate_net_interest(plan.flexible_cash_funds, fcf_capital);
            let net_profit = ias_net_interest + fcf_net_interest - plan.billing;

            PlanRecommendation {
                name: plan.name,
                billing: plan.billing,
                ias_net_interest,
                fcf_net_interest,
                net_profit,
            }
        })
        .collect();

    recommendations.sort_by(|a, b| b.net_profit.partial_cmp(&a.net_profit).unwrap());
    recommendations
}
