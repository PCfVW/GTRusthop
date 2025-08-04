//! Simple HTN domain implementation

use crate::core::{Domain, State, StateValue, PlanItem, string_value};
use crate::error::Result;
use super::{get_state_var_string, get_state_var_f64};

/// Create the simple HTN domain
pub fn create_simple_htn_domain() -> Result<Domain> {
    let mut domain = Domain::new("simple_htn");

    // Declare actions
    domain.declare_action("walk", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (
                args[0].as_str(),
                args[1].as_str(),
                args[2].as_str()
            ) {
                // Check if person is at 'from' location
                if get_state_var_string(state, "loc", person) == Some(from.to_string()) {
                    state.set_var("loc", person, string_value(to));
                    return Some(state.clone());
                }
            }
        }
        None
    })?;

    domain.declare_action("call_taxi", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(_person), Some(location)) = (
                args[0].as_str(),
                args[1].as_str()
            ) {
                // Find an available taxi
                if let Some(_taxi_loc) = get_state_var_string(state, "loc", "taxi1") {
                    state.set_var("loc", "taxi1", string_value(location));
                    return Some(state.clone());
                }
            }
        }
        None
    })?;

    domain.declare_action("ride_taxi", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (
                args[0].as_str(),
                args[1].as_str(),
                args[2].as_str()
            ) {
                // Check if person and taxi are at 'from' location
                let person_loc = get_state_var_string(state, "loc", person);
                let taxi_loc = get_state_var_string(state, "loc", "taxi1");

                if person_loc == Some(from.to_string()) && taxi_loc == Some(from.to_string()) {
                    state.set_var("loc", person, string_value(to));
                    state.set_var("loc", "taxi1", string_value(to));

                    // Deduct taxi fare
                    if let Some(cash) = get_state_var_f64(state, "cash", person) {
                        let fare = 10.0; // Simple fixed fare
                        state.set_var("cash", person, (cash - fare).into());
                    }

                    return Some(state.clone());
                }
            }
        }
        None
    })?;

    // Declare task methods
    domain.declare_task_method("travel", |state: &State, args: &[StateValue]| {
        if args.len() >= 3 {
            if let (Some(person), Some(from), Some(to)) = (
                args[0].as_str(),
                args[1].as_str(),
                args[2].as_str()
            ) {
                // Check if already at destination
                if get_state_var_string(state, "loc", person) == Some(to.to_string()) {
                    return Some(vec![]); // No actions needed
                }

                // Method 1: Walk if close
                if from == "home_a" && to == "park" {
                    return Some(vec![
                        PlanItem::action("walk", vec![
                            string_value(person),
                            string_value(from),
                            string_value(to)
                        ])
                    ]);
                }

                // Method 2: Take taxi
                return Some(vec![
                    PlanItem::action("call_taxi", vec![
                        string_value(person),
                        string_value(from)
                    ]),
                    PlanItem::action("ride_taxi", vec![
                        string_value(person),
                        string_value(from),
                        string_value(to)
                    ])
                ]);
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
    fn test_create_simple_htn_domain() {
        let domain = create_simple_htn_domain().unwrap();
        assert_eq!(domain.name, "simple_htn");
    }
}
