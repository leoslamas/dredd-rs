#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use dredd_rs::rule::*;

    #[test]
    fn test_best_first_rule_context() {
        let mut rule = BestFirstRule::new();
        let mut rule2 = BestFirstRule::new();

        rule.on_eval(|this| {
            this.get_rule_context().set("eval_1", true);
            return *this.get_rule_context().get::<bool>("start").unwrap();
        })
        .on_pre_execute(|this| {
            this.get_rule_context().set("pre_execute_1", true);
        })
        .on_execute(|this| {
            this.get_rule_context().set("execute_1", true);
        })
        .on_post_execute(|this| {
            this.get_rule_context().set("post_execute_1", true);
        })
        .add_child(
            rule2
                .on_eval(|this| {
                    this.get_rule_context().set("eval_2", true);
                    true
                })
                .on_pre_execute(|this| {
                    this.get_rule_context().set("pre_execute_2", true);
                })
                .on_execute(|this| {
                    this.get_rule_context().set("execute_2", true);
                })
                .on_post_execute(|this| {
                    this.get_rule_context().set("post_execute_2", true);
                }),
        );

        let mut rule_context = RuleContext::new();
        rule_context.set("start", true);

        Engine::best_first_runner().run(rule_context.clone(), vec![rule]);

        assert!(*rule_context.get::<bool>("start").unwrap());
        assert!(*rule_context.get::<bool>("eval_1").unwrap());
        assert!(*rule_context.get::<bool>("pre_execute_1").unwrap());
        assert!(*rule_context.get::<bool>("execute_1").unwrap());
        assert!(*rule_context.get::<bool>("post_execute_1").unwrap());
        assert!(*rule_context.get::<bool>("eval_2").unwrap());
        assert!(*rule_context.get::<bool>("pre_execute_2").unwrap());
        assert!(*rule_context.get::<bool>("execute_2").unwrap());
        assert!(*rule_context.get::<bool>("post_execute_2").unwrap());
    }

    #[test]
    fn test_best_first_rule_should_not_panic_on_default_callbacks() {
        let mut rule = BestFirstRule::new();
        let rule2 = BestFirstRule::new();

        rule.add_child(rule2);

        Engine::best_first_runner().run(RuleContext::new(), vec![rule]);
    }

    #[test]
    fn test_best_first_should_run_child_on_eval_true() {
        let mut rule = BestFirstRule::new();
        rule.on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule1", true)); //TRUE

        let mut rule2 = BestFirstRule::new();
        rule2
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule2", true)); //FALSE

        let mut rule3 = BestFirstRule::new();
        rule3
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule3", true)); //TRUE

        let mut rule4 = BestFirstRule::new();
        rule4
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule4", true)); //FALSE

        rule.add_child(rule3);
        rule2.add_child(rule4);

        let rule_context = RuleContext::new();

        Engine::best_first_runner().run(rule_context.clone(), vec![rule, rule2]);

        assert_eq!(
            *rule_context.get::<bool>("rule1").unwrap_or(Rc::new(false)),
            true
        );
        assert_eq!(
            *rule_context.get::<bool>("rule2").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule3").unwrap_or(Rc::new(false)),
            true
        );
        assert_eq!(
            *rule_context.get::<bool>("rule4").unwrap_or(Rc::new(false)),
            false
        );
    }

    #[test]
    fn test_best_first_should_run_sibling_on_eval_false() {
        let mut rule = BestFirstRule::new();
        rule.on_eval(|_| false)
            .on_execute(|this| this.get_rule_context().set("rule1", true)); //FALSE

        let mut rule2 = BestFirstRule::new();
        rule2
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule2", true)); //TRUE

        let mut rule3 = BestFirstRule::new();
        rule3
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule3", true)); //FALSE

        let mut rule4 = BestFirstRule::new();
        rule4
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule4", true)); //TRUE

        rule.add_child(rule3);
        rule2.add_child(rule4);

        let rule_context = RuleContext::new();

        Engine::best_first_runner().run(rule_context.clone(), vec![rule, rule2]);

        assert_eq!(
            *rule_context.get::<bool>("rule1").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule2").unwrap_or(Rc::new(false)),
            true
        );
        assert_eq!(
            *rule_context.get::<bool>("rule3").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule4").unwrap_or(Rc::new(false)),
            true
        );
    }

    #[test]
    fn test_best_first_readme_flow() {
        // Rule1      Rule2      Rule3
        //   |
        // Rule4  ->  Rule5      Rule6
        //              |
        // Rule7      Rule8      Rule9
        //              |
        // Rule10     Rule11  ->  Rule12

        let mut rule = BestFirstRule::new()
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule1", true));

        let mut rule2 = BestFirstRule::new();
        rule2
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule2", true));

        let mut rule3 = BestFirstRule::new();
        rule3
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule3", true));

        let mut rule4 = BestFirstRule::new();
        rule4
            .on_eval(|_| false)
            .on_execute(|this| this.get_rule_context().set("rule4", true)); //FALSE

        let mut rule5 = BestFirstRule::new();
        rule5
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule5", true));

        let mut rule6 = BestFirstRule::new();
        rule6
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule6", true));

        let mut rule7 = BestFirstRule::new();
        rule7
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule7", true));

        let mut rule8 = BestFirstRule::new();
        rule8
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule8", true));

        let mut rule9 = BestFirstRule::new();
        rule9
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule9", true));

        let mut rule10 = BestFirstRule::new();
        rule10
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule10", true));

        let mut rule11 = BestFirstRule::new();
        rule11
            .on_eval(|_| false)
            .on_execute(|this| this.get_rule_context().set("rule11", true)); //FALSE

        let mut rule12 = BestFirstRule::new();
        rule12
            .on_eval(|_| true)
            .on_execute(|this| this.get_rule_context().set("rule12", true));

        rule.add_children(vec![rule4.clone(), rule5.clone(), rule6]);
        rule4.add_child(rule7.clone());
        rule5.add_children(vec![rule8.clone(), rule9]);
        rule7.add_child(rule10);
        rule8.add_children(vec![rule11, rule12]);

        let rule_context = RuleContext::new();

        Engine::best_first_runner().run(rule_context.clone(), vec![rule, rule2, rule3]);

        assert_eq!(
            *rule_context.get::<bool>("rule1").unwrap_or(Rc::new(false)), //TRUE
            true
        );
        assert_eq!(
            *rule_context.get::<bool>("rule2").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule3").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule4").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule5").unwrap_or(Rc::new(false)), //TRUE
            true
        );
        assert_eq!(
            *rule_context.get::<bool>("rule6").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule7").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule8").unwrap_or(Rc::new(false)), //TRUE
            true
        );
        assert_eq!(
            *rule_context.get::<bool>("rule9").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule10").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule11").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule12").unwrap_or(Rc::new(false)), //TRUE
            true
        );
    }
}
