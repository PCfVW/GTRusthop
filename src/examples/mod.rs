//! Example domains and test cases for GTRusthop
//!
//! This module contains comprehensive examples demonstrating both HTN and HGN planning paradigms.

pub mod simple_htn_example;
pub mod simple_hgn_example;
pub mod blocks_htn_example;
pub mod lazy_lookahead_example;
pub mod backtracking_htn_example;
pub mod logistics_hgn_example;
pub mod regression_tests;

// Re-export main example functions
pub use simple_htn_example::run_simple_htn_examples;
pub use simple_hgn_example::run_simple_hgn_examples;
pub use blocks_htn_example::run_blocks_htn_examples;
pub use lazy_lookahead_example::run_lazy_lookahead_examples;
pub use backtracking_htn_example::run_backtracking_htn_examples;
pub use logistics_hgn_example::run_logistics_hgn_examples;
pub use regression_tests::{run_regression_tests, run_domain_regression_tests};

use crate::core::{State, PlanItem, TodoList, Domain};
use crate::planning::PlannerBuilder;
use crate::error::Result;

/// Helper function to run an example with error handling
pub fn run_example<F>(name: &str, example_fn: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    println!("\n=== Running {name} ===");
    match example_fn() {
        Ok(()) => {
            println!("=== {name} completed successfully ===\n");
            Ok(())
        }
        Err(e) => {
            eprintln!("=== {name} failed: {e} ===\n");
            Err(e)
        }
    }
}

/// Helper function to test planning with different verbosity levels
pub fn test_planning_with_verbosity(
    domain: Domain,
    state: State,
    todo_list: TodoList,
    expected_plan_length: Option<usize>,
) -> Result<()> {
    for verbose_level in 0..=3 {
        println!("\n--- Testing with verbose level {verbose_level} ---");

        let planner = PlannerBuilder::new()
            .with_domain(domain.clone())
            .with_verbose_level(verbose_level)?
            .build()?;

        let plan = planner.find_plan(state.clone(), todo_list.clone())?;
        
        match plan {
            Some(plan) => {
                if let Some(expected_len) = expected_plan_length {
                    if plan.len() != expected_len {
                        return Err(crate::error::GTRustHopError::planning_failed(
                            format!("Expected plan length {}, got {}", expected_len, plan.len())
                        ));
                    }
                }
                println!("Plan found with {} actions", plan.len());
            }
            None => {
                println!("No plan found");
                if expected_plan_length.is_some() {
                    return Err(crate::error::GTRustHopError::planning_failed(
                        "Expected to find a plan but got None".to_string()
                    ));
                }
            }
        }
    }
    
    Ok(())
}

/// Helper function to create a simple travel task
pub fn create_travel_task(person: &str, from: &str, to: &str) -> PlanItem {
    PlanItem::task("travel", vec![person.into(), from.into(), to.into()])
}

/// Helper function to create a simple transport task
pub fn create_transport_task(obj: &str, destination: &str) -> PlanItem {
    PlanItem::task("transport", vec![obj.into(), destination.into()])
}

/// Helper function to create a simple move action
pub fn create_move_action(obj: &str, destination: &str) -> PlanItem {
    PlanItem::action("move", vec![obj.into(), destination.into()])
}

/// Helper function to create a location unigoal
pub fn create_location_goal(obj: &str, location: &str) -> PlanItem {
    PlanItem::unigoal("loc", obj, location.into())
}

/// Helper function to pause execution for interactive examples
pub fn pause_for_user(do_pause: bool) {
    if do_pause {
        println!(">>> Press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    } else {
        println!("Continuing without pause...");
    }
}

/// Helper function to check if a plan is valid
pub fn validate_plan(plan: &[PlanItem]) -> Result<()> {
    for (i, item) in plan.iter().enumerate() {
        match item {
            PlanItem::Action(name, _args) => {
                if name.is_empty() {
                    return Err(crate::error::GTRustHopError::planning_failed(
                        format!("Action at index {i} has empty name")
                    ));
                }
                // Additional validation could be added here
            }
            PlanItem::Task(name, _args) => {
                if name.is_empty() {
                    return Err(crate::error::GTRustHopError::planning_failed(
                        format!("Task at index {i} has empty name")
                    ));
                }
            }
            _ => {
                return Err(crate::error::GTRustHopError::planning_failed(
                    format!("Plan should only contain actions, found {item:?} at index {i}")
                ));
            }
        }
    }
    Ok(())
}

/// Helper function to print a plan in a readable format
pub fn print_plan(plan: &[PlanItem]) {
    if plan.is_empty() {
        println!("Empty plan");
    } else {
        println!("Plan ({} actions):", plan.len());
        for (i, action) in plan.iter().enumerate() {
            println!("  {}: {}", i + 1, action);
        }
    }
}

/// Helper function to compare two plans for equality
pub fn plans_equal(plan1: &[PlanItem], plan2: &[PlanItem]) -> bool {
    if plan1.len() != plan2.len() {
        return false;
    }
    
    plan1.iter().zip(plan2.iter()).all(|(a, b)| a == b)
}

/// Helper function to run a planning test case
pub fn run_planning_test(
    name: &str,
    domain: Domain,
    state: State,
    todo_list: TodoList,
    expected_success: bool,
    expected_plan_length: Option<usize>,
) -> Result<()> {
    println!("\n--- Test: {name} ---");

    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    let plan = planner.find_plan(state, todo_list)?;
    
    match (plan, expected_success) {
        (Some(plan), true) => {
            println!("✓ Plan found as expected");
            print_plan(&plan);
            validate_plan(&plan)?;
            
            if let Some(expected_len) = expected_plan_length {
                if plan.len() != expected_len {
                    return Err(crate::error::GTRustHopError::planning_failed(
                        format!("Expected plan length {}, got {}", expected_len, plan.len())
                    ));
                }
                println!("✓ Plan length matches expected ({expected_len})");
            }
        }
        (None, false) => {
            println!("✓ No plan found as expected");
        }
        (Some(plan), false) => {
            println!("✗ Unexpected plan found:");
            print_plan(&plan);
            return Err(crate::error::GTRustHopError::planning_failed(
                "Expected planning to fail but a plan was found".to_string()
            ));
        }
        (None, true) => {
            println!("✗ Expected plan but none found");
            return Err(crate::error::GTRustHopError::planning_failed(
                "Expected to find a plan but planning failed".to_string()
            ));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_plan_validation() {
        let valid_plan = vec![
            create_move_action("obj1", "loc2"),
            create_move_action("obj2", "loc3"),
        ];
        assert!(validate_plan(&valid_plan).is_ok());

        let invalid_plan = vec![
            PlanItem::action("", vec![]), // Empty name
        ];
        assert!(validate_plan(&invalid_plan).is_err());
    }

    #[test]
    fn test_plan_equality() {
        let plan1 = vec![
            create_move_action("obj1", "loc2"),
            create_move_action("obj2", "loc3"),
        ];
        let plan2 = vec![
            create_move_action("obj1", "loc2"),
            create_move_action("obj2", "loc3"),
        ];
        let plan3 = vec![
            create_move_action("obj1", "loc3"), // Different destination
            create_move_action("obj2", "loc3"),
        ];

        assert!(plans_equal(&plan1, &plan2));
        assert!(!plans_equal(&plan1, &plan3));
        assert!(!plans_equal(&plan1, &[]));
    }

    #[test]
    fn test_helper_functions() {
        let travel_task = create_travel_task("alice", "home", "park");
        assert_eq!(travel_task.name(), "travel");
        assert!(travel_task.is_task());

        let transport_task = create_transport_task("obj1", "loc2");
        assert_eq!(transport_task.name(), "transport");
        assert!(transport_task.is_task());

        let move_action = create_move_action("obj1", "loc2");
        assert_eq!(move_action.name(), "move");
        assert!(move_action.is_action());

        let location_goal = create_location_goal("alice", "park");
        assert_eq!(location_goal.name(), "loc");
        assert!(location_goal.is_unigoal());
    }
}
