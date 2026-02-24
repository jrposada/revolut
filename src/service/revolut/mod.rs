pub mod income;
pub mod plan;

pub use income::{calculate_required_capital, IncomeResult};
pub use plan::{calculate_breakpoints, recommend_plan, PlanBreakpoint, PlanRecommendation};
