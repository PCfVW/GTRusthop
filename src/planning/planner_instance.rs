//! Isolated planner instance implementation
//! 
//! This module provides the core planning functionality for isolated planner instances,
//! eliminating race conditions from global state.

use crate::core::{State, PlanItem, Plan};
use crate::error::Result;
use crate::planning::{Planner, PlanningStrategy};

impl Planner {
    /// Find a plan to achieve the given goals/tasks
    ///
    /// This is the main planning function that uses the planner's isolated state
    /// instead of global variables, making it thread-safe.
    pub fn find_plan(&self, state: State, todo_list: Vec<PlanItem>) -> Result<Option<Plan>> {
        if self.verbose_level >= 1 {
            println!("FP> find_plan, verbose={}:", self.verbose_level);
            println!("    state = {}", state.name);
            println!("    todo_list = {:?}", todo_list);
        }

        match self.strategy {
            PlanningStrategy::Iterative => self.find_plan_iterative(state, todo_list),
            PlanningStrategy::Recursive => self.find_plan_recursive(state, todo_list, 0),
        }
    }

    /// Pyhop compatibility function
    ///
    /// This function exists to provide backward compatibility with the original Pyhop planner.
    /// It's essentially a wrapper around `find_plan()` with a deprecation message.
    ///
    /// In the Python GTPyhop version, this function prints a deprecation message when
    /// verbose level > 0, encouraging users to use `find_plan` instead.
    ///
    /// # Arguments
    ///
    /// * `state` - The initial state
    /// * `todo_list` - List of tasks, goals, and actions to achieve
    ///
    /// # Returns
    ///
    /// The same result as `find_plan()`: `Ok(Some(plan))` if successful,
    /// `Ok(None)` if no plan found, or `Err` if an error occurred.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use gtrusthop::{PlannerBuilder, Domain, State, PlanItem};
    /// # let domain = Domain::new("test");
    /// # let state = State::new("test");
    /// # let todo_list: Vec<PlanItem> = vec![];
    /// # let planner = PlannerBuilder::new().with_domain(domain).build().unwrap();
    /// // This is the old Pyhop-style call
    /// let plan = planner.pyhop(state, todo_list)?;
    ///
    /// // Preferred modern call
    /// let plan = planner.find_plan(state, todo_list)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pyhop(&self, state: State, todo_list: Vec<PlanItem>) -> Result<Option<Plan>> {
        if self.verbose_level > 0 {
            println!();
            println!("        >> The function 'pyhop' exists to provide backward compatibility");
            println!("        >> with Pyhop. In the future, please use find_plan instead.");
        }
        self.find_plan(state, todo_list)
    }
    
    /// Iterative planning implementation
    fn find_plan_iterative(&self, initial_state: State, initial_todo: Vec<PlanItem>) -> Result<Option<Plan>> {
        use std::collections::VecDeque;
        
        #[derive(Debug)]
        struct PlanningFrame {
            state: State,
            todo_list: Vec<PlanItem>,
            plan: Plan,
            depth: usize,
        }
        
        let mut stack = VecDeque::new();
        stack.push_back(PlanningFrame {
            state: initial_state,
            todo_list: initial_todo,
            plan: Vec::new(),
            depth: 0,
        });
        
        while let Some(frame) = stack.pop_back() {
            if self.verbose_level >= 2 {
                println!("FP> depth {}, todo_list = {:?}", frame.depth, frame.todo_list);
            }
            
            if frame.todo_list.is_empty() {
                if self.verbose_level >= 1 {
                    println!("FP> result = {:?}", frame.plan);
                }
                return Ok(Some(frame.plan));
            }
            
            let current_item = &frame.todo_list[0];
            let remaining_todo = frame.todo_list[1..].to_vec();
            
            match current_item {
                PlanItem::Action(action_name, args) => {
                    if let Some(action_fn) = self.domain.get_action(action_name) {
                        let mut state_copy = frame.state.copy(None);
                        if let Some(new_state) = action_fn(&mut state_copy, args) {
                            let mut new_plan = frame.plan.clone();
                            new_plan.push(current_item.clone());
                            
                            stack.push_back(PlanningFrame {
                                state: new_state,
                                todo_list: remaining_todo,
                                plan: new_plan,
                                depth: frame.depth + 1,
                            });
                        }
                    }
                }
                PlanItem::Task(task_name, args) => {
                    if let Some(methods) = self.domain.get_task_methods(task_name) {
                        for method in methods.iter().rev() {
                            if let Some(subtasks) = method(&frame.state, args) {
                                let mut new_todo = subtasks;
                                new_todo.extend(remaining_todo.clone());
                                
                                stack.push_back(PlanningFrame {
                                    state: frame.state.copy(None),
                                    todo_list: new_todo,
                                    plan: frame.plan.clone(),
                                    depth: frame.depth + 1,
                                });
                            }
                        }
                    }
                }
                PlanItem::Unigoal(var_name, arg, value) => {
                    if frame.state.satisfies_unigoal(var_name, arg, value) {
                        stack.push_back(PlanningFrame {
                            state: frame.state,
                            todo_list: remaining_todo,
                            plan: frame.plan,
                            depth: frame.depth,
                        });
                    } else if let Some(methods) = self.domain.get_unigoal_methods(var_name) {
                        for method in methods.iter().rev() {
                            if let Some(subtasks) = method(&frame.state, arg, value) {
                                let mut new_todo = subtasks;
                                new_todo.extend(remaining_todo.clone());
                                
                                stack.push_back(PlanningFrame {
                                    state: frame.state.copy(None),
                                    todo_list: new_todo,
                                    plan: frame.plan.clone(),
                                    depth: frame.depth + 1,
                                });
                            }
                        }
                    }
                }
                PlanItem::Multigoal(multigoal) => {
                    if multigoal.is_satisfied_by(&frame.state) {
                        stack.push_back(PlanningFrame {
                            state: frame.state,
                            todo_list: remaining_todo,
                            plan: frame.plan,
                            depth: frame.depth,
                        });
                    } else {
                        // Convert multigoal to individual unigoals
                        let mut new_todo = Vec::new();
                        for (var_name, arg, value) in multigoal.to_unigoals() {
                            new_todo.push(PlanItem::unigoal(var_name, arg, value));
                        }
                        new_todo.extend(remaining_todo);

                        stack.push_back(PlanningFrame {
                            state: frame.state,
                            todo_list: new_todo,
                            plan: frame.plan,
                            depth: frame.depth,
                        });
                    }
                }
            }
        }
        
        if self.verbose_level >= 1 {
            println!("FP> result = None");
        }
        Ok(None)
    }
    
    /// Recursive planning implementation
    fn find_plan_recursive(&self, state: State, todo_list: Vec<PlanItem>, depth: usize) -> Result<Option<Plan>> {
        if self.verbose_level >= 2 {
            println!("FP> depth {}, todo_list = {:?}", depth, todo_list);
        }
        
        if todo_list.is_empty() {
            return Ok(Some(Vec::new()));
        }
        
        let current_item = &todo_list[0];
        let remaining_todo = todo_list[1..].to_vec();
        
        match current_item {
            PlanItem::Action(action_name, args) => {
                if let Some(action_fn) = self.domain.get_action(action_name) {
                    let mut state_copy = state.copy(None);
                    if let Some(new_state) = action_fn(&mut state_copy, args) {
                        if let Some(mut plan) = self.find_plan_recursive(new_state, remaining_todo, depth + 1)? {
                            plan.insert(0, current_item.clone());
                            return Ok(Some(plan));
                        }
                    }
                }
            }
            PlanItem::Task(task_name, args) => {
                if let Some(methods) = self.domain.get_task_methods(task_name) {
                    for method in methods {
                        if let Some(subtasks) = method(&state, args) {
                            let mut new_todo = subtasks;
                            new_todo.extend(remaining_todo.clone());
                            
                            if let Some(plan) = self.find_plan_recursive(state.copy(None), new_todo, depth + 1)? {
                                return Ok(Some(plan));
                            }
                        }
                    }
                }
            }
            PlanItem::Unigoal(var_name, arg, value) => {
                if state.satisfies_unigoal(var_name, arg, value) {
                    return self.find_plan_recursive(state, remaining_todo, depth);
                } else if let Some(methods) = self.domain.get_unigoal_methods(var_name) {
                    for method in methods {
                        if let Some(subtasks) = method(&state, arg, value) {
                            let mut new_todo = subtasks;
                            new_todo.extend(remaining_todo.clone());
                            
                            if let Some(plan) = self.find_plan_recursive(state.copy(None), new_todo, depth + 1)? {
                                return Ok(Some(plan));
                            }
                        }
                    }
                }
            }
            PlanItem::Multigoal(multigoal) => {
                if multigoal.is_satisfied_by(&state) {
                    return self.find_plan_recursive(state, remaining_todo, depth);
                } else {
                    // Convert multigoal to individual unigoals
                    let mut new_todo = Vec::new();
                    for (var_name, arg, value) in multigoal.to_unigoals() {
                        new_todo.push(PlanItem::unigoal(var_name, arg, value));
                    }
                    new_todo.extend(remaining_todo);

                    return self.find_plan_recursive(state, new_todo, depth);
                }
            }
        }
        
        Ok(None)
    }
    
    /// Check if verbose output should be printed at the given level
    pub fn is_verbose(&self, level: i32) -> bool {
        self.verbose_level >= level
    }

    /// Run lazy lookahead algorithm for acting
    ///
    /// An adaptation of the run_lazy_lookahead algorithm from Ghallab et al.
    /// (2016), Automated Planning and Acting. It works roughly like this:
    ///     loop:
    ///         plan = find_plan(state, todo_list)
    ///         if plan = [] then return state    // the new current state
    ///         for each action in plan:
    ///             try to execute the corresponding command
    ///             if the command fails, continue the outer loop
    ///
    /// Arguments:
    /// - `state` is the current state
    /// - `todo_list` is a list of tasks, goals, and multigoals
    /// - `max_tries` is a bound on how many times to execute the outer loop
    ///
    /// Note: whenever run_lazy_lookahead encounters an action for which there is
    /// no corresponding command definition, it uses the action definition instead.
    pub fn run_lazy_lookahead(
        &self,
        mut state: State,
        todo_list: Vec<PlanItem>,
        max_tries: usize,
    ) -> Result<State> {
        if self.is_verbose(1) {
            println!("RLL> run_lazy_lookahead, verbose = {}, max_tries = {}", self.verbose_level, max_tries);
            println!("RLL> initial state: {}", state.name);
            println!("RLL> To do: {:?}", todo_list);
        }

        for tries in 1..=max_tries {
            if self.is_verbose(1) {
                let ordinal = match tries {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                };
                println!("RLL> {}{} call to find_plan:\n", tries, ordinal);
            }

            let plan = self.find_plan(state.clone(), todo_list.clone())?;

            match plan {
                None => {
                    if self.is_verbose(1) {
                        return Err(crate::error::GTRustHopError::planning_failed("run_lazy_lookahead: find_plan has failed"));
                    }
                    return Ok(state);
                }
                Some(plan) if plan.is_empty() => {
                    if self.is_verbose(1) {
                        println!("RLL> Empty plan => success after {} calls to find_plan.", tries);
                    }
                    if self.is_verbose(2) {
                        state.display(Some("RLL> final state"));
                    }
                    return Ok(state);
                }
                Some(plan) => {
                    // Execute the plan
                    let mut plan_failed = false;
                    for action in &plan {
                        if let PlanItem::Action(action_name, args) = action {
                            let command_name = format!("c_{}", action_name);

                            // Try to find a command, fall back to action
                            let command_fn = self.domain.get_command(&command_name)
                                .or_else(|| self.domain.get_action(action_name));

                            if let Some(cmd_fn) = command_fn {
                                if self.domain.get_command(&command_name).is_none() && self.is_verbose(1) {
                                    println!("RLL> {} not defined, using {} instead\n", command_name, action_name);
                                }

                                if self.is_verbose(1) {
                                    println!("RLL> Command: {} {:?}", command_name, args);
                                }

                                let mut state_copy = state.copy(None);
                                if let Some(new_state) = cmd_fn(&mut state_copy, args) {
                                    if self.is_verbose(2) {
                                        new_state.display(None);
                                    }
                                    state = new_state;
                                } else {
                                    if self.is_verbose(1) {
                                        println!("RLL> WARNING: command {} failed; will call find_plan.", command_name);
                                    }
                                    plan_failed = true;
                                    break;
                                }
                            } else {
                                if self.is_verbose(1) {
                                    println!("RLL> WARNING: no command or action {}; will call find_plan.", action_name);
                                }
                                plan_failed = true;
                                break;
                            }
                        }
                    }

                    if !plan_failed && self.is_verbose(1) {
                        println!("RLL> Plan ended; will call find_plan again.");
                    }
                }
            }
        }

        if self.is_verbose(1) {
            println!("RLL> Too many tries, giving up.");
        }
        if self.is_verbose(2) {
            state.display(Some("RLL> final state"));
        }
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Domain, State, string_value};
    use crate::planning::PlannerBuilder;

    #[test]
    fn test_planner_creation() -> Result<()> {
        let domain = Domain::new("test_domain");
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;

        assert_eq!(planner.verbose_level, 0);
        Ok(())
    }

    #[test]
    fn test_planner_with_verbose_level() -> Result<()> {
        let domain = Domain::new("test_domain");
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(2)?
            .build()?;

        assert_eq!(planner.verbose_level, 2);
        assert!(planner.is_verbose(1));
        assert!(planner.is_verbose(2));
        assert!(!planner.is_verbose(3));
        Ok(())
    }

    #[test]
    fn test_run_lazy_lookahead_success() -> Result<()> {
        let mut domain = Domain::new("test_domain");

        // Add action and command
        domain.declare_action("move", |state: &mut State, args: &[crate::core::StateValue]| {
            if args.len() >= 2 {
                if let (Some(obj), Some(target)) = (args[0].as_str(), args[1].as_str()) {
                    state.set_var("loc", obj, string_value(target));
                    return Some(state.clone());
                }
            }
            None
        })?;

        domain.declare_command("c_move", |state: &mut State, args: &[crate::core::StateValue]| {
            if args.len() >= 2 {
                if let (Some(obj), Some(target)) = (args[0].as_str(), args[1].as_str()) {
                    state.set_var("loc", obj, string_value(target));
                    return Some(state.clone());
                }
            }
            None
        })?;

        // Add task method
        domain.declare_task_method("transport", |state: &State, args: &[crate::core::StateValue]| {
            if args.len() >= 2 {
                if let (Some(obj), Some(target)) = (args[0].as_str(), args[1].as_str()) {
                    if let Some(current_loc) = state.get_var("loc", obj) {
                        if current_loc.as_str() != Some(target) {
                            return Some(vec![PlanItem::action("move", vec![string_value(obj), string_value(target)])]);
                        }
                    }
                    return Some(vec![]); // Already at target
                }
            }
            None
        })?;

        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;

        // Create initial state
        let mut state = State::new("initial_state");
        state.set_var("loc", "obj1", string_value("loc1"));

        // Create todo list
        let todo_list = vec![PlanItem::task("transport", vec![string_value("obj1"), string_value("loc2")])];

        // Run lazy lookahead
        let final_state = planner.run_lazy_lookahead(state, todo_list, 5)?;

        // Check that object moved to target location
        assert_eq!(final_state.get_var("loc", "obj1").unwrap().as_str(), Some("loc2"));
        Ok(())
    }

    #[test]
    fn test_run_lazy_lookahead_command_failure() -> Result<()> {
        let mut domain = Domain::new("test_domain");

        // Add action (for planning)
        domain.declare_action("move", |state: &mut State, args: &[crate::core::StateValue]| {
            if args.len() >= 2 {
                if let (Some(obj), Some(target)) = (args[0].as_str(), args[1].as_str()) {
                    state.set_var("loc", obj, string_value(target));
                    return Some(state.clone());
                }
            }
            None
        })?;

        // Add failing command (for execution)
        domain.declare_command("c_move", |_state: &mut State, _args: &[crate::core::StateValue]| {
            None // Always fails
        })?;

        // Add task method
        domain.declare_task_method("transport", |state: &State, args: &[crate::core::StateValue]| {
            if args.len() >= 2 {
                if let (Some(obj), Some(target)) = (args[0].as_str(), args[1].as_str()) {
                    if let Some(current_loc) = state.get_var("loc", obj) {
                        if current_loc.as_str() != Some(target) {
                            return Some(vec![PlanItem::action("move", vec![string_value(obj), string_value(target)])]);
                        }
                    }
                    return Some(vec![]); // Already at target
                }
            }
            None
        })?;

        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;

        // Create initial state
        let mut state = State::new("initial_state");
        state.set_var("loc", "obj1", string_value("loc1"));

        // Create todo list
        let todo_list = vec![PlanItem::task("transport", vec![string_value("obj1"), string_value("loc2")])];

        // Run lazy lookahead with limited tries
        let final_state = planner.run_lazy_lookahead(state, todo_list, 3)?;

        // Check that object is still at original location (command failed)
        assert_eq!(final_state.get_var("loc", "obj1").unwrap().as_str(), Some("loc1"));
        Ok(())
    }

    #[test]
    fn test_run_lazy_lookahead_already_satisfied() -> Result<()> {
        let mut domain = Domain::new("test_domain");

        // Add task method
        domain.declare_task_method("transport", |state: &State, args: &[crate::core::StateValue]| {
            if args.len() >= 2 {
                if let (Some(obj), Some(target)) = (args[0].as_str(), args[1].as_str()) {
                    if let Some(current_loc) = state.get_var("loc", obj) {
                        if current_loc.as_str() != Some(target) {
                            return Some(vec![PlanItem::action("move", vec![string_value(obj), string_value(target)])]);
                        }
                    }
                    return Some(vec![]); // Already at target
                }
            }
            None
        })?;

        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;

        // Create initial state where goal is already satisfied
        let mut state = State::new("initial_state");
        state.set_var("loc", "obj1", string_value("loc2"));

        // Create todo list
        let todo_list = vec![PlanItem::task("transport", vec![string_value("obj1"), string_value("loc2")])];

        // Run lazy lookahead
        let final_state = planner.run_lazy_lookahead(state, todo_list, 5)?;

        // Check that object is still at target location
        assert_eq!(final_state.get_var("loc", "obj1").unwrap().as_str(), Some("loc2"));
        Ok(())
    }
}
