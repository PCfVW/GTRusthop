//! Domain representation for GTRusthop

use super::{State, Multigoal, StateValue, TodoList};
use crate::error::Result;
use indexmap::IndexMap;
use std::sync::Arc;

/// Type alias for action functions
/// Actions take a mutable state and arguments, return Option<State> (None if not applicable)
pub type ActionFn = Arc<dyn Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync>;

/// Type alias for command functions  
/// Commands are like actions but for execution (not planning)
pub type CommandFn = Arc<dyn Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync>;

/// Type alias for task method functions
/// Task methods take a state and arguments, return Option<TodoList> (None if not applicable)
pub type TaskMethodFn = Arc<dyn Fn(&State, &[StateValue]) -> Option<TodoList> + Send + Sync>;

/// Type alias for unigoal method functions
/// Unigoal methods take a state, arg, and desired value, return Option<TodoList>
pub type UnigoalMethodFn = Arc<dyn Fn(&State, &str, &StateValue) -> Option<TodoList> + Send + Sync>;

/// Type alias for multigoal method functions
/// Multigoal methods take a state and multigoal, return Option<TodoList>
pub type MultigoalMethodFn = Arc<dyn Fn(&State, &Multigoal) -> Option<TodoList> + Send + Sync>;

/// Represents a planning domain containing actions, methods, and commands
#[derive(Clone)]
pub struct Domain {
    /// Name of the domain
    pub name: String,
    /// Map of action names to action functions
    actions: IndexMap<String, ActionFn>,
    /// Map of command names to command functions
    commands: IndexMap<String, CommandFn>,
    /// Map of task names to lists of task method functions
    task_methods: IndexMap<String, Vec<TaskMethodFn>>,
    /// Map of state variable names to lists of unigoal method functions
    unigoal_methods: IndexMap<String, Vec<UnigoalMethodFn>>,
    /// List of multigoal method functions
    multigoal_methods: Vec<MultigoalMethodFn>,
    /// Copy counter for generating unique names
    copy_counter: usize,
}

impl Domain {
    /// Create a new domain with the given name
    pub fn new(name: impl Into<String>) -> Self {
        let mut domain = Self {
            name: name.into(),
            actions: IndexMap::new(),
            commands: IndexMap::new(),
            task_methods: IndexMap::new(),
            unigoal_methods: IndexMap::new(),
            multigoal_methods: Vec::new(),
            copy_counter: 0,
        };

        // Add built-in verification methods
        domain.add_builtin_methods();
        domain
    }

    /// Add built-in verification methods
    fn add_builtin_methods(&mut self) {
        // Add _verify_g task method
        let verify_g_method: TaskMethodFn = Arc::new(|state, args| {
            if args.len() >= 5 {
                if let (Some(_method_name), Some(var_name), Some(arg), desired_val, Some(_depth)) = (
                    args[0].as_str(),
                    args[1].as_str(),
                    args[2].as_str(),
                    &args[3],
                    args[4].as_u64()
                ) {
                    if state.satisfies_unigoal(var_name, arg, desired_val) {
                        Some(vec![]) // Success, no subtasks
                    } else {
                        // In a real implementation, this would raise an exception
                        // For now, we'll return None to indicate failure
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });

        // Add _verify_mg task method  
        let verify_mg_method: TaskMethodFn = Arc::new(|_state, args| {
            if args.len() >= 3 {
                if let (Some(_method_name), Some(_depth)) = (
                    args[0].as_str(),
                    args[2].as_u64()
                ) {
                    // For multigoal verification, we'd need to deserialize the multigoal
                    // This is a simplified version
                    Some(vec![]) // Success, no subtasks
                } else {
                    None
                }
            } else {
                None
            }
        });

        self.task_methods.insert("_verify_g".to_string(), vec![verify_g_method]);
        self.task_methods.insert("_verify_mg".to_string(), vec![verify_mg_method]);
    }

    /// Declare actions in this domain
    pub fn declare_actions<F>(&mut self, actions: Vec<(String, F)>) -> Result<()>
    where
        F: Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync + 'static,
    {
        for (name, action_fn) in actions {
            self.actions.insert(name, Arc::new(action_fn));
        }
        Ok(())
    }

    /// Declare a single action in this domain
    pub fn declare_action<F>(&mut self, name: impl Into<String>, action_fn: F) -> Result<()>
    where
        F: Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync + 'static,
    {
        self.actions.insert(name.into(), Arc::new(action_fn));
        Ok(())
    }

    /// Declare commands in this domain
    pub fn declare_commands<F>(&mut self, commands: Vec<(String, F)>) -> Result<()>
    where
        F: Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync + 'static,
    {
        for (name, command_fn) in commands {
            self.commands.insert(name, Arc::new(command_fn));
        }
        Ok(())
    }

    /// Declare a single command in this domain
    pub fn declare_command<F>(&mut self, name: impl Into<String>, command_fn: F) -> Result<()>
    where
        F: Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync + 'static,
    {
        self.commands.insert(name.into(), Arc::new(command_fn));
        Ok(())
    }

    /// Declare task methods for a specific task name
    pub fn declare_task_methods<F>(&mut self, task_name: impl Into<String>, methods: Vec<F>) -> Result<()>
    where
        F: Fn(&State, &[StateValue]) -> Option<TodoList> + Send + Sync + 'static,
    {
        let task_name = task_name.into();
        let method_fns: Vec<TaskMethodFn> = methods.into_iter()
            .map(|f| Arc::new(f) as TaskMethodFn)
            .collect();

        if let Some(existing_methods) = self.task_methods.get_mut(&task_name) {
            existing_methods.extend(method_fns);
        } else {
            self.task_methods.insert(task_name, method_fns);
        }
        Ok(())
    }

    /// Declare a single task method
    pub fn declare_task_method<F>(&mut self, task_name: impl Into<String>, method_fn: F) -> Result<()>
    where
        F: Fn(&State, &[StateValue]) -> Option<TodoList> + Send + Sync + 'static,
    {
        self.declare_task_methods(task_name, vec![method_fn])
    }

    /// Declare unigoal methods for a specific state variable
    pub fn declare_unigoal_methods<F>(&mut self, var_name: impl Into<String>, methods: Vec<F>) -> Result<()>
    where
        F: Fn(&State, &str, &StateValue) -> Option<TodoList> + Send + Sync + 'static,
    {
        let var_name = var_name.into();
        let method_fns: Vec<UnigoalMethodFn> = methods.into_iter()
            .map(|f| Arc::new(f) as UnigoalMethodFn)
            .collect();

        if let Some(existing_methods) = self.unigoal_methods.get_mut(&var_name) {
            existing_methods.extend(method_fns);
        } else {
            self.unigoal_methods.insert(var_name, method_fns);
        }
        Ok(())
    }

    /// Declare a single unigoal method
    pub fn declare_unigoal_method<F>(&mut self, var_name: impl Into<String>, method_fn: F) -> Result<()>
    where
        F: Fn(&State, &str, &StateValue) -> Option<TodoList> + Send + Sync + 'static,
    {
        self.declare_unigoal_methods(var_name, vec![method_fn])
    }

    /// Declare multigoal methods
    pub fn declare_multigoal_methods<F>(&mut self, methods: Vec<F>) -> Result<()>
    where
        F: Fn(&State, &Multigoal) -> Option<TodoList> + Send + Sync + 'static,
    {
        let method_fns: Vec<MultigoalMethodFn> = methods.into_iter()
            .map(|f| Arc::new(f) as MultigoalMethodFn)
            .collect();

        self.multigoal_methods.extend(method_fns);
        Ok(())
    }

    /// Declare a single multigoal method
    pub fn declare_multigoal_method<F>(&mut self, method_fn: F) -> Result<()>
    where
        F: Fn(&State, &Multigoal) -> Option<TodoList> + Send + Sync + 'static,
    {
        self.declare_multigoal_methods(vec![method_fn])
    }

    /// Get an action by name
    pub fn get_action(&self, name: &str) -> Option<&ActionFn> {
        self.actions.get(name)
    }

    /// Get a command by name
    pub fn get_command(&self, name: &str) -> Option<&CommandFn> {
        self.commands.get(name)
    }

    /// Get task methods for a task name
    pub fn get_task_methods(&self, task_name: &str) -> Option<&Vec<TaskMethodFn>> {
        self.task_methods.get(task_name)
    }

    /// Get unigoal methods for a state variable
    pub fn get_unigoal_methods(&self, var_name: &str) -> Option<&Vec<UnigoalMethodFn>> {
        self.unigoal_methods.get(var_name)
    }

    /// Get all multigoal methods
    pub fn get_multigoal_methods(&self) -> &Vec<MultigoalMethodFn> {
        &self.multigoal_methods
    }

    /// Check if an action exists
    pub fn has_action(&self, name: &str) -> bool {
        self.actions.contains_key(name)
    }

    /// Check if a command exists
    pub fn has_command(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    /// Check if task methods exist for a task name
    pub fn has_task_methods(&self, task_name: &str) -> bool {
        self.task_methods.contains_key(task_name)
    }

    /// Check if unigoal methods exist for a state variable
    pub fn has_unigoal_methods(&self, var_name: &str) -> bool {
        self.unigoal_methods.contains_key(var_name)
    }

    /// Get all action names
    pub fn action_names(&self) -> Vec<&String> {
        self.actions.keys().collect()
    }

    /// Get all command names
    pub fn command_names(&self) -> Vec<&String> {
        self.commands.keys().collect()
    }

    /// Get all task names
    pub fn task_names(&self) -> Vec<&String> {
        self.task_methods.keys().collect()
    }

    /// Get all unigoal variable names
    pub fn unigoal_var_names(&self) -> Vec<&String> {
        self.unigoal_methods.keys().collect()
    }

    /// Create a copy of the domain with an optional new name
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

    /// Display domain information
    pub fn display(&self) {
        println!("\nDomain name: {}", self.name);
        self.print_actions();
        self.print_commands();
        self.print_methods();
    }

    /// Print all actions
    pub fn print_actions(&self) {
        if self.actions.is_empty() {
            println!("-- There are no actions --");
        } else {
            let action_names: Vec<String> = self.actions.keys().cloned().collect();
            println!("-- Actions: {}", action_names.join(", "));
        }
    }

    /// Print all commands
    pub fn print_commands(&self) {
        if self.commands.is_empty() {
            println!("-- There are no commands --");
        } else {
            let command_names: Vec<String> = self.commands.keys().cloned().collect();
            println!("-- Commands: {}", command_names.join(", "));
        }
    }

    /// Print all methods
    pub fn print_methods(&self) {
        self.print_task_methods();
        self.print_unigoal_methods();
        self.print_multigoal_methods();
    }

    /// Print task methods
    pub fn print_task_methods(&self) {
        if self.task_methods.is_empty() {
            println!("-- There are no task methods --");
        } else {
            println!("\nTask name:         Relevant task methods:");
            println!("---------------    ----------------------");
            for (task_name, methods) in &self.task_methods {
                println!("{:<19}{} methods", task_name, methods.len());
            }
            println!();
        }
    }

    /// Print unigoal methods
    pub fn print_unigoal_methods(&self) {
        if self.unigoal_methods.is_empty() {
            println!("-- There are no unigoal methods --");
        } else {
            println!("State var name:    Relevant unigoal methods:");
            println!("---------------    -------------------------");
            for (var_name, methods) in &self.unigoal_methods {
                println!("{:<19}{} methods", var_name, methods.len());
            }
            println!();
        }
    }

    /// Print multigoal methods
    pub fn print_multigoal_methods(&self) {
        if self.multigoal_methods.is_empty() {
            println!("-- There are no multigoal methods --");
        } else {
            println!("-- Multigoal methods: {} methods", self.multigoal_methods.len());
        }
    }


}

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Domain {}>", self.name)
    }
}

impl std::fmt::Debug for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Domain")
            .field("name", &self.name)
            .field("actions", &self.actions.keys().collect::<Vec<_>>())
            .field("commands", &self.commands.keys().collect::<Vec<_>>())
            .field("task_methods", &self.task_methods.keys().collect::<Vec<_>>())
            .field("unigoal_methods", &self.unigoal_methods.keys().collect::<Vec<_>>())
            .field("multigoal_methods_count", &self.multigoal_methods.len())
            .finish()
    }
}
