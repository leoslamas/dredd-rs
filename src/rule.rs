use std::{collections::HashMap, fmt};

/// Error types for rule execution and configuration
#[derive(Debug, Clone, PartialEq)]
pub enum RuleError {
    /// Rule context was not set when required
    ContextNotSet,
    /// Type mismatch when retrieving value from context
    TypeMismatch { key: &'static str, expected: &'static str },
    /// Too many children added to a rule that supports only one
    TooManyChildren { max: usize, attempted: usize },
    /// Rule execution failed
    ExecutionFailed(String),
    /// Borrow check failed at runtime
    BorrowFailed(String),
}

impl fmt::Display for RuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleError::ContextNotSet => write!(f, "Rule context was not set"),
            RuleError::TypeMismatch { key, expected } => {
                write!(f, "Type mismatch for key '{}': expected {}", key, expected)
            }
            RuleError::TooManyChildren { max, attempted } => {
                write!(f, "Too many children: max {} but attempted {}", max, attempted)
            }
            RuleError::ExecutionFailed(msg) => write!(f, "Rule execution failed: {}", msg),
            RuleError::BorrowFailed(msg) => write!(f, "Borrow check failed: {}", msg),
        }
    }
}

impl std::error::Error for RuleError {}

/// Result type for rule operations
pub type RuleResult<T> = Result<T, RuleError>;

/// Type aliases for complex function types
pub type EvalFn = Box<dyn Fn(&RuleContext) -> RuleResult<bool>>;
pub type ExecuteFn = Box<dyn Fn(&mut RuleContext) -> RuleResult<()>>;

pub use crate::engine::Engine;
pub use crate::rule::best_first_rule::BestFirstRule;
pub use crate::rule::chain_rule::ChainRule;
pub use crate::runner::RuleRunner;

pub(crate) mod best_first_rule;
pub(crate) mod chain_rule;

// Remove the wrapper types - we'll use direct ownership instead
/// Context value that can hold any type safely
#[derive(Debug)]
pub enum ContextValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
}

impl ContextValue {
    /// Try to extract a boolean value
    pub fn as_bool(&self) -> RuleResult<bool> {
        match self {
            ContextValue::Bool(v) => Ok(*v),
            _ => Err(RuleError::TypeMismatch { 
                key: "unknown", 
                expected: "bool" 
            }),
        }
    }

    /// Try to extract an integer value
    pub fn as_int(&self) -> RuleResult<i64> {
        match self {
            ContextValue::Int(v) => Ok(*v),
            _ => Err(RuleError::TypeMismatch { 
                key: "unknown", 
                expected: "i64" 
            }),
        }
    }

    /// Try to extract a float value
    pub fn as_float(&self) -> RuleResult<f64> {
        match self {
            ContextValue::Float(v) => Ok(*v),
            _ => Err(RuleError::TypeMismatch { 
                key: "unknown", 
                expected: "f64" 
            }),
        }
    }

    /// Try to extract a string value
    pub fn as_string(&self) -> RuleResult<&str> {
        match self {
            ContextValue::String(v) => Ok(v),
            _ => Err(RuleError::TypeMismatch { 
                key: "unknown", 
                expected: "String" 
            }),
        }
    }

    /// Try to extract bytes
    pub fn as_bytes(&self) -> RuleResult<&[u8]> {
        match self {
            ContextValue::Bytes(v) => Ok(v),
            _ => Err(RuleError::TypeMismatch { 
                key: "unknown", 
                expected: "Vec<u8>" 
            }),
        }
    }
}

/// RuleContext is a struct that holds the context of the rule.
/// It provides type-safe access to values without using `dyn Any`.
///
/// Example:
/// ```rust
/// use dredd_rs::rule::*;
///
/// let mut rule_context = RuleContext::new();
/// rule_context.set_bool("test", true);
/// let test = rule_context.get_bool("test");
/// ```
#[derive(Debug, Default)]
pub struct RuleContext {
    context_map: HashMap<&'static str, ContextValue>,
}

impl RuleContext {
    pub fn new() -> Self {
        RuleContext {
            context_map: HashMap::new(),
        }
    }

    /// Set a boolean value in the context
    pub fn set_bool(&mut self, key: &'static str, value: bool) {
        self.context_map.insert(key, ContextValue::Bool(value));
    }

    /// Set an integer value in the context
    pub fn set_int(&mut self, key: &'static str, value: i64) {
        self.context_map.insert(key, ContextValue::Int(value));
    }

    /// Set a float value in the context
    pub fn set_float(&mut self, key: &'static str, value: f64) {
        self.context_map.insert(key, ContextValue::Float(value));
    }

    /// Set a string value in the context
    pub fn set_string(&mut self, key: &'static str, value: String) {
        self.context_map.insert(key, ContextValue::String(value));
    }

    /// Set bytes in the context
    pub fn set_bytes(&mut self, key: &'static str, value: Vec<u8>) {
        self.context_map.insert(key, ContextValue::Bytes(value));
    }

    /// Get a boolean value from the context
    pub fn get_bool(&self, key: &'static str) -> RuleResult<bool> {
        self.context_map
            .get(key)
            .ok_or(RuleError::TypeMismatch { key, expected: "bool" })?
            .as_bool()
    }

    /// Get an integer value from the context
    pub fn get_int(&self, key: &'static str) -> RuleResult<i64> {
        self.context_map
            .get(key)
            .ok_or(RuleError::TypeMismatch { key, expected: "i64" })?
            .as_int()
    }

    /// Get a float value from the context
    pub fn get_float(&self, key: &'static str) -> RuleResult<f64> {
        self.context_map
            .get(key)
            .ok_or(RuleError::TypeMismatch { key, expected: "f64" })?
            .as_float()
    }

    /// Get a string value from the context
    pub fn get_string(&self, key: &'static str) -> RuleResult<&str> {
        self.context_map
            .get(key)
            .ok_or(RuleError::TypeMismatch { key, expected: "String" })?
            .as_string()
    }

    /// Get bytes from the context
    pub fn get_bytes(&self, key: &'static str) -> RuleResult<&[u8]> {
        self.context_map
            .get(key)
            .ok_or(RuleError::TypeMismatch { key, expected: "Vec<u8>" })?
            .as_bytes()
    }

    /// Check if a key exists in the context
    pub fn contains_key(&self, key: &'static str) -> bool {
        self.context_map.contains_key(key)
    }

    /// Remove a value from the context
    pub fn remove(&mut self, key: &'static str) -> Option<ContextValue> {
        self.context_map.remove(key)
    }

    /// Clear all values from the context
    pub fn clear(&mut self) {
        self.context_map.clear();
    }
}

/// Core trait for rule execution with proper error handling
pub trait Rule {
    /// Evaluate if this rule should execute
    fn evaluate(&self, context: &RuleContext) -> RuleResult<bool>;

    /// Execute the rule with proper error handling
    fn execute(&mut self, context: &mut RuleContext) -> RuleResult<()>;

    /// Get immutable reference to children
    fn children(&self) -> &[Box<dyn Rule>];

    /// Get mutable reference to children  
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Rule>>;

    /// Add a child rule
    fn add_child(&mut self, child: Box<dyn Rule>) -> RuleResult<()>;

    /// Add multiple child rules
    fn add_children(&mut self, children: Vec<Box<dyn Rule>>) -> RuleResult<()> {
        for child in children {
            self.add_child(child)?;
        }
        Ok(())
    }

    /// Execute the complete rule lifecycle: evaluate, execute, and run children
    fn fire(&mut self, context: &mut RuleContext) -> RuleResult<bool> {
        if self.evaluate(context)? {
            self.execute(context)?;
            
            // Execute children
            for child in self.children_mut() {
                child.fire(context)?;
            }
            
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Base implementation for rules with callback support
pub struct BaseRule {
    children: Vec<Box<dyn Rule>>,
    eval_fn: Option<EvalFn>,
    pre_execute_fn: Option<ExecuteFn>,
    execute_fn: Option<ExecuteFn>,
    post_execute_fn: Option<ExecuteFn>,
}

impl BaseRule {
    pub fn new() -> Self {
        BaseRule {
            children: Vec::new(),
            eval_fn: None,
            pre_execute_fn: None,
            execute_fn: None,
            post_execute_fn: None,
        }
    }

    /// Set the evaluation function
    pub fn set_eval_fn<F>(&mut self, f: F) 
    where 
        F: Fn(&RuleContext) -> RuleResult<bool> + 'static
    {
        self.eval_fn = Some(Box::new(f));
    }

    /// Set the pre-execution function
    pub fn set_pre_execute_fn<F>(&mut self, f: F)
    where 
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static
    {
        self.pre_execute_fn = Some(Box::new(f));
    }

    /// Set the execution function
    pub fn set_execute_fn<F>(&mut self, f: F)
    where 
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static  
    {
        self.execute_fn = Some(Box::new(f));
    }

    /// Set the post-execution function
    pub fn set_post_execute_fn<F>(&mut self, f: F)
    where 
        F: Fn(&mut RuleContext) -> RuleResult<()> + 'static
    {
        self.post_execute_fn = Some(Box::new(f));
    }
}

impl Rule for BaseRule {
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
}

impl Default for BaseRule {
    fn default() -> Self {
        Self::new()
    }
}
