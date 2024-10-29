use crate::rule::{best_first_rule::BestFirstRule, Rule, RuleContextWrapper, Wrapper};

use super::RuleRunner;

pub struct BestFirstRuleRunner;

impl RuleRunner for BestFirstRuleRunner {
    type RuleType = BestFirstRule;
    fn run(&self, rule_context: RuleContextWrapper, rules: Vec<Wrapper<Self::RuleType>>) {
        if !rules.is_empty() {
            for rule in rules {
                let mut rule_borrow = rule.borrow_mut();
                rule_borrow.set_rule_context(rule_context.clone());
                if !rule_borrow.fire() {
                    break;
                }
            }
        }
    }
}
