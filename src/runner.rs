use crate::rule::{RuleContextWrapper, Wrapper};

pub(crate) mod best_first_rule_runner;
pub(crate) mod chain_rule_runner;

pub trait RuleRunner {
    type RuleType;
    fn run(&self, rule_context: RuleContextWrapper, rules: Vec<Wrapper<Self::RuleType>>);
}
