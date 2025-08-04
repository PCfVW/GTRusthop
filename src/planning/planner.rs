//! Main planning functions for GTRusthop

use super::get_planning_context;
use crate::core::{State, Domain, PlanItem, TodoList, StateValue};
use crate::error::Result;
use std::sync::Arc;








/// Helper function to create a simple action
pub fn create_simple_action<F>(action_fn: F) -> impl Fn(&mut State, &[StateValue]) -> Option<State>
where
    F: Fn(&mut State, &[StateValue]) -> bool,
{
    move |state: &mut State, args: &[StateValue]| {
        if action_fn(state, args) {
            Some(state.clone())
        } else {
            None
        }
    }
}

/// Helper function to create a simple task method
pub fn create_simple_task_method<F>(method_fn: F) -> impl Fn(&State, &[StateValue]) -> Option<TodoList>
where
    F: Fn(&State, &[StateValue]) -> Option<Vec<PlanItem>>,
{
    move |state: &State, args: &[StateValue]| method_fn(state, args)
}

/// Helper function to create a simple unigoal method
pub fn create_simple_unigoal_method<F>(method_fn: F) -> impl Fn(&State, &str, &StateValue) -> Option<TodoList>
where
    F: Fn(&State, &str, &StateValue) -> Option<Vec<PlanItem>>,
{
    move |state: &State, arg: &str, value: &StateValue| method_fn(state, arg, value)
}

/// Helper function to create a simple multigoal method
pub fn create_simple_multigoal_method<F>(method_fn: F) -> impl Fn(&State, &crate::core::Multigoal) -> Option<TodoList>
where
    F: Fn(&State, &crate::core::Multigoal) -> Option<Vec<PlanItem>>,
{
    move |state: &State, multigoal: &crate::core::Multigoal| method_fn(state, multigoal)
}



/// Set the current domain for planning
pub fn set_current_domain(domain: Domain) -> Result<()> {
    let context = super::PlanningContext::new(Arc::new(domain));
    super::set_planning_context(context);
    Ok(())
}

/// Get the current domain
pub fn get_current_domain() -> Result<Arc<Domain>> {
    let context = get_planning_context()?;
    Ok(context.domain.clone())
}

/// Print information about the current domain
pub fn print_domain() -> Result<()> {
    let domain = get_current_domain()?;
    domain.display();
    Ok(())
}

/// Print all action names in the current domain
pub fn print_actions() -> Result<()> {
    let domain = get_current_domain()?;
    domain.print_actions();
    Ok(())
}

/// Print all command names in the current domain
pub fn print_commands() -> Result<()> {
    let domain = get_current_domain()?;
    domain.print_commands();
    Ok(())
}

/// Print all methods in the current domain
pub fn print_methods() -> Result<()> {
    let domain = get_current_domain()?;
    domain.print_methods();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{State, Domain};
    use crate::planning::PlannerBuilder;

    #[test]
    fn test_simple_planning() -> Result<()> {
        // Create a simple domain
        let mut domain = Domain::new("test_domain");
        
        // Add a simple action
        domain.declare_action("move", |state: &mut State, args: &[StateValue]| {
            if args.len() >= 2 {
                if let (Some(obj), Some(target)) = (args[0].as_str(), args[1].as_str()) {
                    state.set_var("loc", obj, target.into());
                    return Some(state.clone());
                }
            }
            None
        })?;

        // Add a simple task method
        domain.declare_task_method("transport", |state: &State, args: &[StateValue]| {
            if args.len() >= 2 {
                if let (Some(obj), Some(target)) = (args[0].as_str(), args[1].as_str()) {
                    if let Some(current_loc) = state.get_var("loc", obj) {
                        if current_loc.as_str() != Some(target) {
                            return Some(vec![PlanItem::action("move", vec![obj.into(), target.into()])]);
                        }
                    }
                    return Some(vec![]); // Already at target
                }
            }
            None
        })?;

        // Create planner with builder pattern
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .build()?;

        // Create initial state
        let mut state = State::new("initial_state");
        state.set_var("loc", "obj1", "loc1".into());

        // Create todo list
        let todo_list = vec![PlanItem::task("transport", vec!["obj1".into(), "loc2".into()])];

        // Find plan
        let plan = planner.find_plan(state, todo_list)?;
        
        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.len(), 1);
        
        if let PlanItem::Action(action_name, args) = &plan[0] {
            assert_eq!(action_name, "move");
            assert_eq!(args.len(), 2);
            assert_eq!(args[0].as_str(), Some("obj1"));
            assert_eq!(args[1].as_str(), Some("loc2"));
        } else {
            panic!("Expected action in plan");
        }

        Ok(())
    }
}
