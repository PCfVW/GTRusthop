//! Core data structures for GTRusthop

pub mod state;
pub mod multigoal;
pub mod domain;

pub use state::State;
pub use multigoal::Multigoal;
pub use domain::Domain;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Type alias for state variable values
pub type StateValue = serde_json::Value;

/// Type alias for state variable maps
pub type StateVarMap = HashMap<String, StateValue>;

/// Represents a task or action in the planning system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlanItem {
    /// A task with name and arguments
    Task(String, Vec<StateValue>),
    /// An action with name and arguments  
    Action(String, Vec<StateValue>),
    /// A unigoal with (state_var_name, arg, value)
    Unigoal(String, String, StateValue),
    /// A multigoal
    Multigoal(Multigoal),
}

impl PlanItem {
    /// Create a new task
    pub fn task(name: impl Into<String>, args: Vec<StateValue>) -> Self {
        Self::Task(name.into(), args)
    }

    /// Create a new action
    pub fn action(name: impl Into<String>, args: Vec<StateValue>) -> Self {
        Self::Action(name.into(), args)
    }

    /// Create a new unigoal
    pub fn unigoal(
        state_var_name: impl Into<String>,
        arg: impl Into<String>,
        value: StateValue,
    ) -> Self {
        Self::Unigoal(state_var_name.into(), arg.into(), value)
    }

    /// Create a new multigoal
    pub fn multigoal(multigoal: Multigoal) -> Self {
        Self::Multigoal(multigoal)
    }

    /// Get the name of this plan item
    pub fn name(&self) -> &str {
        match self {
            Self::Task(name, _) | Self::Action(name, _) | Self::Unigoal(name, _, _) => name,
            Self::Multigoal(mg) => &mg.name,
        }
    }

    /// Get the arguments of this plan item as owned values
    pub fn args(&self) -> Vec<StateValue> {
        match self {
            Self::Task(_, args) | Self::Action(_, args) => args.clone(),
            Self::Unigoal(_, arg, value) => vec![
                string_value(arg),
                value.clone(),
            ],
            Self::Multigoal(_) => vec![],
        }
    }

    /// Check if this is a task
    pub fn is_task(&self) -> bool {
        matches!(self, Self::Task(_, _))
    }

    /// Check if this is an action
    pub fn is_action(&self) -> bool {
        matches!(self, Self::Action(_, _))
    }

    /// Check if this is a unigoal
    pub fn is_unigoal(&self) -> bool {
        matches!(self, Self::Unigoal(_, _, _))
    }

    /// Check if this is a multigoal
    pub fn is_multigoal(&self) -> bool {
        matches!(self, Self::Multigoal(_))
    }
}

impl std::fmt::Display for PlanItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Task(name, args) => {
                write!(f, "({name}")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, ")")
            }
            Self::Action(name, args) => {
                write!(f, "({name}")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, ")")
            }
            Self::Unigoal(var_name, arg, value) => {
                write!(f, "({var_name} {arg} {value})")
            }
            Self::Multigoal(mg) => write!(f, "{mg}"),
        }
    }
}

/// Type alias for a plan (sequence of actions)
pub type Plan = Vec<PlanItem>;

/// Type alias for a todo list (sequence of tasks, goals, and actions)
pub type TodoList = Vec<PlanItem>;

/// Helper function to create a plan item from a tuple
pub fn plan_item_from_tuple(name: &str, args: &[StateValue]) -> PlanItem {
    PlanItem::Task(name.to_string(), args.to_vec())
}

/// Helper function to create a state value from various types
pub fn state_value_from<T: Into<StateValue>>(value: T) -> StateValue {
    value.into()
}

/// Helper functions to create StateValue from various types
pub fn string_value(s: impl Into<String>) -> StateValue {
    StateValue::String(s.into())
}

pub fn int_value(i: i64) -> StateValue {
    StateValue::Number(serde_json::Number::from(i))
}

pub fn float_value(f: f64) -> StateValue {
    StateValue::Number(serde_json::Number::from_f64(f).unwrap_or(serde_json::Number::from(0)))
}

pub fn bool_value(b: bool) -> StateValue {
    StateValue::Bool(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_item_creation() {
        let task = PlanItem::task("travel", vec!["alice".into(), "home".into(), "park".into()]);
        assert!(task.is_task());
        assert_eq!(task.name(), "travel");

        let action = PlanItem::action("move", vec!["obj1".into(), "loc2".into()]);
        assert!(action.is_action());
        assert_eq!(action.name(), "move");

        let unigoal = PlanItem::unigoal("loc", "alice", "park".into());
        assert!(unigoal.is_unigoal());
        assert_eq!(unigoal.name(), "loc");
    }

    #[test]
    fn test_state_value_conversions() {
        let str_val: StateValue = "test".into();
        assert_eq!(str_val, StateValue::String("test".to_string()));

        let int_val: StateValue = 42.into();
        assert_eq!(int_val, StateValue::Number(serde_json::Number::from(42)));

        let bool_val: StateValue = true.into();
        assert_eq!(bool_val, StateValue::Bool(true));
    }
}
