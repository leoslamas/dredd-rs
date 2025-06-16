use crate::rule::{EvalFn, ExecuteFn, Rule, RuleContext, RuleError, RuleResult};

/// ChainRule represents a rule that can have at most one child.
/// When executed, it will run its child rule if the evaluation succeeds.
///
/// # Example
///
/// ```rust
/// use dredd_rs::rule::*;
///
/// let mut rule = ChainRule::new();
/// rule.set_eval_fn(|context| {
///     context.get_bool("should_execute")
/// });
/// rule.set_execute_fn(|context| {
///     context.set_bool("executed", true);
///     Ok(())
/// });
///
/// let mut context = RuleContext::new();
/// context.set_bool("should_execute", true);
///
/// let result = rule.fire(&mut context).unwrap();
/// assert!(result);
/// ```
pub struct ChainRule {
    child: Option<Box<dyn Rule>>,
    eval_fn: Option<EvalFn>,
    pre_execute_fn: Option<ExecuteFn>,
    execute_fn: Option<ExecuteFn>,
    post_execute_fn: Option<ExecuteFn>,
}

impl ChainRule {
    /// Create a new ChainRule
    pub fn new() -> Self {
        ChainRule {
            child: None,
            eval_fn: None,
            pre_execute_fn: None,
            execute_fn: None,
            post_execute_fn: None,
        }
    }

    /// Create a builder for ChainRule
    pub fn builder() -> ChainRuleBuilder {
        ChainRuleBuilder::new()
    }

    /// Set the evaluation function
    pub fn set_eval_fn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&RuleContext) -> RuleResult<bool> + 'static,
    {
        self.eval_fn = Some(Box::new(f));
        self
    }

    /// Set the pre-execution function
    pub fn set_pre_execute_fn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static,
    {
        self.pre_execute_fn = Some(Box::new(f));
        self
    }

    /// Set the execution function
    pub fn set_execute_fn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static,
    {
        self.execute_fn = Some(Box::new(f));
        self
    }

    /// Set the post-execution function
    pub fn set_post_execute_fn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static,
    {
        self.post_execute_fn = Some(Box::new(f));
        self
    }

    /// Add a child rule (ChainRule can only have one child)
    pub fn set_child(&mut self, child: Box<dyn Rule>) -> RuleResult<&mut Self> {
        if self.child.is_some() {
            return Err(RuleError::TooManyChildren {
                max: 1,
                attempted: 2,
            });
        }
        self.child = Some(child);
        Ok(self)
    }
}

impl Rule for ChainRule {
    fn evaluate(&self, context: &RuleContext) -> RuleResult<bool> {
        match &self.eval_fn {
            Some(f) => f(context),
            None => Ok(true), // Default: always evaluate to true
        }
    }

    fn execute(&mut self, context: &mut RuleContext) -> RuleResult<()> {
        // Pre-execute
        if let Some(f) = &self.pre_execute_fn {
            f(context)?;
        }

        // Main execute
        if let Some(f) = &self.execute_fn {
            f(context)?;
        }

        // Post-execute
        if let Some(f) = &self.post_execute_fn {
            f(context)?;
        }

        Ok(())
    }

    fn children(&self) -> &[Box<dyn Rule>] {
        match &self.child {
            Some(child) => std::slice::from_ref(child),
            None => &[],
        }
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Rule>> {
        // This is a bit tricky for ChainRule since it has at most one child
        // We'll implement it differently in the fire method
        unimplemented!("ChainRule uses custom child execution in fire()")
    }

    fn add_child(&mut self, child: Box<dyn Rule>) -> RuleResult<()> {
        if self.child.is_some() {
            return Err(RuleError::TooManyChildren {
                max: 1,
                attempted: 2,
            });
        }
        self.child = Some(child);
        Ok(())
    }

    /// Custom fire implementation for ChainRule that handles single child execution
    fn fire(&mut self, context: &mut RuleContext) -> RuleResult<bool> {
        if self.evaluate(context)? {
            self.execute(context)?;

            // Execute the single child if it exists
            if let Some(child) = &mut self.child {
                child.fire(context)?;
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Default for ChainRule {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for ChainRule to provide a more ergonomic API
pub struct ChainRuleBuilder {
    rule: ChainRule,
}

impl ChainRuleBuilder {
    /// Create a new ChainRuleBuilder
    pub fn new() -> Self {
        ChainRuleBuilder {
            rule: ChainRule::new(),
        }
    }

    /// Set the evaluation function
    pub fn eval_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&RuleContext) -> RuleResult<bool> + 'static,
    {
        self.rule.set_eval_fn(f);
        self
    }

    /// Set the pre-execution function
    pub fn pre_execute_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static,
    {
        self.rule.set_pre_execute_fn(f);
        self
    }

    /// Set the execution function
    pub fn execute_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static,
    {
        self.rule.set_execute_fn(f);
        self
    }

    /// Set the post-execution function
    pub fn post_execute_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static,
    {
        self.rule.set_post_execute_fn(f);
        self
    }

    /// Add a child rule
    pub fn child(mut self, child: Box<dyn Rule>) -> RuleResult<Self> {
        self.rule.add_child(child)?;
        Ok(self)
    }

    /// Build the ChainRule
    pub fn build(self) -> ChainRule {
        self.rule
    }
}

impl Default for ChainRuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
