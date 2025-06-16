use crate::rule::{Rule, RuleResult, RuleContext};

use super::RuleRunner;

/// ChainRuleRunner executes rules in sequence
pub struct ChainRuleRunner;

impl RuleRunner for ChainRuleRunner {
    fn run(&self, context: &mut RuleContext, rules: &mut [Box<dyn Rule>]) -> RuleResult<()> {
        // Chain rules execute sequentially
        for rule in rules {
            rule.fire(context)?;
        }
        Ok(())
    }
}