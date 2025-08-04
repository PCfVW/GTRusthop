//! Planning strategy implementations for GTRusthop

use super::{PlanningContext, PlanningResult, PlanningStrategyTrait, is_verbose, verbose_print, item_to_string, todo_list_to_string};
use crate::core::{State, Multigoal, PlanItem, TodoList, Plan, StateValue};
use crate::error::{GTRustHopError, Result};
use std::sync::Mutex;

/// Parameters for action-related planning operations
#[derive(Debug, Clone)]
struct ActionParams<'a> {
    action_name: &'a str,
    args: &'a [StateValue],
}

/// Parameters for task-related planning operations
#[derive(Debug, Clone)]
struct TaskParams<'a> {
    task_name: &'a str,
    args: &'a [StateValue],
}

/// Parameters for unigoal-related planning operations
#[derive(Debug, Clone)]
struct UnigoalParams<'a> {
    var_name: &'a str,
    arg: &'a str,
    value: &'a StateValue,
}

/// Parameters for planning state
#[derive(Debug, Clone)]
struct PlanningState {
    todo_list: TodoList,
    plan: Plan,
    depth: usize,
}

/// Available planning strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanningStrategy {
    /// Recursive planning strategy (uses call stack)
    Recursive,
    /// Iterative planning strategy (uses explicit stack)
    Iterative,
}

/// Global planning strategy
static CURRENT_STRATEGY: Mutex<Option<PlanningStrategy>> = Mutex::new(None);

/// Set the current planning strategy
pub fn set_planning_strategy(strategy: PlanningStrategy) {
    let mut current = CURRENT_STRATEGY.lock().unwrap();
    *current = Some(strategy);
    
    match strategy {
        PlanningStrategy::Recursive => println!("Using recursive seek_plan."),
        PlanningStrategy::Iterative => println!("Using iterative seek_plan."),
    }
}

/// Get the current planning strategy
pub fn get_planning_strategy() -> Result<PlanningStrategy> {
    let current = CURRENT_STRATEGY.lock().unwrap();
    current.ok_or_else(|| GTRustHopError::NoPlanningStrategy)
}

/// Reset the planning strategy (force user to set it again)
pub fn reset_planning_strategy() {
    let mut current = CURRENT_STRATEGY.lock().unwrap();
    *current = None;
}

/// Recursive planning strategy implementation
pub struct RecursiveStrategy;

impl PlanningStrategyTrait for RecursiveStrategy {
    fn seek_plan(
        &self,
        context: &PlanningContext,
        state: State,
        todo_list: TodoList,
        plan: Plan,
        depth: usize,
    ) -> Result<PlanningResult> {
        if is_verbose(2) {
            let todo_string = todo_list_to_string(&todo_list);
            verbose_print(2, &format!("depth {depth} todo_list {todo_string}"));
        }

        // Base case: empty todo list means we're done
        if todo_list.is_empty() {
            if is_verbose(3) {
                verbose_print(3, &format!("depth {depth} no more tasks or goals, return plan"));
            }
            return Ok(PlanningResult::Success(plan));
        }

        let item = &todo_list[0];
        let remaining_todo = todo_list[1..].to_vec();

        match item {
            PlanItem::Multigoal(multigoal) => {
                self.refine_multigoal_and_continue(context, &state, multigoal, remaining_todo, plan, depth)
            }
            PlanItem::Task(task_name, args) => {
                if context.domain.has_action(task_name) {
                    let action_params = ActionParams { action_name: task_name, args };
                    let planning_state = PlanningState { todo_list: remaining_todo, plan, depth };
                    self.apply_action_and_continue(context, &state, &action_params, planning_state)
                } else if context.domain.has_task_methods(task_name) {
                    let task_params = TaskParams { task_name, args };
                    let planning_state = PlanningState { todo_list: remaining_todo, plan, depth };
                    self.refine_task_and_continue(context, &state, &task_params, planning_state)
                } else {
                    Err(GTRustHopError::invalid_item_type(item_to_string(item), depth))
                }
            }
            PlanItem::Action(action_name, args) => {
                let action_params = ActionParams { action_name, args };
                let planning_state = PlanningState { todo_list: remaining_todo, plan, depth };
                self.apply_action_and_continue(context, &state, &action_params, planning_state)
            }
            PlanItem::Unigoal(var_name, arg, value) => {
                if context.domain.has_unigoal_methods(var_name) {
                    let unigoal_params = UnigoalParams { var_name, arg, value };
                    let planning_state = PlanningState { todo_list: remaining_todo, plan, depth };
                    self.refine_unigoal_and_continue(context, &state, &unigoal_params, planning_state)
                } else {
                    Err(GTRustHopError::invalid_item_type(item_to_string(item), depth))
                }
            }
        }
    }
}

impl RecursiveStrategy {
    fn apply_action_and_continue(
        &self,
        context: &PlanningContext,
        state: &State,
        action_params: &ActionParams,
        planning_state: PlanningState,
    ) -> Result<PlanningResult> {
        let PlanningState { todo_list, mut plan, depth } = planning_state;

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} action {}: ", action_params.action_name));
        }

        if let Some(action_fn) = context.domain.get_action(action_params.action_name) {
            let mut new_state = state.copy(None);
            if let Some(result_state) = action_fn(&mut new_state, action_params.args) {
                if is_verbose(3) {
                    verbose_print(3, "applied");
                    result_state.display(None);
                }
                plan.push(PlanItem::action(action_params.action_name, action_params.args.to_vec()));
                return self.seek_plan(context, result_state, todo_list, plan, depth + 1);
            }
        }

        if is_verbose(3) {
            verbose_print(3, "not applicable");
        }
        Ok(PlanningResult::Failure)
    }

    fn refine_task_and_continue(
        &self,
        context: &PlanningContext,
        state: &State,
        task_params: &TaskParams,
        planning_state: PlanningState,
    ) -> Result<PlanningResult> {
        let PlanningState { todo_list, plan, depth } = planning_state;

        if let Some(methods) = context.domain.get_task_methods(task_params.task_name) {
            if is_verbose(3) {
                verbose_print(3, &format!("depth {} task {} methods: {} methods", depth, task_params.task_name, methods.len()));
            }

            for method in methods {
                if is_verbose(3) {
                    verbose_print(3, &format!("depth {depth} trying method: "));
                }

                if let Some(subtasks) = method(state, task_params.args) {
                    if is_verbose(3) {
                        verbose_print(3, "applicable");
                        verbose_print(3, &format!("depth {} subtasks: {}", depth, todo_list_to_string(&subtasks)));
                    }

                    let mut new_todo = subtasks;
                    new_todo.extend(todo_list.clone());

                    let result = self.seek_plan(context, state.clone(), new_todo, plan.clone(), depth + 1)?;
                    if let PlanningResult::Success(_) = result {
                        return Ok(result);
                    }
                } else if is_verbose(3) {
                    verbose_print(3, "not applicable");
                }
            }
        }

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} could not accomplish task {}", task_params.task_name));
        }
        Ok(PlanningResult::Failure)
    }

    fn refine_unigoal_and_continue(
        &self,
        context: &PlanningContext,
        state: &State,
        unigoal_params: &UnigoalParams,
        planning_state: PlanningState,
    ) -> Result<PlanningResult> {
        let PlanningState { todo_list, plan, depth } = planning_state;

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} goal ({} {} {}): ", unigoal_params.var_name, unigoal_params.arg, unigoal_params.value));
        }

        // Check if goal is already achieved
        if state.satisfies_unigoal(unigoal_params.var_name, unigoal_params.arg, unigoal_params.value) {
            if is_verbose(3) {
                verbose_print(3, "already achieved");
            }
            return self.seek_plan(context, state.clone(), todo_list, plan, depth + 1);
        }

        if let Some(methods) = context.domain.get_unigoal_methods(unigoal_params.var_name) {
            if is_verbose(3) {
                verbose_print(3, &format!("methods: {} methods", methods.len()));
            }

            for method in methods {
                if is_verbose(3) {
                    verbose_print(3, &format!("depth {depth} trying method: "));
                }

                if let Some(subgoals) = method(state, unigoal_params.arg, unigoal_params.value) {
                    if is_verbose(3) {
                        verbose_print(3, "applicable");
                        verbose_print(3, &format!("depth {} subgoals: {}", depth, todo_list_to_string(&subgoals)));
                    }

                    let mut new_todo = subgoals;
                    
                    // Add verification if enabled
                    if context.verify_goals {
                        let verification = vec![PlanItem::task("_verify_g", vec![
                            "method_name".into(),
                            unigoal_params.var_name.into(),
                            unigoal_params.arg.into(),
                            unigoal_params.value.clone(),
                            (depth as i64).into(),
                        ])];
                        new_todo.extend(verification);
                    }
                    
                    new_todo.extend(todo_list.clone());

                    let result = self.seek_plan(context, state.clone(), new_todo, plan.clone(), depth + 1)?;
                    if let PlanningResult::Success(_) = result {
                        return Ok(result);
                    }
                } else if is_verbose(3) {
                    verbose_print(3, "not applicable");
                }
            }
        }

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} could not achieve goal ({} {} {})", unigoal_params.var_name, unigoal_params.arg, unigoal_params.value));
        }
        Ok(PlanningResult::Failure)
    }

    fn refine_multigoal_and_continue(
        &self,
        context: &PlanningContext,
        state: &State,
        multigoal: &Multigoal,
        todo_list: TodoList,
        plan: Plan,
        depth: usize,
    ) -> Result<PlanningResult> {
        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} multigoal {multigoal}: "));
        }

        let methods = context.domain.get_multigoal_methods();
        if is_verbose(3) {
            verbose_print(3, &format!("methods: {} methods", methods.len()));
        }

        for method in methods {
            if is_verbose(3) {
                verbose_print(3, &format!("depth {depth} trying method: "));
            }

            if let Some(subgoals) = method(state, multigoal) {
                if is_verbose(3) {
                    verbose_print(3, "applicable");
                    verbose_print(3, &format!("depth {} subgoals: {}", depth, todo_list_to_string(&subgoals)));
                }

                let mut new_todo = subgoals;
                
                // Add verification if enabled
                if context.verify_goals {
                    let verification = vec![PlanItem::task("_verify_mg", vec![
                        "method_name".into(),
                        serde_json::to_value(multigoal).unwrap_or_default(),
                        (depth as i64).into(),
                    ])];
                    new_todo.extend(verification);
                }
                
                new_todo.extend(todo_list.clone());

                let result = self.seek_plan(context, state.clone(), new_todo, plan.clone(), depth + 1)?;
                if let PlanningResult::Success(_) = result {
                    return Ok(result);
                }
            } else if is_verbose(3) {
                verbose_print(3, "not applicable");
            }
        }

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} could not achieve multigoal {multigoal}"));
        }
        Ok(PlanningResult::Failure)
    }
}

/// Iterative planning strategy implementation
pub struct IterativeStrategy;

impl PlanningStrategyTrait for IterativeStrategy {
    fn seek_plan(
        &self,
        context: &PlanningContext,
        initial_state: State,
        initial_todo_list: TodoList,
        initial_plan: Plan,
        initial_depth: usize,
    ) -> Result<PlanningResult> {
        let mut stack = vec![(initial_state, initial_todo_list, initial_plan, initial_depth)];

        while let Some((state, todo_list, plan, depth)) = stack.pop() {
            if is_verbose(2) {
                let todo_string = todo_list_to_string(&todo_list);
                verbose_print(2, &format!("depth {depth} todo_list {todo_string}"));
            }

            // Base case: empty todo list means we're done
            if todo_list.is_empty() {
                if is_verbose(3) {
                    verbose_print(3, &format!("depth {depth} no more tasks or goals, return plan"));
                }
                return Ok(PlanningResult::Success(plan));
            }

            let item = &todo_list[0];
            let remaining_todo = todo_list[1..].to_vec();

            match item {
                PlanItem::Multigoal(multigoal) => {
                    if let Some(new_state_info) = self.refine_multigoal_iterative(context, &state, multigoal, remaining_todo, plan, depth)? {
                        stack.push(new_state_info);
                    }
                }
                PlanItem::Task(task_name, args) => {
                    if context.domain.has_action(task_name) {
                        let action_params = ActionParams { action_name: task_name, args };
                        let planning_state = PlanningState { todo_list: remaining_todo, plan, depth };
                        if let Some(new_state_info) = self.apply_action_iterative(context, &state, &action_params, planning_state)? {
                            stack.push(new_state_info);
                        }
                    } else if context.domain.has_task_methods(task_name) {
                        let task_params = TaskParams { task_name, args };
                        let planning_state = PlanningState { todo_list: remaining_todo, plan, depth };
                        if let Some(new_state_info) = self.refine_task_iterative(context, &state, &task_params, planning_state)? {
                            stack.push(new_state_info);
                        }
                    } else {
                        return Err(GTRustHopError::invalid_item_type(item_to_string(item), depth));
                    }
                }
                PlanItem::Action(action_name, args) => {
                    let action_params = ActionParams { action_name, args };
                    let planning_state = PlanningState { todo_list: remaining_todo, plan, depth };
                    if let Some(new_state_info) = self.apply_action_iterative(context, &state, &action_params, planning_state)? {
                        stack.push(new_state_info);
                    }
                }
                PlanItem::Unigoal(var_name, arg, value) => {
                    if context.domain.has_unigoal_methods(var_name) {
                        let unigoal_params = UnigoalParams { var_name, arg, value };
                        let planning_state = PlanningState { todo_list: remaining_todo, plan, depth };
                        if let Some(new_state_info) = self.refine_unigoal_iterative(context, &state, &unigoal_params, planning_state)? {
                            stack.push(new_state_info);
                        }
                    } else {
                        return Err(GTRustHopError::invalid_item_type(item_to_string(item), depth));
                    }
                }
            }
        }

        Ok(PlanningResult::Failure)
    }
}

impl IterativeStrategy {
    fn apply_action_iterative(
        &self,
        context: &PlanningContext,
        state: &State,
        action_params: &ActionParams,
        planning_state: PlanningState,
    ) -> Result<Option<(State, TodoList, Plan, usize)>> {
        let PlanningState { todo_list, mut plan, depth } = planning_state;

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} action {}: ", action_params.action_name));
        }

        if let Some(action_fn) = context.domain.get_action(action_params.action_name) {
            let mut new_state = state.copy(None);
            if let Some(result_state) = action_fn(&mut new_state, action_params.args) {
                if is_verbose(3) {
                    verbose_print(3, "applied");
                    result_state.display(None);
                }
                plan.push(PlanItem::action(action_params.action_name, action_params.args.to_vec()));
                return Ok(Some((result_state, todo_list, plan, depth + 1)));
            }
        }

        if is_verbose(3) {
            verbose_print(3, "not applicable");
        }
        Ok(None)
    }

    fn refine_task_iterative(
        &self,
        context: &PlanningContext,
        state: &State,
        task_params: &TaskParams,
        planning_state: PlanningState,
    ) -> Result<Option<(State, TodoList, Plan, usize)>> {
        let PlanningState { todo_list, plan, depth } = planning_state;

        if let Some(methods) = context.domain.get_task_methods(task_params.task_name) {
            if is_verbose(3) {
                verbose_print(3, &format!("depth {} task {} methods: {} methods", depth, task_params.task_name, methods.len()));
            }

            for method in methods {
                if is_verbose(3) {
                    verbose_print(3, &format!("depth {depth} trying method: "));
                }

                if let Some(subtasks) = method(state, task_params.args) {
                    if is_verbose(3) {
                        verbose_print(3, "applicable");
                        verbose_print(3, &format!("depth {} subtasks: {}", depth, todo_list_to_string(&subtasks)));
                    }

                    let mut new_todo = subtasks;
                    new_todo.extend(todo_list);
                    
                    return Ok(Some((state.clone(), new_todo, plan, depth + 1)));
                } else if is_verbose(3) {
                    verbose_print(3, "not applicable");
                }
            }
        }

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} could not accomplish task {}", task_params.task_name));
        }
        Ok(None)
    }

    fn refine_unigoal_iterative(
        &self,
        context: &PlanningContext,
        state: &State,
        unigoal_params: &UnigoalParams,
        planning_state: PlanningState,
    ) -> Result<Option<(State, TodoList, Plan, usize)>> {
        let PlanningState { todo_list, plan, depth } = planning_state;

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} goal ({} {} {}): ", unigoal_params.var_name, unigoal_params.arg, unigoal_params.value));
        }

        // Check if goal is already achieved
        if state.satisfies_unigoal(unigoal_params.var_name, unigoal_params.arg, unigoal_params.value) {
            if is_verbose(3) {
                verbose_print(3, "already achieved");
            }
            return Ok(Some((state.clone(), todo_list, plan, depth + 1)));
        }

        if let Some(methods) = context.domain.get_unigoal_methods(unigoal_params.var_name) {
            if is_verbose(3) {
                verbose_print(3, &format!("methods: {} methods", methods.len()));
            }

            for method in methods {
                if is_verbose(3) {
                    verbose_print(3, &format!("depth {depth} trying method: "));
                }

                if let Some(subgoals) = method(state, unigoal_params.arg, unigoal_params.value) {
                    if is_verbose(3) {
                        verbose_print(3, "applicable");
                        verbose_print(3, &format!("depth {} subgoals: {}", depth, todo_list_to_string(&subgoals)));
                    }

                    let mut new_todo = subgoals;
                    
                    // Add verification if enabled
                    if context.verify_goals {
                        let verification = vec![PlanItem::task("_verify_g", vec![
                            "method_name".into(),
                            unigoal_params.var_name.into(),
                            unigoal_params.arg.into(),
                            unigoal_params.value.clone(),
                            (depth as i64).into(),
                        ])];
                        new_todo.extend(verification);
                    }
                    
                    new_todo.extend(todo_list);
                    
                    return Ok(Some((state.clone(), new_todo, plan, depth + 1)));
                } else if is_verbose(3) {
                    verbose_print(3, "not applicable");
                }
            }
        }

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} could not achieve goal ({} {} {})", unigoal_params.var_name, unigoal_params.arg, unigoal_params.value));
        }
        Ok(None)
    }

    fn refine_multigoal_iterative(
        &self,
        context: &PlanningContext,
        state: &State,
        multigoal: &Multigoal,
        todo_list: TodoList,
        plan: Plan,
        depth: usize,
    ) -> Result<Option<(State, TodoList, Plan, usize)>> {
        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} multigoal {multigoal}: "));
        }

        let methods = context.domain.get_multigoal_methods();
        if is_verbose(3) {
            verbose_print(3, &format!("methods: {} methods", methods.len()));
        }

        for method in methods {
            if is_verbose(3) {
                verbose_print(3, &format!("depth {depth} trying method: "));
            }

            if let Some(subgoals) = method(state, multigoal) {
                if is_verbose(3) {
                    verbose_print(3, "applicable");
                    verbose_print(3, &format!("depth {} subgoals: {}", depth, todo_list_to_string(&subgoals)));
                }

                let mut new_todo = subgoals;
                
                // Add verification if enabled
                if context.verify_goals {
                    let verification = vec![PlanItem::task("_verify_mg", vec![
                        "method_name".into(),
                        serde_json::to_value(multigoal).unwrap_or_default(),
                        (depth as i64).into(),
                    ])];
                    new_todo.extend(verification);
                }
                
                new_todo.extend(todo_list);
                
                return Ok(Some((state.clone(), new_todo, plan, depth + 1)));
            } else if is_verbose(3) {
                verbose_print(3, "not applicable");
            }
        }

        if is_verbose(3) {
            verbose_print(3, &format!("depth {depth} could not achieve multigoal {multigoal}"));
        }
        Ok(None)
    }
}
