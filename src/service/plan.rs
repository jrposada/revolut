use crate::data::{Plan, PLANS};

const TAX_WITHHOLDING: f64 = 0.19;

pub struct PlanBreakpoint {
    pub name: &'static str,
    pub billing: f64,
    pub instant_access_savings: Option<u64>,
    pub flexible_cash_funds: Option<u64>,
}

pub fn calculate_breakpoints() -> Vec<PlanBreakpoint> {
    let standard = &PLANS[0];

    PLANS
        .iter()
        .skip(1)
        .map(|plan| breakpoint(plan, standard))
        .collect()
}

fn breakpoint(plan: &Plan, baseline: &Plan) -> PlanBreakpoint {
    let ias_diff = plan.instant_access_savings - baseline.instant_access_savings;
    let fcf_diff = plan.flexible_cash_funds - baseline.flexible_cash_funds;
    let net_factor = 1.0 - TAX_WITHHOLDING;

    PlanBreakpoint {
        name: plan.name,
        billing: plan.billing,
        instant_access_savings: if ias_diff > 0.0 {
            Some((plan.billing / (ias_diff / 100.0 * net_factor)).ceil() as u64)
        } else {
            None
        },
        flexible_cash_funds: if fcf_diff > 0.0 {
            Some((plan.billing / (fcf_diff / 100.0 * net_factor)).ceil() as u64)
        } else {
            None
        },
    }
}
