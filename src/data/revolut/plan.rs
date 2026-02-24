pub struct Plan {
    pub name: &'static str,
    pub billing: f64,
    pub instant_access_savings: f64,
    pub flexible_cash_funds: f64,
}

pub static PLANS: &[Plan] = &[
    Plan {
        name: "standard",
        billing: 0.0,
        instant_access_savings: 1.25,
        flexible_cash_funds: 1.2,
    },
    Plan {
        name: "plus",
        billing: 39.99,
        instant_access_savings: 1.25,
        flexible_cash_funds: 1.35,
    },
    Plan {
        name: "premium",
        billing: 90.0,
        instant_access_savings: 1.51,
        flexible_cash_funds: 1.8,
    },
    Plan {
        name: "metal",
        billing: 155.0,
        instant_access_savings: 2.02,
        flexible_cash_funds: 1.95,
    },
    Plan {
        name: "ultra",
        billing: 540.0,
        instant_access_savings: 2.27,
        flexible_cash_funds: 2.05,
    },
];
