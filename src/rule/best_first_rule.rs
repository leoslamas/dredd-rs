use crate::{engine::Engine, runner::RuleRunner as _};

use super::{wrap, Rule, RuleCallback, RuleChildren, RuleContextWrapper, Wrapper};

/// Represents a best first rule in the rule evaluation system.
///
/// A `BestFirstRule` consists of a context, a list of child rules, and several
/// function wrappers for evaluation and execution phases.
///
/// # Example
///
/// ```rust
/// use dredd_rs::rule::*;
///
/// let mut rule = BestFirstRule::new();
/// let rule2 = BestFirstRule::new();
///
/// rule.on_eval(|_| {
///     println!("Eval");
///     true
/// })
/// .on_pre_execute(|_| {
///    println!("Pre Execute");
/// })
/// .on_execute(|_| {
///   println!("Execute");
/// })
/// .on_post_execute(|_| {
///  println!("Post Execute");
/// })
/// .add_child(rule2);
///
/// Engine::best_first_runner().run(RuleContext::new(), vec![rule]);
/// ```
///
#[derive(Clone)]
pub struct BestFirstRule {
    rule_context: Option<RuleContextWrapper>,
    children: Vec<Wrapper<BestFirstRule>>,
    eval: Wrapper<dyn Fn(&mut Self) -> bool>,
    pre_execute: Wrapper<dyn Fn(&mut Self)>,
    execute: Wrapper<dyn Fn(&mut Self)>,
    post_execute: Wrapper<dyn Fn(&mut Self)>,
}

impl BestFirstRule {
    pub fn new() -> Wrapper<Self> {
        wrap(BestFirstRule {
            rule_context: None,
            children: Vec::new(),
            eval: wrap(|_: &mut Self| true),
            pre_execute: wrap(|_: &mut Self| ()),
            execute: wrap(|_: &mut Self| ()),
            post_execute: wrap(|_: &mut Self| ()),
        })
    }

    pub fn on_eval(&mut self, eval: impl Fn(&mut Self) -> bool + 'static) {
        self.eval = wrap(eval);
    }

    pub fn on_pre_execute(&mut self, pre_execute: impl Fn(&mut Self) + 'static) {
        self.pre_execute = wrap(pre_execute);
    }

    pub fn on_execute(&mut self, execute: impl Fn(&mut Self) + 'static) {
        self.execute = wrap(execute);
    }

    pub fn on_post_execute(&mut self, post_execute: impl Fn(&mut Self) + 'static) {
        self.post_execute = wrap(post_execute);
    }
}

impl Rule<BestFirstRule> for BestFirstRule {
    fn fire(&mut self) -> bool {
        if self.run_eval() {
            self.run_pre_execute();
            self.run_execute();
            self.run_post_execute();
            self.run_children();
            return false;
        }
        true
    }

    fn run_eval(&self) -> bool {
        (self.eval.borrow_mut())(&mut self.clone())
    }

    fn run_pre_execute(&mut self) {
        (self.pre_execute.borrow_mut())(&mut self.clone());
    }

    fn run_execute(&mut self) {
        (self.execute.borrow_mut())(&mut self.clone());
    }

    fn run_post_execute(&mut self) {
        (self.post_execute.borrow_mut())(&mut self.clone());
    }

    fn set_rule_context(&mut self, rule_context: RuleContextWrapper) {
        self.rule_context = Some(rule_context);
    }

    fn get_rule_context(&mut self) -> RuleContextWrapper {
        self.rule_context.clone().unwrap()
    }

    fn run_children(&mut self) {
        let children = self.get_children();
        let rule_context = self.get_rule_context();

        Engine::best_first_runner().run(rule_context, children);
    }

    fn get_children(&mut self) -> Vec<Wrapper<BestFirstRule>> {
        self.children.clone()
    }

    fn add_child(&mut self, rule: Wrapper<BestFirstRule>) {
        self.children.push(rule);
    }

    fn add_children(&mut self, rules: Vec<Wrapper<BestFirstRule>>) {
        self.children.extend(rules);
    }
}

/// Implementation of the `RuleHelper` trait for `Wrapper<BestFirstRule>`.
///
/// This implementation provides methods to set various callbacks for the
/// `BestFirstRule` wrapped inside the `Wrapper`.
///
/// # Type Parameters
/// - `RuleType`: The type of the rule, which is `BestFirstRule` in this case.
///
/// # Methods
/// - `on_eval`: Sets the evaluation callback for the rule.
/// - `on_pre_execute`: Sets the pre-execution callback for the rule.
/// - `on_execute`: Sets the execution callback for the rule.
/// - `on_post_execute`: Sets the post-execution callback for the rule.
///
/// Each method takes a closure as an argument, wraps it, assigns it to the
/// corresponding field in the rule, and returns a clone of the `Wrapper`.
impl RuleCallback for Wrapper<BestFirstRule> {
    type RuleType = BestFirstRule;

    /// Sets the evaluation function for the rule.
    fn on_eval(
        &mut self,
        eval: impl Fn(&mut Self::RuleType) -> bool + 'static,
    ) -> Wrapper<Self::RuleType> {
        self.borrow_mut().eval = wrap(eval);
        self.clone()
    }

    /// Sets the pre-execution function for the rule.
    fn on_pre_execute(
        &mut self,
        pre_execute: impl Fn(&mut Self::RuleType) + 'static,
    ) -> Wrapper<Self::RuleType> {
        self.borrow_mut().pre_execute = wrap(pre_execute);
        self.clone()
    }

    /// Sets the execution function for the rule.
    fn on_execute(
        &mut self,
        execute: impl Fn(&mut Self::RuleType) + 'static,
    ) -> Wrapper<Self::RuleType> {
        self.borrow_mut().execute = wrap(execute);
        self.clone()
    }
    /// Sets the post-execution function for the rule.
    fn on_post_execute(
        &mut self,
        post_execute: impl Fn(&mut Self::RuleType) + 'static,
    ) -> Wrapper<Self::RuleType> {
        self.borrow_mut().post_execute = wrap(post_execute);
        self.clone()
    }
}

/// Implementation of the `AddChild` trait for `Wrapper<BestFirstRule>`.
///
/// This implementation allows adding a single child rule or multiple child rules
/// to a `Wrapper<BestFirstRule>` instance.
///
/// # Type Parameters
/// - `RuleType`: The type of the rule, which is `BestFirstRule` in this case.
///
/// # Methods
/// - `add_child`: Adds a single child rule to the current instance and returns a clone of the updated instance.
/// - `add_children`: Adds multiple child rules to the current instance and returns a clone of the updated instance.
impl RuleChildren for Wrapper<BestFirstRule> {
    type RuleType = BestFirstRule;

    /// Adds a single child rule to the current instance and returns a clone of the updated instance.
    ///
    /// # Arguments
    ///
    /// * `rule` - A `Wrapper` containing the child rule to be added.
    ///
    /// # Returns
    ///
    /// A `Wrapper` containing a clone of the updated instance.
    fn add_child(&mut self, rule: Wrapper<Self::RuleType>) -> Wrapper<Self::RuleType> {
        self.borrow_mut().add_child(rule);
        self.clone()
    }

    /// Adds multiple child rules to the current instance and returns a clone of the updated instance.
    ///
    /// # Arguments
    ///
    /// * `rules` - A vector of `Wrapper` containing the child rules to be added.
    ///
    /// # Returns
    ///
    /// A `Wrapper` containing a clone of the updated instance.
    fn add_children(&mut self, rules: Vec<Wrapper<Self::RuleType>>) -> Wrapper<Self::RuleType> {
        self.borrow_mut().add_children(rules);
        self.clone()
    }
}
