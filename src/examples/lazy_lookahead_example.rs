//! Lazy Lookahead Example for GTRusthop
//! 
//! This example demonstrates the run_lazy_lookahead algorithm, which combines
//! planning and acting. It shows how commands can fail during execution,
//! triggering replanning.

use crate::core::{State, Domain, PlanItem, string_value};
use crate::planning::PlannerBuilder;
use crate::error::Result;

/// Run lazy lookahead examples
pub fn run_lazy_lookahead_examples() -> Result<()> {
    println!("=== Running Lazy Lookahead Examples ===");

    // Create domain with actions and commands
    let domain = create_taxi_domain()?;
    
    // Create planner
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Create initial state
    let mut state = State::new("initial_state");
    state.set_var("loc", "alice", string_value("home_a"));
    state.set_var("loc", "taxi1", string_value("station"));
    state.set_var("cash", "alice", 50.0.into());
    
    println!("\nInitial state:");
    state.display(None);

    // Example 1: Successful execution
    println!("\n--- Example 1: Successful execution ---");
    let todo_list = vec![PlanItem::task("travel", vec![
        string_value("alice"),
        string_value("home_a"),
        string_value("park")
    ])];
    
    let final_state = planner.run_lazy_lookahead(state.clone(), todo_list, 5)?;
    println!("\nFinal state after successful execution:");
    final_state.display(None);

    // Example 2: Command failure and replanning
    println!("\n--- Example 2: Command failure and replanning ---");
    let unreliable_domain = create_unreliable_taxi_domain()?;
    let unreliable_planner = PlannerBuilder::new()
        .with_domain(unreliable_domain)
        .with_verbose_level(1)?
        .build()?;

    // Create a state with limited cash to force walking after taxi fails
    let mut limited_cash_state = State::new("limited_cash_state");
    limited_cash_state.set_var("loc", "alice", string_value("home_a"));
    limited_cash_state.set_var("loc", "taxi1", string_value("station"));
    limited_cash_state.set_var("cash", "alice", 5.0.into()); // Not enough for taxi

    let todo_list = vec![PlanItem::task("travel", vec![
        string_value("alice"),
        string_value("home_a"),
        string_value("park")
    ])];

    let final_state = unreliable_planner.run_lazy_lookahead(limited_cash_state, todo_list, 5)?;
    println!("\nFinal state after unreliable execution:");
    final_state.display(None);

    println!("\n=== Lazy Lookahead Examples Completed ===");
    Ok(())
}

/// Create a taxi domain with reliable commands
fn create_taxi_domain() -> Result<Domain> {
    let mut domain = Domain::new("taxi_domain");
    
    // Actions (for planning)
    domain.declare_action("walk", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                if let Some(current_loc) = state.get_var("loc", person) {
                    if current_loc.as_str() == Some(from) {
                        state.set_var("loc", person, string_value(to));
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    domain.declare_action("call_taxi", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(_person), Some(_location)) = (args[0].as_str(), args[1].as_str()) {
                // For planning purposes, assume taxi always comes
                return Some(state.clone());
            }
        }
        None
    })?;

    domain.declare_action("ride_taxi", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                if let Some(current_loc) = state.get_var("loc", person) {
                    if current_loc.as_str() == Some(from) {
                        state.set_var("loc", person, string_value(to));
                        // Deduct taxi fare
                        if let Some(cash) = state.get_var("cash", person) {
                            if let Some(cash_amount) = cash.as_f64() {
                                state.set_var("cash", person, (cash_amount - 10.0).into());
                            }
                        }
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    // Commands (for execution) - same as actions in this simple case
    domain.declare_command("c_walk", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                if let Some(current_loc) = state.get_var("loc", person) {
                    if current_loc.as_str() == Some(from) {
                        state.set_var("loc", person, string_value(to));
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    domain.declare_command("c_call_taxi", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            // Command always succeeds (taxi always comes)
            return Some(state.clone());
        }
        None
    })?;

    domain.declare_command("c_ride_taxi", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                if let Some(current_loc) = state.get_var("loc", person) {
                    if current_loc.as_str() == Some(from) {
                        state.set_var("loc", person, string_value(to));
                        // Deduct taxi fare
                        if let Some(cash) = state.get_var("cash", person) {
                            if let Some(cash_amount) = cash.as_f64() {
                                state.set_var("cash", person, (cash_amount - 10.0).into());
                            }
                        }
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    // Task methods
    domain.declare_task_method("travel", |state: &State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(_from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                // Check current location of person
                if let Some(current_loc) = state.get_var("loc", person) {
                    if let Some(current_loc_str) = current_loc.as_str() {
                        if current_loc_str == to {
                            return Some(vec![]); // Already at destination
                        }

                        // Use current location instead of 'from' parameter
                        let actual_from = current_loc_str;

                        // Check if person has enough cash for taxi
                        if let Some(cash) = state.get_var("cash", person) {
                            if let Some(cash_amount) = cash.as_f64() {
                                if cash_amount >= 10.0 {
                                    // Use taxi
                                    return Some(vec![
                                        PlanItem::action("call_taxi", vec![string_value(person), string_value(actual_from)]),
                                        PlanItem::action("ride_taxi", vec![string_value(person), string_value(actual_from), string_value(to)])
                                    ]);
                                }
                            }
                        }

                        // Fall back to walking
                        return Some(vec![
                            PlanItem::action("walk", vec![string_value(person), string_value(actual_from), string_value(to)])
                        ]);
                    }
                }
            }
        }
        None
    })?;

    Ok(domain)
}

/// Create a taxi domain with unreliable commands that sometimes fail
fn create_unreliable_taxi_domain() -> Result<Domain> {
    let mut domain = Domain::new("unreliable_taxi_domain");
    
    // Actions (for planning) - same as reliable domain
    domain.declare_action("walk", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                if let Some(current_loc) = state.get_var("loc", person) {
                    if current_loc.as_str() == Some(from) {
                        state.set_var("loc", person, string_value(to));
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    domain.declare_action("call_taxi", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            return Some(state.clone());
        }
        None
    })?;

    domain.declare_action("ride_taxi", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                if let Some(current_loc) = state.get_var("loc", person) {
                    if current_loc.as_str() == Some(from) {
                        state.set_var("loc", person, string_value(to));
                        if let Some(cash) = state.get_var("cash", person) {
                            if let Some(cash_amount) = cash.as_f64() {
                                state.set_var("cash", person, (cash_amount - 10.0).into());
                            }
                        }
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    // Unreliable commands
    domain.declare_command("c_walk", |state: &mut State, args: &[crate::core::StateValue]| {
        // Walking always works
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                if let Some(current_loc) = state.get_var("loc", person) {
                    if current_loc.as_str() == Some(from) {
                        state.set_var("loc", person, string_value(to));
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    domain.declare_command("c_call_taxi", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            // Check if this is the first attempt (person still at home_a)
            if let Some(person_arg) = args[0].as_str() {
                if let Some(current_loc) = state.get_var("loc", person_arg) {
                    if let Some(loc_str) = current_loc.as_str() {
                        if loc_str == "home_a" {
                            // First attempt fails
                            return None;
                        }
                    }
                }
            }
            // Subsequent attempts succeed
            return Some(state.clone());
        }
        None
    })?;

    domain.declare_command("c_ride_taxi", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                if let Some(current_loc) = state.get_var("loc", person) {
                    if current_loc.as_str() == Some(from) {
                        state.set_var("loc", person, string_value(to));
                        if let Some(cash) = state.get_var("cash", person) {
                            if let Some(cash_amount) = cash.as_f64() {
                                state.set_var("cash", person, (cash_amount - 10.0).into());
                            }
                        }
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    // Task method with taxi preference but walking fallback
    domain.declare_task_method("travel", |state: &State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(_from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                // Check current location of person
                if let Some(current_loc) = state.get_var("loc", person) {
                    if let Some(current_loc_str) = current_loc.as_str() {
                        if current_loc_str == to {
                            return Some(vec![]); // Already at destination
                        }

                        // Use current location instead of 'from' parameter
                        let actual_from = current_loc_str;

                        if let Some(cash) = state.get_var("cash", person) {
                            if let Some(cash_amount) = cash.as_f64() {
                                if cash_amount >= 10.0 {
                                    return Some(vec![
                                        PlanItem::action("call_taxi", vec![string_value(person), string_value(actual_from)]),
                                        PlanItem::action("ride_taxi", vec![string_value(person), string_value(actual_from), string_value(to)])
                                    ]);
                                }
                            }
                        }

                        return Some(vec![
                            PlanItem::action("walk", vec![string_value(person), string_value(actual_from), string_value(to)])
                        ]);
                    }
                }
            }
        }
        None
    })?;

    // Alternative method: just walk (lower priority)
    domain.declare_task_method("travel", |state: &State, args: &[crate::core::StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(_from), Some(to)) = (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
                // Check current location of person
                if let Some(current_loc) = state.get_var("loc", person) {
                    if let Some(current_loc_str) = current_loc.as_str() {
                        if current_loc_str == to {
                            return Some(vec![]); // Already at destination
                        }

                        // Use current location instead of 'from' parameter
                        let actual_from = current_loc_str;

                        return Some(vec![
                            PlanItem::action("walk", vec![string_value(person), string_value(actual_from), string_value(to)])
                        ]);
                    }
                }
            }
        }
        None
    })?;

    Ok(domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_lazy_lookahead_examples() -> Result<()> {
        run_lazy_lookahead_examples()
    }
}
