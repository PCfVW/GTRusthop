//! Domain implementations for GTRusthop

pub mod simple_htn;
pub mod simple_hgn;
pub mod blocks_htn;

// Re-export common domain utilities
pub use simple_htn::create_simple_htn_domain;
pub use simple_hgn::create_simple_hgn_domain;
pub use blocks_htn::create_blocks_htn_domain;

use crate::core::{State, StateValue};

/// Helper function to check if a variable is of a specific type
pub fn is_a(variable: &str, var_type: &str, rigid_types: &std::collections::HashMap<String, Vec<String>>) -> bool {
    rigid_types
        .get(var_type)
        .is_some_and(|type_list| type_list.contains(&variable.to_string()))
}

/// Helper function to get distance between two locations
pub fn distance(
    x: &str,
    y: &str,
    distances: &std::collections::HashMap<(String, String), f64>,
) -> Option<f64> {
    distances
        .get(&(x.to_string(), y.to_string()))
        .or_else(|| distances.get(&(y.to_string(), x.to_string())))
        .copied()
}

/// Helper function to calculate taxi rate based on distance
pub fn taxi_rate(dist: f64) -> f64 {
    1.5 + 0.5 * dist
}

/// Create a rigid relations state for travel domains
pub fn create_rigid_relations() -> State {
    let mut rigid = State::new("rigid_relations");
    
    // Types
    let mut types = std::collections::HashMap::new();
    types.insert("person".to_string(), vec!["alice".to_string(), "bob".to_string()]);
    types.insert("location".to_string(), vec![
        "home_a".to_string(),
        "home_b".to_string(),
        "park".to_string(),
        "station".to_string(),
    ]);
    types.insert("taxi".to_string(), vec!["taxi1".to_string(), "taxi2".to_string()]);
    
    rigid.set_var("types", "person", serde_json::to_value(&types["person"]).unwrap());
    rigid.set_var("types", "location", serde_json::to_value(&types["location"]).unwrap());
    rigid.set_var("types", "taxi", serde_json::to_value(&types["taxi"]).unwrap());
    
    // Distances
    let mut distances = std::collections::HashMap::new();
    distances.insert(("home_a".to_string(), "park".to_string()), 8.0);
    distances.insert(("home_b".to_string(), "park".to_string()), 2.0);
    distances.insert(("station".to_string(), "home_a".to_string()), 1.0);
    distances.insert(("station".to_string(), "home_b".to_string()), 7.0);
    distances.insert(("home_a".to_string(), "home_b".to_string()), 7.0);
    distances.insert(("station".to_string(), "park".to_string()), 9.0);
    
    for ((from, to), dist) in distances {
        rigid.set_var("dist", format!("{from}_{to}"), dist.into());
    }
    
    rigid
}

/// Create a prototypical initial state for travel domains
pub fn create_initial_state() -> State {
    let mut state = State::new("state0");
    
    // Locations
    state.set_var("loc", "alice", "home_a".into());
    state.set_var("loc", "bob", "home_b".into());
    state.set_var("loc", "taxi1", "park".into());
    state.set_var("loc", "taxi2", "station".into());
    
    // Cash
    state.set_var("cash", "alice", 20.into());
    state.set_var("cash", "bob", 15.into());
    
    // Owe
    state.set_var("owe", "alice", 0.into());
    state.set_var("owe", "bob", 0.into());
    
    state
}

/// Helper function to extract string from StateValue
pub fn state_value_as_string(value: &StateValue) -> Option<String> {
    match value {
        StateValue::String(s) => Some(s.clone()),
        _ => None,
    }
}

/// Helper function to extract number from StateValue
pub fn state_value_as_f64(value: &StateValue) -> Option<f64> {
    match value {
        StateValue::Number(n) => n.as_f64(),
        _ => None,
    }
}

/// Helper function to extract integer from StateValue
pub fn state_value_as_i64(value: &StateValue) -> Option<i64> {
    match value {
        StateValue::Number(n) => n.as_i64(),
        _ => None,
    }
}

/// Helper function to extract boolean from StateValue
pub fn state_value_as_bool(value: &StateValue) -> Option<bool> {
    match value {
        StateValue::Bool(b) => Some(*b),
        _ => None,
    }
}

/// Helper function to check if a state variable has a specific value
pub fn state_var_equals(state: &State, var_name: &str, arg: &str, expected: &StateValue) -> bool {
    state.get_var(var_name, arg) == Some(expected)
}

/// Helper function to get a state variable as a string
pub fn get_state_var_string(state: &State, var_name: &str, arg: &str) -> Option<String> {
    state.get_var(var_name, arg).and_then(state_value_as_string)
}

/// Helper function to get a state variable as a number
pub fn get_state_var_f64(state: &State, var_name: &str, arg: &str) -> Option<f64> {
    state.get_var(var_name, arg).and_then(state_value_as_f64)
}

/// Helper function to get a state variable as an integer
pub fn get_state_var_i64(state: &State, var_name: &str, arg: &str) -> Option<i64> {
    state.get_var(var_name, arg).and_then(state_value_as_i64)
}

/// Helper function to get a state variable as a boolean
pub fn get_state_var_bool(state: &State, var_name: &str, arg: &str) -> Option<bool> {
    state.get_var(var_name, arg).and_then(state_value_as_bool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigid_relations_creation() {
        let rigid = create_rigid_relations();
        assert_eq!(rigid.name, "rigid_relations");
        assert!(rigid.has_var("types"));
        assert!(rigid.has_var("dist"));
    }

    #[test]
    fn test_initial_state_creation() {
        let state = create_initial_state();
        assert_eq!(state.name, "state0");
        
        assert_eq!(get_state_var_string(&state, "loc", "alice"), Some("home_a".to_string()));
        assert_eq!(get_state_var_string(&state, "loc", "bob"), Some("home_b".to_string()));
        assert_eq!(get_state_var_i64(&state, "cash", "alice"), Some(20));
        assert_eq!(get_state_var_i64(&state, "cash", "bob"), Some(15));
        assert_eq!(get_state_var_i64(&state, "owe", "alice"), Some(0));
        assert_eq!(get_state_var_i64(&state, "owe", "bob"), Some(0));
    }

    #[test]
    fn test_taxi_rate_calculation() {
        assert_eq!(taxi_rate(0.0), 1.5);
        assert_eq!(taxi_rate(1.0), 2.0);
        assert_eq!(taxi_rate(8.0), 5.5);
    }

    #[test]
    fn test_state_value_helpers() {
        assert_eq!(state_value_as_string(&"test".into()), Some("test".to_string()));
        assert_eq!(state_value_as_f64(&42.5.into()), Some(42.5));
        assert_eq!(state_value_as_i64(&42.into()), Some(42));
        assert_eq!(state_value_as_bool(&true.into()), Some(true));
        
        assert_eq!(state_value_as_string(&42.into()), None);
        assert_eq!(state_value_as_f64(&"test".into()), None);
    }
}
