//! Multigoal representation for GTRusthop

use super::StateValue;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a conjunctive goal (multigoal) in the planning domain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Multigoal {
    /// Name of the multigoal
    pub name: String,
    /// Goal variables as nested maps: var_name -> arg -> desired_value
    pub variables: IndexMap<String, HashMap<String, StateValue>>,
    /// Copy counter for generating unique names
    copy_counter: usize,
}

impl Multigoal {
    /// Create a new multigoal with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variables: IndexMap::new(),
            copy_counter: 0,
        }
    }

    /// Set a goal variable value
    pub fn set_goal(&mut self, var_name: impl Into<String>, arg: impl Into<String>, value: StateValue) {
        let var_name = var_name.into();
        let arg = arg.into();
        
        self.variables
            .entry(var_name)
            .or_default()
            .insert(arg, value);
    }

    /// Get a goal variable value
    pub fn get_goal(&self, var_name: &str, arg: &str) -> Option<&StateValue> {
        self.variables
            .get(var_name)
            .and_then(|var_map| var_map.get(arg))
    }

    /// Check if a goal variable exists
    pub fn has_goal_var(&self, var_name: &str) -> bool {
        self.variables.contains_key(var_name)
    }

    /// Check if a goal variable argument exists
    pub fn has_goal_arg(&self, var_name: &str, arg: &str) -> bool {
        self.variables
            .get(var_name)
            .is_some_and(|var_map| var_map.contains_key(arg))
    }

    /// Get all goal variable names
    pub fn goal_var_names(&self) -> Vec<&String> {
        self.variables.keys().collect()
    }

    /// Get all arguments for a goal variable
    pub fn goal_args(&self, var_name: &str) -> Option<Vec<&String>> {
        self.variables
            .get(var_name)
            .map(|var_map| var_map.keys().collect())
    }

    /// Get the entire variable map for a goal variable
    pub fn get_goal_map(&self, var_name: &str) -> Option<&HashMap<String, StateValue>> {
        self.variables.get(var_name)
    }

    /// Set an entire variable map for a goal variable
    pub fn set_goal_map(&mut self, var_name: impl Into<String>, var_map: HashMap<String, StateValue>) {
        self.variables.insert(var_name.into(), var_map);
    }

    /// Create a deep copy of the multigoal with an optional new name
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

    /// Display the multigoal in a human-readable format
    pub fn display(&self, heading: Option<&str>) {
        let heading = heading.unwrap_or("Multigoal");
        let title = format!("{} {}:", heading, self.name);
        let dashes = "-".repeat(title.len());
        
        println!("{title}");
        println!("{dashes}");
        
        if self.variables.is_empty() {
            println!("  (no goal variables)");
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

    /// Check if this multigoal is satisfied by the given state
    pub fn is_satisfied_by(&self, state: &crate::core::State) -> bool {
        for (var_name, goal_map) in &self.variables {
            for (arg, desired_value) in goal_map {
                if !state.satisfies_unigoal(var_name, arg, desired_value) {
                    return false;
                }
            }
        }
        true
    }

    /// Get all unsatisfied goals in this multigoal given a state
    pub fn unsatisfied_goals(&self, state: &crate::core::State) -> HashMap<String, HashMap<String, StateValue>> {
        let mut unsatisfied = HashMap::new();
        
        for (var_name, goal_map) in &self.variables {
            for (arg, desired_value) in goal_map {
                if !state.satisfies_unigoal(var_name, arg, desired_value) {
                    unsatisfied
                        .entry(var_name.clone())
                        .or_insert_with(HashMap::new)
                        .insert(arg.clone(), desired_value.clone());
                }
            }
        }
        
        unsatisfied
    }

    /// Check if this multigoal is empty (has no goals)
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty() || 
        self.variables.values().all(|var_map| var_map.is_empty())
    }

    /// Get the total number of individual goals in this multigoal
    pub fn goal_count(&self) -> usize {
        self.variables.values().map(|var_map| var_map.len()).sum()
    }

    /// Convert to a JSON representation
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Create from a JSON representation
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }

    /// Create a multigoal from individual unigoals
    pub fn from_unigoals(name: impl Into<String>, unigoals: Vec<(String, String, StateValue)>) -> Self {
        let mut multigoal = Self::new(name);
        
        for (var_name, arg, value) in unigoals {
            multigoal.set_goal(var_name, arg, value);
        }
        
        multigoal
    }

    /// Convert this multigoal to a list of individual unigoals
    pub fn to_unigoals(&self) -> Vec<(String, String, StateValue)> {
        let mut unigoals = Vec::new();
        
        for (var_name, goal_map) in &self.variables {
            for (arg, value) in goal_map {
                unigoals.push((var_name.clone(), arg.clone(), value.clone()));
            }
        }
        
        unigoals
    }
}

impl std::fmt::Display for Multigoal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Multigoal {}>", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::State;

    #[test]
    fn test_multigoal_creation() {
        let multigoal = Multigoal::new("test_goal");
        assert_eq!(multigoal.name, "test_goal");
        assert!(multigoal.variables.is_empty());
        assert!(multigoal.is_empty());
    }

    #[test]
    fn test_multigoal_goals() {
        let mut multigoal = Multigoal::new("test");
        
        // Set some goals
        multigoal.set_goal("loc", "alice", "park".into());
        multigoal.set_goal("loc", "bob", "home".into());
        multigoal.set_goal("cash", "alice", 50.into());
        
        // Test getting goals
        assert_eq!(multigoal.get_goal("loc", "alice"), Some(&"park".into()));
        assert_eq!(multigoal.get_goal("loc", "bob"), Some(&"home".into()));
        assert_eq!(multigoal.get_goal("cash", "alice"), Some(&50.into()));
        assert_eq!(multigoal.get_goal("loc", "charlie"), None);
        
        // Test existence checks
        assert!(multigoal.has_goal_var("loc"));
        assert!(multigoal.has_goal_arg("loc", "alice"));
        assert!(!multigoal.has_goal_arg("loc", "charlie"));
        assert!(!multigoal.has_goal_var("nonexistent"));
        
        // Test goal count
        assert_eq!(multigoal.goal_count(), 3);
        assert!(!multigoal.is_empty());
    }

    #[test]
    fn test_multigoal_satisfaction() {
        let mut multigoal = Multigoal::new("test_goal");
        multigoal.set_goal("loc", "alice", "park".into());
        multigoal.set_goal("loc", "bob", "home".into());
        
        let mut state = State::new("test_state");
        state.set_var("loc", "alice", "park".into());
        state.set_var("loc", "bob", "home".into());
        
        assert!(multigoal.is_satisfied_by(&state));
        
        // Change one variable
        state.set_var("loc", "alice", "store".into());
        assert!(!multigoal.is_satisfied_by(&state));
        
        // Check unsatisfied goals
        let unsatisfied = multigoal.unsatisfied_goals(&state);
        assert_eq!(unsatisfied.len(), 1);
        assert!(unsatisfied.contains_key("loc"));
        assert_eq!(unsatisfied["loc"]["alice"], crate::core::string_value("park"));
    }

    #[test]
    fn test_multigoal_copy() {
        let mut multigoal = Multigoal::new("original");
        multigoal.set_goal("loc", "alice", "park".into());
        
        let copy1 = multigoal.copy(None);
        assert_eq!(copy1.name, "original_copy_0");
        assert_eq!(copy1.get_goal("loc", "alice"), Some(&"park".into()));
        
        let copy2 = multigoal.copy(Some("custom_name".to_string()));
        assert_eq!(copy2.name, "custom_name");
        assert_eq!(copy2.get_goal("loc", "alice"), Some(&"park".into()));
    }

    #[test]
    fn test_unigoal_conversion() {
        let unigoals = vec![
            ("loc".to_string(), "alice".to_string(), "park".into()),
            ("loc".to_string(), "bob".to_string(), "home".into()),
            ("cash".to_string(), "alice".to_string(), 50.into()),
        ];
        
        let multigoal = Multigoal::from_unigoals("test", unigoals.clone());
        assert_eq!(multigoal.goal_count(), 3);
        
        let converted_back = multigoal.to_unigoals();
        assert_eq!(converted_back.len(), 3);
        
        // Check that all original unigoals are present (order may differ)
        for original_unigoal in &unigoals {
            assert!(converted_back.contains(original_unigoal));
        }
    }
}
