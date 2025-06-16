#[cfg(test)]
mod tests {
    use dredd_rs::rule::*;

    #[test]
    fn test_chain_rule_basic_execution() {
        let mut rule = ChainRule::new();
        let mut context = RuleContext::new();
        
        // Set up the rule
        rule.set_eval_fn(|context| {
            Ok(context.get_bool("should_execute").unwrap_or(true))
        });
        
        rule.set_execute_fn(|context| {
            context.set_bool("executed", true);
            Ok(())
        });

        // Set up the context
        context.set_bool("should_execute", true);
        
        // Execute the rule
        let result = rule.fire(&mut context).unwrap();
        
        assert!(result);
        assert_eq!(context.get_bool("executed").unwrap(), true);
    }

    #[test]
    fn test_chain_rule_with_child() {
        let mut parent_rule = ChainRule::new();
        let mut child_rule = ChainRule::new();
        let mut context = RuleContext::new();
        
        // Set up parent rule
        parent_rule.set_eval_fn(|context| {
            Ok(context.get_bool("parent_should_execute").unwrap_or(true))
        });
        
        parent_rule.set_execute_fn(|context| {
            context.set_bool("parent_executed", true);
            Ok(())
        });

        // Set up child rule
        child_rule.set_eval_fn(|context| {
            Ok(context.get_bool("child_should_execute").unwrap_or(true))
        });
        
        child_rule.set_execute_fn(|context| {
            context.set_bool("child_executed", true);
            Ok(())
        });

        // Add child to parent
        parent_rule.add_child(Box::new(child_rule)).unwrap();

        // Set up the context
        context.set_bool("parent_should_execute", true);
        context.set_bool("child_should_execute", true);
        
        // Execute the rule
        let result = parent_rule.fire(&mut context).unwrap();
        
        assert!(result);
        assert_eq!(context.get_bool("parent_executed").unwrap(), true);
        assert_eq!(context.get_bool("child_executed").unwrap(), true);
    }

    #[test]
    fn test_chain_rule_evaluation_false() {
        let mut rule = ChainRule::new();
        let mut context = RuleContext::new();
        
        // Set up the rule to not execute
        rule.set_eval_fn(|_context| Ok(false));
        
        rule.set_execute_fn(|context| {
            context.set_bool("executed", true);
            Ok(())
        });

        // Execute the rule
        let result = rule.fire(&mut context).unwrap();
        
        assert!(!result);
        // Should not have executed
        assert!(context.get_bool("executed").is_err());
    }

    #[test]
    fn test_chain_rule_only_one_child() {
        let mut rule = ChainRule::new();
        let child1 = Box::new(ChainRule::new());
        let child2 = Box::new(ChainRule::new());
        
        // Add first child should succeed
        rule.add_child(child1).unwrap();
        
        // Add second child should fail
        let result = rule.add_child(child2);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            RuleError::TooManyChildren { max: 1, attempted: 2 } => {},
            other => panic!("Expected TooManyChildren error, got {:?}", other),
        }
    }

    #[test]
    fn test_chain_rule_lifecycle() {
        let mut rule = ChainRule::new();
        let mut context = RuleContext::new();
        
        // Set up the rule with all lifecycle functions
        rule.set_eval_fn(|_context| Ok(true));
        
        rule.set_pre_execute_fn(|context| {
            context.set_int("order", 1);
            Ok(())
        });
        
        rule.set_execute_fn(|context| {
            let current = context.get_int("order").unwrap_or(0);
            context.set_int("order", current + 1);
            Ok(())
        });
        
        rule.set_post_execute_fn(|context| {
            let current = context.get_int("order").unwrap_or(0);
            context.set_int("order", current + 1);
            Ok(())
        });

        // Execute the rule
        let result = rule.fire(&mut context).unwrap();
        
        assert!(result);
        // Should have executed pre (1) + execute (+1=2) + post (+1=3)
        assert_eq!(context.get_int("order").unwrap(), 3);
    }
}