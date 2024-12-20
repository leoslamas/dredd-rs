use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

pub use crate::engine::Engine;
pub use crate::rule::best_first_rule::BestFirstRule;
pub use crate::rule::chain_rule::ChainRule;
pub use crate::runner::RuleRunner;

pub(crate) mod best_first_rule;
pub(crate) mod chain_rule;

pub(crate) type Wrapper<T> = Rc<RefCell<T>>;
pub(crate) type RuleContextWrapper = Rc<RefCell<RuleContext>>;
pub(crate) type RuleContextMap = HashMap<&'static str, Rc<dyn Any + 'static>>;

pub(crate) fn wrap<T>(something: T) -> Wrapper<T> {
    Rc::new(RefCell::new(something))
}

/// RuleContext is a struct that holds the context of the rule.
/// It is used to store and retrieve values that are used by the rules.
///
/// Example:
/// ```rust
/// use dredd_rs::rule::*;
///
/// let mut rule_context = RuleContext::new();
/// rule_context.set("test", true);
/// let test = rule_context.get::<bool>("test");
/// ```
#[derive(Debug, Clone)]
pub struct RuleContext {
    context_map: RuleContextMap,
}

impl RuleContext {
    pub fn new() -> Wrapper<Self> {
        wrap(RuleContext {
            context_map: HashMap::new(),
        })
    }
}

pub trait GetSet {
    fn set<T: 'static>(&mut self, k: &'static str, v: T);
    fn get<T: 'static>(&self, key: &'static str) -> Option<Rc<T>>;
}

impl GetSet for RuleContext {
    fn set<T: 'static>(&mut self, k: &'static str, v: T) {
        self.context_map.insert(k, Rc::new(v));
    }

    fn get<T: 'static>(&self, key: &'static str) -> Option<Rc<T>> {
        let val = self.context_map.get(key).cloned();
        if let Some(v) = val {
            if let Ok(result) = v.downcast::<T>() {
                return Some(result.clone());
            }
        }
        None
    }
}

impl GetSet for RuleContextWrapper {
    fn set<T: 'static>(&mut self, k: &'static str, v: T) {
        self.borrow_mut().set(k, v);
    }

    fn get<T: 'static>(&self, key: &'static str) -> Option<Rc<T>> {
        self.borrow_mut().get::<T>(key)
    }
}

pub trait Rule<T> {
    fn fire(&mut self) -> bool;

    fn run_eval(&self) -> bool;
    fn run_pre_execute(&mut self);
    fn run_execute(&mut self);
    fn run_post_execute(&mut self);

    fn set_rule_context(&mut self, rule_context: RuleContextWrapper);
    fn get_rule_context(&mut self) -> RuleContextWrapper;

    fn run_children(&mut self);
    fn get_children(&mut self) -> Vec<Wrapper<T>>;
    fn add_child(&mut self, rule: Wrapper<T>);
    fn add_children(&mut self, rules: Vec<Wrapper<T>>);
}

pub trait RuleCallback {
    type RuleType;
    fn on_eval(
        &mut self,
        eval: impl Fn(&mut Self::RuleType) -> bool + 'static,
    ) -> Wrapper<Self::RuleType>;
    fn on_pre_execute(
        &mut self,
        pre_execute: impl Fn(&mut Self::RuleType) + 'static,
    ) -> Wrapper<Self::RuleType>;
    fn on_execute(
        &mut self,
        execute: impl Fn(&mut Self::RuleType) + 'static,
    ) -> Wrapper<Self::RuleType>;
    fn on_post_execute(
        &mut self,
        post_execute: impl Fn(&mut Self::RuleType) + 'static,
    ) -> Wrapper<Self::RuleType>;
}

pub trait RuleChildren {
    type RuleType;
    fn add_child(&mut self, rule: Wrapper<Self::RuleType>) -> Wrapper<Self::RuleType>;
    fn add_children(&mut self, rules: Vec<Wrapper<Self::RuleType>>) -> Wrapper<Self::RuleType>;
}

#[derive(Clone)]
pub struct BaseRule<T> {
    rule_context: Option<RuleContextWrapper>,
    children: Vec<Wrapper<T>>, // Usar Self permite que a struct seja genérica
    eval: Wrapper<dyn Fn(&mut T) -> bool>,
    pre_execute: Wrapper<dyn Fn(&mut T)>,
    execute: Wrapper<dyn Fn(&mut T)>,
    post_execute: Wrapper<dyn Fn(&mut T)>,
}

impl<T> BaseRule<T> {
    pub fn new() -> Wrapper<Self> {
        wrap(BaseRule {
            rule_context: None,
            children: Vec::new(),
            eval: wrap(|_: &mut T| true),
            pre_execute: wrap(|_: &mut T| ()),
            execute: wrap(|_: &mut T| ()),
            post_execute: wrap(|_: &mut T| ()),
        })
    }

    pub fn get_eval(&self) -> Wrapper<dyn Fn(&mut T) -> bool> {
        self.eval.clone()
    }

    pub fn set_eval(&mut self, eval: impl Fn(&mut T) -> bool + 'static) {
        self.eval = wrap(eval);
    }

    pub fn get_pre_execute(&self) -> Wrapper<dyn Fn(&mut T)> {
        self.pre_execute.clone()
    }

    pub fn set_pre_execute(&mut self, pre_execute: impl Fn(&mut T) + 'static) {
        self.pre_execute = wrap(pre_execute);
    }

    pub fn get_execute(&self) -> Wrapper<dyn Fn(&mut T)> {
        self.execute.clone()
    }

    pub fn set_execute(&mut self, execute: impl Fn(&mut T) + 'static) {
        self.execute = wrap(execute);
    }

    pub fn get_post_execute(&self) -> Wrapper<dyn Fn(&mut T)> {
        self.post_execute.clone()
    }

    pub fn set_post_execute(&mut self, post_execute: impl Fn(&mut T) + 'static) {
        self.post_execute = wrap(post_execute);
    }

    pub fn get_rule_context(&self) -> Option<RuleContextWrapper> {
        self.rule_context.clone()
    }

    pub fn set_rule_context(&mut self, rule_context: RuleContextWrapper) {
        self.rule_context = Some(rule_context);
    }

    pub fn get_children(&self) -> Vec<Wrapper<T>> {
        self.children.clone()
    }

    pub fn add_children(&mut self, rules: Vec<Wrapper<T>>) {
        self.children.extend(rules);
    }

    pub fn add_child(&mut self, rule: Wrapper<T>) {
        self.children.push(rule);
    }
}
