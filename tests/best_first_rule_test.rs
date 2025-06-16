#[cfg(test)]
mod tests {
    use dredd_rs::rule::*;

    #[test]
    fn test_best_first_rule_basic_execution() {
        let mut rule = BestFirstRule::new();
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
    fn test_best_first_rule_with_children() {
        let mut parent_rule = BestFirstRule::new();
        let mut child1 = BestFirstRule::new();
        let mut child2 = BestFirstRule::new();
        let mut context = RuleContext::new();
        
        // Set up parent rule
        parent_rule.set_eval_fn(|_context| Ok(true));
        parent_rule.set_execute_fn(|context| {
            context.set_bool("parent_executed", true);
            Ok(())
        });

        // Set up first child (should NOT execute)
        child1.set_eval_fn(|_context| Ok(false));
        child1.set_execute_fn(|context| {
            context.set_bool("child1_executed", true);
            Ok(())
        });

        // Set up second child (should execute)
        child2.set_eval_fn(|_context| Ok(true));
        child2.set_execute_fn(|context| {
            context.set_bool("child2_executed", true);
            Ok(())
        });

        // Add children to parent
        parent_rule.add_child(Box::new(child1)).unwrap();
        parent_rule.add_child(Box::new(child2)).unwrap();
        
        // Execute the rule
        let result = parent_rule.fire(&mut context).unwrap();
        
        assert!(result);
        assert_eq!(context.get_bool("parent_executed").unwrap(), true);
        // child1 should not have executed (eval returned false)
        assert!(context.get_bool("child1_executed").is_err());
        // child2 should have executed (first child that evaluated to true)
        assert_eq!(context.get_bool("child2_executed").unwrap(), true);
    }

    #[test]
    fn test_best_first_rule_evaluation_false() {
        let mut rule = BestFirstRule::new();
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
    fn test_best_first_rule_multiple_children() {
        let mut parent_rule = BestFirstRule::new();
        let mut context = RuleContext::new();
        
        // Set up parent rule
        parent_rule.set_eval_fn(|_context| Ok(true));
        parent_rule.set_execute_fn(|context| {
            context.set_bool("parent_executed", true);
            Ok(())
        });

        // Add multiple children - all evaluate to true
        let mut child1 = BestFirstRule::new();
        child1.set_eval_fn(|_context| Ok(true));
        child1.set_execute_fn(|context| {
            context.set_bool("child1_executed", true);
            Ok(())
        });
        parent_rule.add_child(Box::new(child1)).unwrap();

        let mut child2 = BestFirstRule::new();
        child2.set_eval_fn(|_context| Ok(true));
        child2.set_execute_fn(|context| {
            context.set_bool("child2_executed", true);
            Ok(())
        });
        parent_rule.add_child(Box::new(child2)).unwrap();

        let mut child3 = BestFirstRule::new();
        child3.set_eval_fn(|_context| Ok(true));
        child3.set_execute_fn(|context| {
            context.set_bool("child3_executed", true);
            Ok(())
        });
        parent_rule.add_child(Box::new(child3)).unwrap();
        
        // Execute the rule
        let result = parent_rule.fire(&mut context).unwrap();
        
        assert!(result);
        assert_eq!(context.get_bool("parent_executed").unwrap(), true);
        
        // In best-first, only the first child that evaluates to true should execute
        assert_eq!(context.get_bool("child1_executed").unwrap(), true);
        // The rest should not execute since the first one already did
        assert!(context.get_bool("child2_executed").is_err());
        assert!(context.get_bool("child3_executed").is_err());
    }

    #[test]
    fn test_best_first_rule_lifecycle() {
        let mut rule = BestFirstRule::new();
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

    #[test]
    fn test_best_first_no_matching_children() {
        let mut parent_rule = BestFirstRule::new();
        let mut context = RuleContext::new();
        
        // Set up parent rule
        parent_rule.set_eval_fn(|_context| Ok(true));
        parent_rule.set_execute_fn(|context| {
            context.set_bool("parent_executed", true);
            Ok(())
        });

        // Add children that all evaluate to false
        let mut child1 = BestFirstRule::new();
        child1.set_eval_fn(|_context| Ok(false));
        child1.set_execute_fn(|context| {
            context.set_bool("child1_executed", true);
            Ok(())
        });
        parent_rule.add_child(Box::new(child1)).unwrap();

        let mut child2 = BestFirstRule::new();
        child2.set_eval_fn(|_context| Ok(false));
        child2.set_execute_fn(|context| {
            context.set_bool("child2_executed", true);
            Ok(())
        });
        parent_rule.add_child(Box::new(child2)).unwrap();

        let mut child3 = BestFirstRule::new();
        child3.set_eval_fn(|_context| Ok(false));
        child3.set_execute_fn(|context| {
            context.set_bool("child3_executed", true);
            Ok(())
        });
        parent_rule.add_child(Box::new(child3)).unwrap();
        
        // Execute the rule
        let result = parent_rule.fire(&mut context).unwrap();
        
        assert!(result);
        assert_eq!(context.get_bool("parent_executed").unwrap(), true);
        
        // No children should have executed since they all evaluated to false
        assert!(context.get_bool("child1_executed").is_err());
        assert!(context.get_bool("child2_executed").is_err());
        assert!(context.get_bool("child3_executed").is_err());
    }
}