use crate::runner::{
    best_first_rule_runner::BestFirstRuleRunner, chain_rule_runner::ChainRuleRunner,
};

/// The `Engine` struct provides methods to create instances of different rule runners.
///
/// # Methods
///
/// - `best_first_runner`: Creates a new instance of `BestFirstRuleRunner`.
/// - `chain_runner`: Creates a new instance of `ChainRuleRunner`.
///
pub struct Engine;

impl Engine {
    /// Creates a new instance of `BestFirstRuleRunner`.
    ///
    /// # Returns
    ///
    /// A `BestFirstRuleRunner` instance.
    pub fn best_first_runner() -> BestFirstRuleRunner {
        BestFirstRuleRunner
    }

    /// Creates a new instance of `ChainRuleRunner`.
    ///
    /// # Returns
    ///
    /// A `ChainRuleRunner` instance.
    pub fn chain_runner() -> ChainRuleRunner {
        ChainRuleRunner
    }
}
