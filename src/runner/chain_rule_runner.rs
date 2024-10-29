use crate::rule::{chain_rule::ChainRule, Rule, RuleContextWrapper, Wrapper};

use super::RuleRunner;

pub struct ChainRuleRunner;

impl RuleRunner for ChainRuleRunner {
    type RuleType = ChainRule;
    fn run(&self, rule_context: RuleContextWrapper, rules: Vec<Wrapper<Self::RuleType>>) {
        if rules.len() <= 1 {
            if let Some(rule) = rules.first() {
                let mut rule = rule.borrow_mut();
                rule.set_rule_context(rule_context);
                rule.fire();
            }
        } else {
            panic!("ChainRuleRunner does not support sibling rules, only child rules.");
        }
    }
}
