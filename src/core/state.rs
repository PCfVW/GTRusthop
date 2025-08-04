//! State representation for GTRusthop

use super::StateValue;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a state in the planning domain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    /// Name of the state
    pub name: String,
    /// State variables as nested maps: var_name -> arg -> value
    variables: IndexMap<String, HashMap<String, StateValue>>,
    /// Copy counter for generating unique names
    copy_counter: usize,
}

impl State {
    /// Create a new state with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variables: IndexMap::new(),
            copy_counter: 0,
        }
    }

    /// Set a state variable value
    pub fn set_var(&mut self, var_name: impl Into<String>, arg: impl Into<String>, value: StateValue) {
        let var_name = var_name.into();
        let arg = arg.into();
        
        self.variables
            .entry(var_name)
            .or_default()
            .insert(arg, value);
    }

    /// Get a state variable value
    pub fn get_var(&self, var_name: &str, arg: &str) -> Option<&StateValue> {
        self.variables
            .get(var_name)
            .and_then(|var_map| var_map.get(arg))
    }

    /// Get a mutable reference to a state variable value
    pub fn get_var_mut(&mut self, var_name: &str, arg: &str) -> Option<&mut StateValue> {
        self.variables
            .get_mut(var_name)
            .and_then(|var_map| var_map.get_mut(arg))
    }

    /// Check if a state variable exists
    pub fn has_var(&self, var_name: &str) -> bool {
        self.variables.contains_key(var_name)
    }

    /// Check if a state variable argument exists
    pub fn has_var_arg(&self, var_name: &str, arg: &str) -> bool {
        self.variables
            .get(var_name)
            .is_some_and(|var_map| var_map.contains_key(arg))
    }

    /// Get all state variable names
    pub fn var_names(&self) -> Vec<&String> {
        self.variables.keys().collect()
    }

    /// Get all arguments for a state variable
    pub fn var_args(&self, var_name: &str) -> Option<Vec<&String>> {
        self.variables
            .get(var_name)
            .map(|var_map| var_map.keys().collect())
    }

    /// Get the entire variable map for a state variable
    pub fn get_var_map(&self, var_name: &str) -> Option<&HashMap<String, StateValue>> {
        self.variables.get(var_name)
    }

    /// Set an entire variable map for a state variable
    pub fn set_var_map(&mut self, var_name: impl Into<String>, var_map: HashMap<String, StateValue>) {
        self.variables.insert(var_name.into(), var_map);
    }

    /// Create a deep copy of the state with an optional new name
    pub fn copy(&self, new_name: Option<String>) -> Self {
        let mut copy = self.clone();
        
        if let Some(name) = new_name {
            copy.name = name;
        } else {
            copy.name = format!("{}_copy_{}", self.name, self.copy_counter);
            copy.copy_counter += 1;
        }
        
        copy
    }

    /// Display the state in a human-readable format
    pub fn display(&self, heading: Option<&str>) {
        let heading = heading.unwrap_or("State");
        let title = format!("{} {}:", heading, self.name);
        let dashes = "-".repeat(title.len());
        
        println!("{title}");
        println!("{dashes}");
        
        if self.variables.is_empty() {
            println!("  (no state variables)");
        } else {
            for (var_name, var_map) in &self.variables {
                if var_map.is_empty() {
                    println!("  - {var_name} = {{}}");
                } else {
                    println!("  - {var_name} = {{");
                    for (arg, value) in var_map {
                        println!("      '{arg}': {value},");
                    }
                    println!("    }}");
                }
            }
        }
        println!();
    }

    /// Check if this state satisfies a unigoal
    pub fn satisfies_unigoal(&self, var_name: &str, arg: &str, desired_value: &StateValue) -> bool {
        self.get_var(var_name, arg) == Some(desired_value)
    }

    /// Get all state variables that don't match the desired values in a multigoal
    pub fn unsatisfied_goals(&self, multigoal: &crate::core::Multigoal) -> HashMap<String, HashMap<String, StateValue>> {
        let mut unsatisfied = HashMap::new();
        
        for (var_name, desired_map) in &multigoal.variables {
            for (arg, desired_value) in desired_map {
                if !self.satisfies_unigoal(var_name, arg, desired_value) {
                    unsatisfied
                        .entry(var_name.clone())
                        .or_insert_with(HashMap::new)
                        .insert(arg.clone(), desired_value.clone());
                }
            }
        }
        
        unsatisfied
    }

    /// Apply changes from another state (for action execution)
    pub fn apply_changes(&mut self, other: &State) {
        for (var_name, var_map) in &other.variables {
            for (arg, value) in var_map {
                self.set_var(var_name, arg, value.clone());
            }
        }
    }

    /// Convert to a JSON representation
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Create from a JSON representation
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<State {}>", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_creation() {
        let state = State::new("test_state");
        assert_eq!(state.name, "test_state");
        assert!(state.variables.is_empty());
    }

    #[test]
    fn test_state_variables() {
        let mut state = State::new("test");
        
        // Set some variables
        state.set_var("loc", "alice", "home".into());
        state.set_var("loc", "bob", "park".into());
        state.set_var("cash", "alice", 20.into());
        
        // Test getting variables
        assert_eq!(state.get_var("loc", "alice"), Some(&"home".into()));
        assert_eq!(state.get_var("loc", "bob"), Some(&"park".into()));
        assert_eq!(state.get_var("cash", "alice"), Some(&20.into()));
        assert_eq!(state.get_var("loc", "charlie"), None);
        
        // Test existence checks
        assert!(state.has_var("loc"));
        assert!(state.has_var_arg("loc", "alice"));
        assert!(!state.has_var_arg("loc", "charlie"));
        assert!(!state.has_var("nonexistent"));
    }

    #[test]
    fn test_state_copy() {
        let mut state = State::new("original");
        state.set_var("loc", "alice", "home".into());
        
        let copy1 = state.copy(None);
        assert_eq!(copy1.name, "original_copy_0");
        assert_eq!(copy1.get_var("loc", "alice"), Some(&"home".into()));
        
        let copy2 = state.copy(Some("custom_name".to_string()));
        assert_eq!(copy2.name, "custom_name");
        assert_eq!(copy2.get_var("loc", "alice"), Some(&"home".into()));
    }

    #[test]
    fn test_unigoal_satisfaction() {
        let mut state = State::new("test");
        state.set_var("loc", "alice", "home".into());
        
        assert!(state.satisfies_unigoal("loc", "alice", &"home".into()));
        assert!(!state.satisfies_unigoal("loc", "alice", &"park".into()));
        assert!(!state.satisfies_unigoal("loc", "bob", &"home".into()));
    }
}
