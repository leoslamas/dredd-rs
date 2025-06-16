use crate::rule::{RuleResult, RuleContext, Rule};

pub(crate) mod best_first_rule_runner;
pub(crate) mod chain_rule_runner;

/// Trait for rule execution strategies
pub trait RuleRunner {
    fn run(&self, context: &mut RuleContext, rules: &mut [Box<dyn Rule>]) -> RuleResult<()>;
}
