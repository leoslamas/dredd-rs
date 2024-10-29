#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use dredd_rs::rule::*;

    #[test]
    fn test_chain_rule_context() {
        let mut rule = ChainRule::new();
        let mut rule2 = ChainRule::new();

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

        Engine::chain_runner().run(rule_context.clone(), vec![rule]);

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
    fn test_chain_rule_should_not_panic_on_default_callbacks() {
        let mut rule = ChainRule::new();
        let rule2 = ChainRule::new();

        rule.add_child(rule2);

        Engine::chain_runner().run(RuleContext::new(), vec![rule]);
    }

    #[test]
    fn test_chain_rule_run_child_on_eval_true() {
        let mut rule = ChainRule::new();
        rule.on_eval(|_| true).on_execute(|this| {
            this.get_rule_context().set("rule1", true);
        });

        let mut rule2 = ChainRule::new();
        rule2.on_eval(|_| true).on_execute(|this| {
            this.get_rule_context().set("rule2", true);
        });

        let mut rule3 = ChainRule::new();
        rule3.on_eval(|_| true).on_execute(|this| {
            this.get_rule_context().set("rule3", true);
        });

        rule.add_child(rule2.add_child(rule3));

        let rule_context = RuleContext::new();
        Engine::chain_runner().run(rule_context.clone(), vec![rule]);

        assert!(*rule_context.get::<bool>("rule1").unwrap());
        assert!(*rule_context.get::<bool>("rule2").unwrap());
        assert!(*rule_context.get::<bool>("rule3").unwrap());
    }

    #[test]
    fn test_chain_rule_stop_running_on_eval_false() {
        let mut rule = ChainRule::new();
        rule.on_eval(|_| true).on_execute(|this| {
            this.get_rule_context().set("rule1", true);
        });

        let mut rule2 = ChainRule::new();
        rule2.on_eval(|_| false).on_execute(|this| {
            this.get_rule_context().set("rule2", true);
        });

        let mut rule3 = ChainRule::new();
        rule3.on_eval(|_| true).on_execute(|this| {
            this.get_rule_context().set("rule3", true);
        });

        rule.add_child(rule2.add_child(rule3));

        let rule_context = RuleContext::new();
        Engine::chain_runner().run(rule_context.clone(), vec![rule]);

        assert!(*rule_context.get::<bool>("rule1").unwrap_or(Rc::new(false)));
        assert_eq!(
            *rule_context.get::<bool>("rule2").unwrap_or(Rc::new(false)),
            false
        );
        assert_eq!(
            *rule_context.get::<bool>("rule3").unwrap_or(Rc::new(false)),
            false
        );
    }

    #[test]
    #[should_panic]
    fn test_chain_rule_panic_on_passing_sibling_rules_to_runner() {
        let mut rule = ChainRule::new();
        rule.on_eval(|_| true).on_execute(|this| {
            this.get_rule_context().set("rule1", true);
        });

        let mut rule2 = ChainRule::new();
        rule2.on_eval(|_| false).on_execute(|this| {
            this.get_rule_context().set("rule2", true);
        });

        let rule_context = RuleContext::new();

        Engine::chain_runner().run(rule_context.clone(), vec![rule, rule2]);

        assert!(*rule_context.get::<bool>("rule1").unwrap_or(Rc::new(false)));
        assert!(*rule_context.get::<bool>("rule1").unwrap_or(Rc::new(false)));
    }

    #[test]
    #[should_panic]
    fn test_chain_rule_panic_on_passing_sibling_rules_to_rule() {
        let mut rule = ChainRule::new();
        rule.on_eval(|_| true).on_execute(|this| {
            this.get_rule_context().set("rule1", true);
        });

        let mut rule2 = ChainRule::new();
        rule2.on_eval(|_| false).on_execute(|this| {
            this.get_rule_context().set("rule2", true);
        });

        let mut rule3 = ChainRule::new();
        rule3.on_eval(|_| true).on_execute(|this| {
            this.get_rule_context().set("rule3", true);
        });

        rule.add_children(vec![rule2, rule3]);

        let rule_context = RuleContext::new();

        Engine::chain_runner().run(rule_context.clone(), vec![rule]);

        assert!(*rule_context.get::<bool>("rule1").unwrap_or(Rc::new(false)));
        assert!(*rule_context.get::<bool>("rule2").unwrap_or(Rc::new(false)));
        assert!(*rule_context.get::<bool>("rule3").unwrap_or(Rc::new(false)));
    }
}
