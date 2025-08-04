//! Goal verification functions for GTRusthop

use crate::core::{State, Multigoal, StateValue, TodoList};
use crate::error::{GTRustHopError, Result};
use super::{is_verbose, verbose_print};

/// Verify that a unigoal method achieved its goal
pub fn verify_unigoal(
    state: &State,
    method_name: &str,
    var_name: &str,
    arg: &str,
    desired_value: &StateValue,
    depth: usize,
) -> Result<TodoList> {
    if !state.satisfies_unigoal(var_name, arg, desired_value) {
        return Err(GTRustHopError::method_verification_failed(
            method_name,
            format!("{var_name}[{arg}] = {desired_value}"),
            depth,
        ));
    }
    
    if is_verbose(3) {
        verbose_print(3, &format!(
            "depth {depth}: method {method_name} achieved goal {var_name}[{arg}] = {desired_value}"
        ));
    }
    
    Ok(vec![]) // No subtasks needed
}

/// Verify that a multigoal method achieved its multigoal
pub fn verify_multigoal(
    state: &State,
    method_name: &str,
    multigoal: &Multigoal,
    depth: usize,
) -> Result<TodoList> {
    let unsatisfied = multigoal.unsatisfied_goals(state);
    
    if !unsatisfied.is_empty() {
        return Err(GTRustHopError::multigoal_verification_failed(
            method_name,
            format!("{multigoal}"),
            depth,
        ));
    }
    
    if is_verbose(3) {
        verbose_print(3, &format!(
            "depth {depth}: method {method_name} achieved {multigoal}"
        ));
    }
    
    Ok(vec![]) // No subtasks needed
}

/// Check which goals in a multigoal are not achieved by the current state
pub fn goals_not_achieved(state: &State, multigoal: &Multigoal) -> std::collections::HashMap<String, std::collections::HashMap<String, StateValue>> {
    multigoal.unsatisfied_goals(state)
}

/// Built-in verification task method for unigoals
#[allow(clippy::manual_map)]
pub fn m_verify_g(state: &State, args: &[StateValue]) -> Option<TodoList> {
    if args.len() >= 5 {
        if let (
            Some(method_name),
            Some(var_name),
            Some(arg),
            desired_value,
            Some(depth_val)
        ) = (
            args[0].as_str(),
            args[1].as_str(),
            args[2].as_str(),
            &args[3],
            args[4].as_u64()
        ) {
            let depth = depth_val as usize;
            
            verify_unigoal(state, method_name, var_name, arg, desired_value, depth).ok()
        } else {
            None
        }
    } else {
        None
    }
}

/// Built-in verification task method for multigoals
#[allow(clippy::manual_map)]
pub fn m_verify_mg(state: &State, args: &[StateValue]) -> Option<TodoList> {
    if args.len() >= 3 {
        if let (
            Some(method_name),
            multigoal_value,
            Some(depth_val)
        ) = (
            args[0].as_str(),
            &args[1],
            args[2].as_u64()
        ) {
            let depth = depth_val as usize;
            
            // Try to deserialize the multigoal from the value
            if let Ok(multigoal) = serde_json::from_value::<Multigoal>(multigoal_value.clone()) {
                verify_multigoal(state, method_name, &multigoal, depth).ok()
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// Helper function to create verification tasks for unigoals
pub fn create_unigoal_verification_task(
    method_name: &str,
    var_name: &str,
    arg: &str,
    desired_value: &StateValue,
    depth: usize,
) -> crate::core::PlanItem {
    crate::core::PlanItem::task("_verify_g", vec![
        method_name.into(),
        var_name.into(),
        arg.into(),
        desired_value.clone(),
        (depth as i64).into(),
    ])
}

/// Helper function to create verification tasks for multigoals
pub fn create_multigoal_verification_task(
    method_name: &str,
    multigoal: &Multigoal,
    depth: usize,
) -> Result<crate::core::PlanItem> {
    let multigoal_value = serde_json::to_value(multigoal)
        .map_err(|e| GTRustHopError::generic(format!("Failed to serialize multigoal: {e}")))?;
    
    Ok(crate::core::PlanItem::task("_verify_mg", vec![
        method_name.into(),
        multigoal_value,
        (depth as i64).into(),
    ]))
}

/// Check if verification is enabled globally
pub fn is_verification_enabled() -> bool {
    // For now, always return true. In a full implementation,
    // this would check a global setting.
    true
}

/// Set whether verification is enabled globally
pub fn set_verification_enabled(_enabled: bool) {
    // In a full implementation, this would set a global flag
    // For now, we'll just ignore it since verification is always enabled
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{State, Multigoal};

    #[test]
    fn test_unigoal_verification_success() {
        let mut state = State::new("test_state");
        state.set_var("loc", "alice", "park".into());
        
        let result = verify_unigoal(&state, "test_method", "loc", "alice", &"park".into(), 0);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_unigoal_verification_failure() {
        let mut state = State::new("test_state");
        state.set_var("loc", "alice", "home".into());
        
        let result = verify_unigoal(&state, "test_method", "loc", "alice", &"park".into(), 0);
        assert!(result.is_err());
        
        if let Err(GTRustHopError::MethodVerificationFailed { method, goal, depth }) = result {
            assert_eq!(method, "test_method");
            assert_eq!(goal, "loc[alice] = \"park\"");
            assert_eq!(depth, 0);
        } else {
            panic!("Expected MethodVerificationFailed error");
        }
    }

    #[test]
    fn test_multigoal_verification_success() {
        let mut state = State::new("test_state");
        state.set_var("loc", "alice", "park".into());
        state.set_var("loc", "bob", "home".into());
        
        let mut multigoal = Multigoal::new("test_goal");
        multigoal.set_goal("loc", "alice", "park".into());
        multigoal.set_goal("loc", "bob", "home".into());
        
        let result = verify_multigoal(&state, "test_method", &multigoal, 0);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_multigoal_verification_failure() {
        let mut state = State::new("test_state");
        state.set_var("loc", "alice", "home".into()); // Wrong location
        state.set_var("loc", "bob", "home".into());
        
        let mut multigoal = Multigoal::new("test_goal");
        multigoal.set_goal("loc", "alice", "park".into());
        multigoal.set_goal("loc", "bob", "home".into());
        
        let result = verify_multigoal(&state, "test_method", &multigoal, 0);
        assert!(result.is_err());
        
        if let Err(GTRustHopError::MultigoalVerificationFailed { method, multigoal: _mg, depth }) = result {
            assert_eq!(method, "test_method");
            assert_eq!(depth, 0);
        } else {
            panic!("Expected MultigoalVerificationFailed error");
        }
    }

    #[test]
    fn test_goals_not_achieved() {
        let mut state = State::new("test_state");
        state.set_var("loc", "alice", "home".into());
        state.set_var("loc", "bob", "home".into());
        
        let mut multigoal = Multigoal::new("test_goal");
        multigoal.set_goal("loc", "alice", "park".into()); // Not achieved
        multigoal.set_goal("loc", "bob", "home".into());   // Achieved
        multigoal.set_goal("cash", "alice", 50.into());    // Not achieved (doesn't exist)
        
        let unachieved = goals_not_achieved(&state, &multigoal);
        
        assert_eq!(unachieved.len(), 2); // loc and cash
        assert!(unachieved.contains_key("loc"));
        assert!(unachieved.contains_key("cash"));
        assert_eq!(unachieved["loc"]["alice"], crate::core::string_value("park"));
        assert_eq!(unachieved["cash"]["alice"], crate::core::int_value(50));
    }

    #[test]
    fn test_verification_task_creation() {
        let task = create_unigoal_verification_task("test_method", "loc", "alice", &"park".into(), 5);
        
        if let crate::core::PlanItem::Task(name, args) = task {
            assert_eq!(name, "_verify_g");
            assert_eq!(args.len(), 5);
            assert_eq!(args[0].as_str(), Some("test_method"));
            assert_eq!(args[1].as_str(), Some("loc"));
            assert_eq!(args[2].as_str(), Some("alice"));
            assert_eq!(args[3], crate::core::string_value("park"));
            assert_eq!(args[4].as_i64(), Some(5));
        } else {
            panic!("Expected Task plan item");
        }
    }

    #[test]
    fn test_multigoal_verification_task_creation() {
        let mut multigoal = Multigoal::new("test_goal");
        multigoal.set_goal("loc", "alice", "park".into());
        
        let task = create_multigoal_verification_task("test_method", &multigoal, 3).unwrap();
        
        if let crate::core::PlanItem::Task(name, args) = task {
            assert_eq!(name, "_verify_mg");
            assert_eq!(args.len(), 3);
            assert_eq!(args[0].as_str(), Some("test_method"));
            assert_eq!(args[2].as_i64(), Some(3));
            
            // Verify we can deserialize the multigoal back
            let deserialized: Multigoal = serde_json::from_value(args[1].clone()).unwrap();
            assert_eq!(deserialized.name, "test_goal");
        } else {
            panic!("Expected Task plan item");
        }
    }
}
