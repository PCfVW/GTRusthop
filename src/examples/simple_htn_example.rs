//! Simple HTN example

use crate::core::{State, PlanItem, string_value};
use crate::domains::create_simple_htn_domain;
use crate::planning::{PlannerBuilder, PlanningStrategy};
use crate::error::Result;

/// Run simple HTN examples
pub fn run_simple_htn_examples() -> Result<()> {
    println!("=== Running Simple HTN Examples ===");

    // Create domain
    let domain = create_simple_htn_domain()?;
    println!("Created simple HTN domain with {} actions", domain.action_names().len());

    // Create planner with builder pattern
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_strategy(PlanningStrategy::Iterative)
        .with_verbose_level(1)?
        .build()?;

    // Create initial state
    let mut state = State::new("initial_state");
    state.set_var("loc", "alice", string_value("home_a"));
    state.set_var("loc", "taxi1", string_value("station"));
    state.set_var("cash", "alice", 50.0.into());

    println!("\nInitial state:");
    state.display(None);

    // Example 1: Travel by walking (short distance)
    println!("\n--- Example 1: Travel by walking ---");
    let todo_list = vec![
        PlanItem::task("travel", vec![
            string_value("alice"),
            string_value("home_a"),
            string_value("park")
        ])
    ];

    match planner.find_plan(state.clone(), todo_list)? {
        Some(plan) => {
            println!("Found plan with {} actions:", plan.len());
            for (i, action) in plan.iter().enumerate() {
                println!("  {}: {}", i + 1, action);
            }
        }
        None => println!("No plan found"),
    }

    // Example 2: Travel by taxi (long distance)
    println!("\n--- Example 2: Travel by taxi ---");
    let todo_list = vec![
        PlanItem::task("travel", vec![
            string_value("alice"),
            string_value("home_a"),
            string_value("home_b")
        ])
    ];

    match planner.find_plan(state.clone(), todo_list)? {
        Some(plan) => {
            println!("Found plan with {} actions:", plan.len());
            for (i, action) in plan.iter().enumerate() {
                println!("  {}: {}", i + 1, action);
            }
        }
        None => println!("No plan found"),
    }

    // Example 3: Already at destination
    println!("\n--- Example 3: Already at destination ---");
    let todo_list = vec![
        PlanItem::task("travel", vec![
            string_value("alice"),
            string_value("home_a"),
            string_value("home_a")
        ])
    ];

    match planner.find_plan(state.clone(), todo_list)? {
        Some(plan) => {
            println!("Found plan with {} actions:", plan.len());
            for (i, action) in plan.iter().enumerate() {
                println!("  {}: {}", i + 1, action);
            }
        }
        None => println!("No plan found"),
    }

    println!("\n=== Simple HTN Examples Completed ===");
    Ok(())
}

/// Run Pyhop compatibility examples
///
/// This function demonstrates the `pyhop()` function that provides backward compatibility
/// with the original Pyhop planner. It mirrors the Python `pyhop_simple_travel_example.py`.
pub fn run_pyhop_simple_travel_example() -> Result<()> {
    use crate::planning::{pyhop, set_verbose_level};
    use crate::domains::create_simple_htn_domain;

    println!("********************************************************************************");
    println!("Call pyhop(domain, state, todo_list) with different verbosity levels");
    println!("********************************************************************************");

    // Create domain
    let domain = create_simple_htn_domain()?;

    // Create initial state similar to Python version
    let mut state = State::new("state1");
    state.set_var("loc", "me", string_value("home"));
    state.set_var("cash", "me", 20.0.into());
    state.set_var("owe", "me", 0.0.into());

    // Create todo list for travel task
    let todo_list = vec![
        PlanItem::task("travel", vec![
            string_value("me"),
            string_value("home"),
            string_value("park")
        ])
    ];

    println!("- If verbose=0, GTRusthop returns the solution but prints nothing.\n");
    set_verbose_level(0)?;
    let _result = pyhop(domain.clone(), state.clone(), todo_list.clone())?;

    println!("- If verbose=1, GTRusthop prints the problem and solution, and returns the solution:");
    set_verbose_level(1)?;
    let _result = pyhop(domain.clone(), state.clone(), todo_list.clone())?;

    println!("- If verbose=2, GTRusthop also prints a note at each recursive call:");
    set_verbose_level(2)?;
    let _result = pyhop(domain.clone(), state.clone(), todo_list.clone())?;

    println!("- If verbose=3, GTRusthop also prints the intermediate states:");
    set_verbose_level(3)?;
    let _result = pyhop(domain, state, todo_list)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple_htn_examples() {
        assert!(run_simple_htn_examples().is_ok());
    }

    #[test]
    fn test_run_pyhop_simple_travel_example() {
        assert!(run_pyhop_simple_travel_example().is_ok());
    }

    #[test]
    fn test_pyhop_function() -> Result<()> {
        use crate::planning::{pyhop, set_verbose_level};
        use crate::domains::create_simple_htn_domain;

        // Create domain and state
        let domain = create_simple_htn_domain()?;
        let mut state = State::new("test_state");
        state.set_var("loc", "alice", string_value("home_a"));
        state.set_var("cash", "alice", 50.0.into());

        // Create todo list
        let todo_list = vec![
            PlanItem::task("travel", vec![
                string_value("alice"),
                string_value("home_a"),
                string_value("park")
            ])
        ];

        // Test pyhop function with verbose=0 (no output)
        set_verbose_level(0)?;
        let plan = pyhop(domain, state, todo_list)?;

        // Should find a plan
        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert!(!plan.is_empty());

        Ok(())
    }

    #[test]
    fn test_planner_pyhop_method() -> Result<()> {
        use crate::planning::PlannerBuilder;
        use crate::domains::create_simple_htn_domain;

        // Create domain and planner
        let domain = create_simple_htn_domain()?;
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;

        // Create state
        let mut state = State::new("test_state");
        state.set_var("loc", "alice", string_value("home_a"));
        state.set_var("cash", "alice", 50.0.into());

        // Create todo list
        let todo_list = vec![
            PlanItem::task("travel", vec![
                string_value("alice"),
                string_value("home_a"),
                string_value("park")
            ])
        ];

        // Test planner.pyhop() method
        let plan = planner.pyhop(state, todo_list)?;

        // Should find a plan
        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert!(!plan.is_empty());

        Ok(())
    }
}
