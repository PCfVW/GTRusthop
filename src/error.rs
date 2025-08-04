//! Error types for GTRusthop

use thiserror::Error;

/// Result type alias for GTRusthop operations
pub type Result<T> = std::result::Result<T, GTRustHopError>;

/// Main error type for GTRusthop operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum GTRustHopError {
    /// No domain has been created yet
    #[error("Cannot declare {item_type} until a domain has been created")]
    NoDomainCreated { item_type: String },

    /// Domain not found
    #[error("Domain '{name}' not found")]
    DomainNotFound { name: String },

    /// No planning strategy has been set
    #[error("No planning strategy (iterative or recursive) has been set. Use set_planning_strategy() to set it")]
    NoPlanningStrategy,

    /// Invalid verbose level
    #[error("Verbose level must be between 0 and 3, got {level}")]
    InvalidVerboseLevel { level: i32 },

    /// Planning failed
    #[error("Planning failed: {reason}")]
    PlanningFailed { reason: String },

    /// Method verification failed
    #[error("Method '{method}' didn't achieve goal {goal} at depth {depth}")]
    MethodVerificationFailed {
        method: String,
        goal: String,
        depth: usize,
    },

    /// Multigoal method verification failed
    #[error("Method '{method}' didn't achieve multigoal {multigoal} at depth {depth}")]
    MultigoalVerificationFailed {
        method: String,
        multigoal: String,
        depth: usize,
    },

    /// Invalid task/action/goal type
    #[error("Item '{item}' isn't an action, task, unigoal, or multigoal at depth {depth}")]
    InvalidItemType { item: String, depth: usize },

    /// Action execution failed
    #[error("Action '{action}' is not applicable in current state")]
    ActionNotApplicable { action: String },

    /// Command execution failed
    #[error("Command '{command}' failed")]
    CommandFailed { command: String },

    /// State variable not found
    #[error("State variable '{var_name}' not found")]
    StateVariableNotFound { var_name: String },

    /// State variable argument not found
    #[error("Argument '{arg}' not found in state variable '{var_name}'")]
    StateVariableArgNotFound { var_name: String, arg: String },

    /// Generic error for other cases
    #[error("GTRusthop error: {message}")]
    Generic { message: String },
}

impl GTRustHopError {
    /// Create a new NoDomainCreated error
    pub fn no_domain_created(item_type: impl Into<String>) -> Self {
        Self::NoDomainCreated {
            item_type: item_type.into(),
        }
    }

    /// Create a new DomainNotFound error
    pub fn domain_not_found(name: impl Into<String>) -> Self {
        Self::DomainNotFound { name: name.into() }
    }

    /// Create a new PlanningFailed error
    pub fn planning_failed(reason: impl Into<String>) -> Self {
        Self::PlanningFailed {
            reason: reason.into(),
        }
    }

    /// Create a new MethodVerificationFailed error
    pub fn method_verification_failed(
        method: impl Into<String>,
        goal: impl Into<String>,
        depth: usize,
    ) -> Self {
        Self::MethodVerificationFailed {
            method: method.into(),
            goal: goal.into(),
            depth,
        }
    }

    /// Create a new MultigoalVerificationFailed error
    pub fn multigoal_verification_failed(
        method: impl Into<String>,
        multigoal: impl Into<String>,
        depth: usize,
    ) -> Self {
        Self::MultigoalVerificationFailed {
            method: method.into(),
            multigoal: multigoal.into(),
            depth,
        }
    }

    /// Create a new InvalidItemType error
    pub fn invalid_item_type(item: impl Into<String>, depth: usize) -> Self {
        Self::InvalidItemType {
            item: item.into(),
            depth,
        }
    }

    /// Create a new ActionNotApplicable error
    pub fn action_not_applicable(action: impl Into<String>) -> Self {
        Self::ActionNotApplicable {
            action: action.into(),
        }
    }

    /// Create a new CommandFailed error
    pub fn command_failed(command: impl Into<String>) -> Self {
        Self::CommandFailed {
            command: command.into(),
        }
    }

    /// Create a new StateVariableNotFound error
    pub fn state_variable_not_found(var_name: impl Into<String>) -> Self {
        Self::StateVariableNotFound {
            var_name: var_name.into(),
        }
    }

    /// Create a new StateVariableArgNotFound error
    pub fn state_variable_arg_not_found(
        var_name: impl Into<String>,
        arg: impl Into<String>,
    ) -> Self {
        Self::StateVariableArgNotFound {
            var_name: var_name.into(),
            arg: arg.into(),
        }
    }

    /// Create a new Generic error
    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic {
            message: message.into(),
        }
    }
}
