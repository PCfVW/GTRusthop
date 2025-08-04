//! # Backtracking HTN Example for GTRusthop
//! 
//! This example demonstrates HTN planning with backtracking capabilities, showing how
//! the planner can backtrack through multiple task methods when earlier choices fail.
//! 
//! This is a faithful Rust port of the Python `backtracking_htn.py` example from GTPyhop,
//! maintaining the HTN principle of using **only task methods**.
//! 
//! ## Key Concepts Demonstrated
//! 
//! - **HTN Backtracking**: How the planner tries different task methods when one fails
//! - **Method Ordering**: Multiple methods for the same task, tried in sequence
//! - **Failure Propagation**: How action failures cause method backtracking
//! - **State Dependencies**: How later tasks depend on earlier task outcomes
//! 
//! ## Domain Description
//! 
//! The domain is intentionally simple to focus on backtracking behavior:
//! - **State**: Contains a single `flag` variable
//! - **Actions**: `putv(flag_val)` sets the flag, `getv(flag_val)` succeeds only if flag matches
//! - **Tasks**: Various tasks with multiple methods that may fail
//! 
//! ## Backtracking Scenarios
//! 
//! 1. **Simple Backtracking**: `put_it` task has methods that may fail, forcing backtracking
//! 2. **Complex Backtracking**: Multiple tasks with interdependencies requiring multiple backtracks
//! 3. **Method Ordering**: Different method orderings (`need01` vs `need10`) affect backtracking
//! 
//! **Python equivalent**: `backtracking_htn.py`

use crate::core::{State, Domain, PlanItem, StateValue};
use crate::planning::PlannerBuilder;
use crate::error::Result;

/// Run backtracking HTN examples demonstrating HTN backtracking capabilities
pub fn run_backtracking_htn_examples() -> Result<()> {
    println!("=== Running Backtracking HTN Examples ===");
    
    // Create the backtracking HTN domain
    let domain = create_backtracking_htn_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(3)? // High verbosity to see backtracking
        .build()?;
    
    // Create initial state
    let state0 = create_initial_state();
    
    println!("\nInitial state:");
    state0.display(None);
    
    // Expected results for verification
    let expect0 = vec![
        PlanItem::action("putv", vec![0.into()]),
        PlanItem::action("getv", vec![0.into()]),
        PlanItem::action("getv", vec![0.into()])
    ];
    let expect1 = vec![
        PlanItem::action("putv", vec![1.into()]),
        PlanItem::action("getv", vec![1.into()]),
        PlanItem::action("getv", vec![1.into()])
    ];
    
    println!("\nNext are some example problems with verbose=3 to see the backtracking.\n");
    
    // Test 1: Simple backtracking
    println!("Below, find_plan backtracks once to use a different method for 'put_it'.");
    let todo_list = vec![
        PlanItem::task("put_it", vec![]),
        PlanItem::task("need0", vec![])
    ];
    let result = planner.find_plan(state0.clone(), todo_list)?;
    check_result(&result, &expect0, "Test 1");
    
    // Test 2: Same backtracking pattern
    println!("\nThe backtracking in this example is the same as in the first one.");
    let todo_list = vec![
        PlanItem::task("put_it", vec![]),
        PlanItem::task("need01", vec![])
    ];
    let result = planner.find_plan(state0.clone(), todo_list)?;
    check_result(&result, &expect0, "Test 2");
    
    // Test 3: Multiple backtracks
    println!("\nBelow, find_plan backtracks to use a different method for 'put_it',");
    println!("and later it backtracks to use a different method for 'need10'.");
    let todo_list = vec![
        PlanItem::task("put_it", vec![]),
        PlanItem::task("need10", vec![])
    ];
    let result = planner.find_plan(state0.clone(), todo_list)?;
    check_result(&result, &expect0, "Test 3");
    
    // Test 4: Complex backtracking
    println!("\nFirst, find_plan backtracks to use a different method for 'put_it'. But the");
    println!("solution it finds for 'put_it' doesn't satisfy the preconditions of the");
    println!("method for 'need1', making it backtrack to use a third method for 'put_it'.");
    let todo_list = vec![
        PlanItem::task("put_it", vec![]),
        PlanItem::task("need1", vec![])
    ];
    let result = planner.find_plan(state0, todo_list)?;
    check_result(&result, &expect1, "Test 4");
    
    println!("\n=== Backtracking HTN Examples Completed ===");
    Ok(())
}

/// Create the backtracking HTN domain with actions and task methods
pub fn create_backtracking_htn_domain() -> Result<Domain> {
    let mut domain = Domain::new("backtracking_htn");
    
    // Declare actions
    declare_backtracking_actions(&mut domain)?;
    
    // Declare task methods
    declare_backtracking_task_methods(&mut domain)?;
    
    Ok(domain)
}

/// Declare actions for the backtracking domain
fn declare_backtracking_actions(domain: &mut Domain) -> Result<()> {
    // putv action: set the flag value
    domain.declare_action("putv", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 1 {
            if let Some(flag_val) = args[0].as_i64() {
                state.set_var("flag", "value", flag_val.into());
                return Some(state.clone());
            }
        }
        None
    })?;
    
    // getv action: succeed only if flag matches the expected value
    domain.declare_action("getv", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 1 {
            if let Some(expected_val) = args[0].as_i64() {
                if let Some(current_val) = state.get_var("flag", "value") {
                    if current_val.as_i64() == Some(expected_val) {
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;
    
    Ok(())
}

/// Declare task methods for the backtracking domain
fn declare_backtracking_task_methods(domain: &mut Domain) -> Result<()> {
    // Task methods for 'put_it' - multiple methods that may fail
    domain.declare_task_method("put_it", |_state: &State, _args: &[StateValue]| {
        // m_err method - this will fail because getv(1) won't match putv(0)
        Some(vec![
            PlanItem::action("putv", vec![0.into()]),
            PlanItem::action("getv", vec![1.into()])
        ])
    })?;
    
    domain.declare_task_method("put_it", |_state: &State, _args: &[StateValue]| {
        // m0 method - puts 0 and gets 0
        Some(vec![
            PlanItem::action("putv", vec![0.into()]),
            PlanItem::action("getv", vec![0.into()])
        ])
    })?;
    
    domain.declare_task_method("put_it", |_state: &State, _args: &[StateValue]| {
        // m1 method - puts 1 and gets 1
        Some(vec![
            PlanItem::action("putv", vec![1.into()]),
            PlanItem::action("getv", vec![1.into()])
        ])
    })?;
    
    // Task methods for 'need0'
    domain.declare_task_method("need0", |_state: &State, _args: &[StateValue]| {
        Some(vec![PlanItem::action("getv", vec![0.into()])])
    })?;
    
    // Task methods for 'need1'
    domain.declare_task_method("need1", |_state: &State, _args: &[StateValue]| {
        Some(vec![PlanItem::action("getv", vec![1.into()])])
    })?;
    
    // Task methods for 'need01' - tries need0 first, then need1
    domain.declare_task_method("need01", |_state: &State, _args: &[StateValue]| {
        Some(vec![PlanItem::action("getv", vec![0.into()])])
    })?;
    
    domain.declare_task_method("need01", |_state: &State, _args: &[StateValue]| {
        Some(vec![PlanItem::action("getv", vec![1.into()])])
    })?;
    
    // Task methods for 'need10' - tries need1 first, then need0
    domain.declare_task_method("need10", |_state: &State, _args: &[StateValue]| {
        Some(vec![PlanItem::action("getv", vec![1.into()])])
    })?;
    
    domain.declare_task_method("need10", |_state: &State, _args: &[StateValue]| {
        Some(vec![PlanItem::action("getv", vec![0.into()])])
    })?;
    
    Ok(())
}

/// Create the initial state for backtracking examples
fn create_initial_state() -> State {
    let mut state = State::new("state0");
    state.set_var("flag", "value", (-1).into());
    state
}

/// Check if the result matches the expected plan
fn check_result(result: &Option<Vec<PlanItem>>, expected: &[PlanItem], test_name: &str) {
    match result {
        Some(plan) => {
            if plan.len() == expected.len() {
                let matches = plan.iter().zip(expected.iter()).all(|(actual, expected)| {
                    actual.name() == expected.name() && actual.args() == expected.args()
                });
                if matches {
                    println!("✓ {} passed: Found expected plan with {} actions", test_name, plan.len());
                } else {
                    println!("✗ {} failed: Plan actions don't match expected", test_name);
                }
            } else {
                println!("✗ {} failed: Expected {} actions, got {}", test_name, expected.len(), plan.len());
            }
        }
        None => {
            println!("✗ {} failed: No plan found", test_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_backtracking_htn_examples() -> Result<()> {
        run_backtracking_htn_examples()
    }
    
    #[test]
    fn test_create_backtracking_htn_domain() -> Result<()> {
        let domain = create_backtracking_htn_domain()?;
        assert_eq!(domain.name, "backtracking_htn");
        Ok(())
    }
    
    #[test]
    fn test_backtracking_actions() -> Result<()> {
        let domain = create_backtracking_htn_domain()?;
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;
        
        let state = create_initial_state();
        
        // Test putv action
        let plan = planner.find_plan(state.clone(), vec![
            PlanItem::action("putv", vec![5.into()])
        ])?;
        assert!(plan.is_some());
        
        // Test getv action that should fail
        let plan = planner.find_plan(state.clone(), vec![
            PlanItem::action("getv", vec![5.into()])
        ])?;
        assert!(plan.is_none()); // Should fail because flag is -1, not 5
        
        Ok(())
    }
    
    #[test]
    fn test_simple_backtracking() -> Result<()> {
        let domain = create_backtracking_htn_domain()?;
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;
        
        let state = create_initial_state();
        
        // This should succeed with backtracking
        let todo_list = vec![
            PlanItem::task("put_it", vec![]),
            PlanItem::task("need0", vec![])
        ];
        let plan = planner.find_plan(state, todo_list)?;
        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.len(), 3); // putv, getv, getv
        
        Ok(())
    }
}
