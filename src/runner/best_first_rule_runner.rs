use crate::rule::{Rule, RuleResult, RuleContext};

use super::RuleRunner;

/// BestFirstRuleRunner executes the first rule that evaluates to true
pub struct BestFirstRuleRunner;

impl RuleRunner for BestFirstRuleRunner {
    fn run(&self, context: &mut RuleContext, rules: &mut [Box<dyn Rule>]) -> RuleResult<()> {
        // Execute the first rule that evaluates to true
        for rule in rules {
            if rule.evaluate(context)? {
                rule.fire(context)?;
                break; // Only execute the first matching rule
            }
        }
        Ok(())
    }
}