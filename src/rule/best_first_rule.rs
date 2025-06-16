use crate::rule::{Rule, RuleResult, RuleContext, EvalFn, ExecuteFn};

/// BestFirstRule represents a rule that executes the first child that evaluates to true.
/// If no child evaluates to true, it tries siblings until one succeeds.
///
/// # Example
///
/// ```rust
/// use dredd_rs::rule::*;
///
/// let mut rule = BestFirstRule::new();
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
pub struct BestFirstRule {
    children: Vec<Box<dyn Rule>>,
    eval_fn: Option<EvalFn>,
    pre_execute_fn: Option<ExecuteFn>,
    execute_fn: Option<ExecuteFn>,
    post_execute_fn: Option<ExecuteFn>,
}

impl BestFirstRule {
    /// Create a new BestFirstRule
    pub fn new() -> Self {
        BestFirstRule {
            children: Vec::new(),
            eval_fn: None,
            pre_execute_fn: None,
            execute_fn: None,
            post_execute_fn: None,
        }
    }

    /// Set the evaluation function
    pub fn set_eval_fn<F>(&mut self, f: F) -> &mut Self
    where 
        F: Fn(&RuleContext) -> RuleResult<bool> + 'static
    {
        self.eval_fn = Some(Box::new(f));
        self
    }

    /// Set the pre-execution function
    pub fn set_pre_execute_fn<F>(&mut self, f: F) -> &mut Self
    where 
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static
    {
        self.pre_execute_fn = Some(Box::new(f));
        self
    }

    /// Set the execution function
    pub fn set_execute_fn<F>(&mut self, f: F) -> &mut Self
    where 
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static  
    {
        self.execute_fn = Some(Box::new(f));
        self
    }

    /// Set the post-execution function
    pub fn set_post_execute_fn<F>(&mut self, f: F) -> &mut Self
    where 
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static
    {
        self.post_execute_fn = Some(Box::new(f));
        self
    }
}

impl Rule for BestFirstRule {
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
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Rule>> {
        &mut self.children
    }

    fn add_child(&mut self, child: Box<dyn Rule>) -> RuleResult<()> {
        self.children.push(child);
        Ok(())
    }

    /// Custom fire implementation for BestFirstRule that implements best-first execution
    fn fire(&mut self, context: &mut RuleContext) -> RuleResult<bool> {
        if self.evaluate(context)? {
            self.execute(context)?;
            
            // Execute the first child that evaluates to true
            for child in &mut self.children {
                if child.evaluate(context)? {
                    child.fire(context)?;
                    return Ok(true);
                }
            }
            
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Default for BestFirstRule {
    fn default() -> Self {
        Self::new()
    }
}