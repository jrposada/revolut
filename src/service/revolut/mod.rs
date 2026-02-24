pub mod income;
pub mod plan;
pub mod simulate;

pub use income::{calculate_required_capital, IncomeResult};
pub use plan::{calculate_breakpoints, recommend_plan, PlanBreakpoint, PlanRecommendation};
pub use simulate::{calculate_net_interest, simulate_compound_interest};
