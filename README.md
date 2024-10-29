# dredd-rs

This is a port of [Dredd](https://github.com/amsterdatech/Dredd) rules engine to Rust

## Dredd Rules Engine

*From the original:*

> Dredd was created to be a simple way to detach application business logic in order to create a decision tree model best for visualize and perhaps easy to understand and maintain.

---

## Chain Rule Runner 

When using the `ChainRuleRunner`, the rules will be executed in a linear sequence. When the `on_eval()` of a rule returns true, its child rule will be evaluated, continuing until there are no more child rules.

![ChainRuleRunner](img/chain-runner.png)

## Best First Rule Runner

When using the `BestFirstRuleRunner`, the rules will be executed so that if `on_eval()` returns true, the first child rule will be evaluated. If `on_eval()` returns false, the next sibling rule will be evaluated until there are no more child or sibling rules.

![alt text](img/best-first-runner.png)

## Rules

Here are some useful methods for setting up your rules:

- `on_eval()` sets the condition that determines whether the rule should execute.
- `on_execute()` contains the main code the rule should execute.
- `on_pre_execute()` any actions the rule needs to perform beforehand.
- `on_post_execute()` any actions the rule should perform afterward.
- `add_child()` helper method to add a child rule.
- `add_children()` helper method to add multiple child rules.
  
*Notes:*

* You don't need to provide all the callbacks.

* Additionally, you should pass a `RuleContext` during execution, which is a map accessible from within the rules. 

* You can even mix runners and call another runner within the execution of a rule, using a new sequence of different rules from any type.

## Example

```rust
use dredd_rs::rule::*;

let mut rule = ChainRule::new();
let mut rule2 = ChainRule::new();

rule.on_eval(|this| {
   println!("Eval Chain Rule 1")
   let should_run = this.get_rule_context().get::<bool>("test").unwrap();
   should_run //true
})
.on_pre_execute(|this| {
   println!("Pre Chain Rule 1");
})
.on_execute(|this| {
   println!("Execute Chain Rule 1");
})
.on_post_execute(|this| {
   println!("Post Chain Rule 1");
})
.add_child(
   rule2.on_eval(|this| {
      println!("Eval Chain Rule 2")
      false
   })
   .on_execute(|this| {
      println!("Execute Chain Rule 2");
   })
);

let rule_context = RuleContext::new();
rule_context.set("test", true);

Engine::ChainRuleRunner.run(rule_context, vec![rule]);
```

Result:

```
> Eval Chain Rule 1
> Pre Chain Rule 1
> Execute Chain Rule 1
> Post Chain Rule 1
> Eval Chain Rule 2
```

## Todo

- [ ] Async rules

---

# License #

    Copyright 2015 Amsterda Technology, Inc.

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.