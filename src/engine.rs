use crate::runner::{
    best_first_rule_runner::BestFirstRuleRunner, 
    chain_rule_runner::ChainRuleRunner,
    RuleRunner,
};
use crate::rule::{RuleResult, RuleContext, Rule};

/// The Engine provides convenient methods for rule execution
pub struct Engine;

impl Engine {
    /// Get a BestFirstRuleRunner instance
    pub fn best_first_runner() -> BestFirstRuleRunner {
        BestFirstRuleRunner
    }

    /// Get a ChainRuleRunner instance  
    pub fn chain_runner() -> ChainRuleRunner {
        ChainRuleRunner
    }

    /// Execute rules using the best-first strategy
    pub fn execute_best_first(
        context: &mut RuleContext, 
        rules: &mut [Box<dyn Rule>]
    ) -> RuleResult<()> {
        Self::best_first_runner().run(context, rules)
    }

    /// Execute rules using the chain strategy
    pub fn execute_chain(
        context: &mut RuleContext, 
        rules: &mut [Box<dyn Rule>]
    ) -> RuleResult<()> {
        Self::chain_runner().run(context, rules)
    }
}
