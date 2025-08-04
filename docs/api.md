# GTRusthop API Reference

This document provides a comprehensive reference for the GTRusthop Goal-Task-Network planning library. Written for experienced developers who need detailed technical specifications and integration guidance.

**🔒 Thread-Safe Architecture**: GTRusthop uses a thread-safe builder pattern that eliminates race conditions and enables parallel planning without global state.

## Table of Contents

1. [Core Types and Structures](#core-types-and-structures)
2. [Planning Paradigms](#planning-paradigms)
3. [Thread-Safe Builder Pattern](#thread-safe-builder-pattern)
4. [Planning Functions](#planning-functions)
5. [Pyhop Compatibility API](#pyhop-compatibility-api)
6. [Lazy Lookahead Algorithm](#lazy-lookahead-algorithm)
7. [Domain Management](#domain-management)
8. [State Management](#state-management)
9. [Error Handling](#error-handling)
10. [Performance Considerations](#performance-considerations)
11. [Integration Guidelines](#integration-guidelines)

---

## Core Types and Structures

### StateValue

```rust
pub type StateValue = serde_json::Value;
```

**Description**: Core type for representing values in the planning system. Supports all JSON value types.

**Supported Types**:
- `String`: Text values
- `Number`: Integer and floating-point numbers
- `Bool`: Boolean values
- `Array`: Ordered collections
- `Object`: Key-value mappings
- `Null`: Null values

**Helper Functions**:
```rust
pub fn string_value(s: impl Into<String>) -> StateValue
pub fn int_value(i: i64) -> StateValue
pub fn float_value(f: f64) -> StateValue
pub fn bool_value(b: bool) -> StateValue
```

**Example**:
```rust
use gtrusthop::core::{string_value, int_value, bool_value};

let name = string_value("alice");
let age = int_value(25);
let active = bool_value(true);
```

### PlanItem

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlanItem {
    Task(String, Vec<StateValue>),
    Action(String, Vec<StateValue>),
    Unigoal(String, String, StateValue),
    Multigoal(Multigoal),
}
```

**Description**: Represents executable items in the planning system.

**Constructors**:
```rust
impl PlanItem {
    pub fn task(name: impl Into<String>, args: Vec<StateValue>) -> Self
    pub fn action(name: impl Into<String>, args: Vec<StateValue>) -> Self
    pub fn unigoal(state_var_name: impl Into<String>, arg: impl Into<String>, value: StateValue) -> Self
    pub fn multigoal(multigoal: Multigoal) -> Self
}
```

**Methods**:
```rust
impl PlanItem {
    pub fn name(&self) -> &str
    pub fn args(&self) -> Vec<StateValue>
    pub fn is_task(&self) -> bool
    pub fn is_action(&self) -> bool
    pub fn is_unigoal(&self) -> bool
    pub fn is_multigoal(&self) -> bool
}
```

**Example**:
```rust
use gtrusthop::{PlanItem, core::string_value};

// Create a task
let task = PlanItem::task("transport", vec![
    string_value("package"),
    string_value("warehouse"),
    string_value("customer")
]);

// Create an action
let action = PlanItem::action("move", vec![
    string_value("robot"),
    string_value("room1")
]);

// Create a unigoal
let goal = PlanItem::unigoal("location", "robot", string_value("room2"));
```

---

## Planning Paradigms

GTRusthop supports two distinct planning approaches that differ in their fundamental methodology:

### HTN (Hierarchical Task Network) Planning

**Philosophy**: Task-oriented, procedural approach - "How to do something"

**Key Characteristics**:
- Uses **task methods exclusively** (`declare_task_method`)
- Decomposes abstract tasks into subtasks and primitive actions
- Follows a hierarchical task decomposition structure
- Best for domains with well-defined procedures

**Domain Declaration**:
```rust
// HTN domain uses only task methods
domain.declare_task_method("travel", |state, args| {
    if args.len() >= 3 {
        if let (Some(person), Some(from), Some(to)) =
            (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
            return Some(vec![
                PlanItem::task("get_taxi", vec![string_value(person)]),
                PlanItem::task("ride_taxi", vec![string_value(person), string_value(from), string_value(to)]),
                PlanItem::task("pay_taxi", vec![string_value(person)])
            ]);
        }
    }
    None
})?;
```

**Planning Call**:
```rust
// HTN planning uses tasks
let todo_list = vec![PlanItem::task("travel", vec![
    string_value("alice"), string_value("home"), string_value("park")
])];
```

### HGN (Hierarchical Goal Network) Planning

**Philosophy**: Goal-oriented, declarative approach - "What to achieve"

**Key Characteristics**:
- Uses **multigoal methods** (`declare_multigoal_method`)
- Decomposes goals into subgoals and actions
- Focuses on desired states rather than procedures
- Best for domains with clear goal states

**Domain Declaration**:
```rust
// HGN domain uses multigoal methods
domain.declare_multigoal_method(|state, mgoal| {
    if let Some(target_loc) = mgoal.get_goal("loc", "alice") {
        return Some(vec![
            PlanItem::action("move", vec![string_value("alice"), target_loc.clone()]),
            // Additional subgoals would go here
        ]);
    }
    None
})?;
```

**Planning Call**:
```rust
// HGN planning uses multigoals
let mut goal = Multigoal::new("travel_goal");
goal.set_goal("loc", "alice", string_value("park"));
let todo_list = vec![PlanItem::multigoal(goal)];
```

### When to Use Each Paradigm

| Use HTN When | Use HGN When |
|--------------|--------------|
| You have clear procedures | You have clear goal states |
| Domain knowledge is procedural | Domain knowledge is declarative |
| Tasks have natural decomposition | Goals have natural decomposition |
| Examples: recipes, workflows | Examples: configuration, optimization |

---

### State

```rust
pub struct State {
    pub name: String,
    // Internal fields...
}
```

**Description**: Represents the current state of the world in the planning system.

**Constructor**:
```rust
impl State {
    pub fn new(name: impl Into<String>) -> Self
}
```

**Core Methods**:
```rust
impl State {
    // Variable management
    pub fn set_var(&mut self, var_name: &str, arg: &str, value: StateValue)
    pub fn get_var(&self, var_name: &str, arg: &str) -> Option<&StateValue>
    pub fn has_var(&self, var_name: &str) -> bool
    pub fn remove_var(&mut self, var_name: &str, arg: &str) -> Option<StateValue>
    
    // State operations
    pub fn copy(&self, new_name: Option<&str>) -> Self
    pub fn display(&self, var_names: Option<&[&str]>)
    
    // Goal satisfaction
    pub fn satisfies_unigoal(&self, var_name: &str, arg: &str, value: &StateValue) -> bool
    pub fn satisfies_multigoal(&self, multigoal: &Multigoal) -> bool
}
```

**Performance Notes**:
- `copy()` performs a deep clone - use sparingly in performance-critical code
- `set_var()` and `get_var()` are O(1) operations using HashMap internally
- `display()` is for debugging only - avoid in production code

**Example**:
```rust
use gtrusthop::{State, core::string_value};

let mut state = State::new("initial_state");

// Set variables
state.set_var("location", "robot", string_value("room1"));
state.set_var("battery", "robot", 85.0.into());
state.set_var("carrying", "robot", serde_json::Value::Null);

// Check variables
if let Some(location) = state.get_var("location", "robot") {
    println!("Robot is at: {}", location);
}

// Check goal satisfaction
let goal_satisfied = state.satisfies_unigoal("location", "robot", &string_value("room1"));
assert!(goal_satisfied);
```

### Domain

```rust
pub struct Domain {
    pub name: String,
    // Internal fields...
}
```

**Description**: Contains the domain knowledge including actions, task methods, and goal methods.

**Constructor**:
```rust
impl Domain {
    pub fn new(name: impl Into<String>) -> Self
}
```

**Action Management**:
```rust
impl Domain {
    pub fn declare_action<F>(&mut self, name: &str, action_fn: F) -> Result<()>
    where F: Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync + 'static

    pub fn get_action(&self, name: &str) -> Option<&ActionFn>
    pub fn has_action(&self, name: &str) -> bool
    pub fn action_names(&self) -> Vec<&str>
}
```

**Command Management** (for lazy lookahead execution):
```rust
impl Domain {
    pub fn declare_command<F>(&mut self, name: &str, command_fn: F) -> Result<()>
    where F: Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync + 'static

    pub fn get_command(&self, name: &str) -> Option<&CommandFn>
    pub fn has_command(&self, name: &str) -> bool
    pub fn command_names(&self) -> Vec<&str>
}
```

**Commands vs Actions**:
- **Actions**: Used during planning to determine what can be done
- **Commands**: Used during execution to actually perform actions
- **Naming Convention**: Commands typically named `c_{action_name}`
- **Failure Handling**: Commands can fail (return `None`) to trigger replanning

**Task Method Management**:
```rust
impl Domain {
    pub fn declare_task_method<F>(&mut self, task_name: &str, method_fn: F) -> Result<()>
    where F: Fn(&State, &[StateValue]) -> Option<Vec<PlanItem>> + Send + Sync + 'static
    
    pub fn get_task_methods(&self, task_name: &str) -> Option<&Vec<TaskMethodFn>>
    pub fn has_task_methods(&self, task_name: &str) -> bool
}
```

**Goal Method Management**:
```rust
impl Domain {
    pub fn declare_unigoal_method<F>(&mut self, var_name: &str, method_fn: F) -> Result<()>
    where F: Fn(&State, &str, &StateValue) -> Option<Vec<PlanItem>> + Send + Sync + 'static
    
    pub fn get_unigoal_methods(&self, var_name: &str) -> Option<&Vec<UnigoalMethodFn>>
    pub fn has_unigoal_methods(&self, var_name: &str) -> bool
}
```

**Example**:
```rust
use gtrusthop::{Domain, State, PlanItem, core::string_value};

let mut domain = Domain::new("logistics_domain");

// Declare an action (for planning)
domain.declare_action("move", |state: &mut State, args: &[StateValue]| {
    if args.len() >= 3 {
        if let (Some(obj), Some(from), Some(to)) = (
            args[0].as_str(),
            args[1].as_str(),
            args[2].as_str()
        ) {
            // Check preconditions
            if state.get_var("location", obj) == Some(&string_value(from)) {
                // Apply effects
                state.set_var("location", obj, string_value(to));
                return Some(state.clone());
            }
        }
    }
    None
})?;

// Declare corresponding command (for execution)
domain.declare_command("c_move", |state: &mut State, args: &[StateValue]| {
    if args.len() >= 3 {
        if let (Some(obj), Some(from), Some(to)) = (
            args[0].as_str(),
            args[1].as_str(),
            args[2].as_str()
        ) {
            // Check preconditions (might be different from planning)
            if state.get_var("location", obj) == Some(&string_value(from)) {
                // Simulate real-world execution - might fail
                if movement_possible(obj, from, to) {
                    state.set_var("location", obj, string_value(to));
                    return Some(state.clone());
                }
            }
        }
    }
    None // Command failed - will trigger replanning
})?;

// Declare a task method
domain.declare_task_method("transport", |state: &State, args: &[StateValue]| {
    if args.len() >= 3 {
        if let (Some(obj), Some(from), Some(to)) = (
            args[0].as_str(),
            args[1].as_str(),
            args[2].as_str()
        ) {
            // Check if already at destination
            if state.get_var("location", obj) == Some(&string_value(to)) {
                return Some(vec![]); // No actions needed
            }
            
            // Return subtasks
            return Some(vec![
                PlanItem::action("move", vec![
                    string_value(obj),
                    string_value(from),
                    string_value(to)
                ])
            ]);
        }
    }
    None
})?;
```

### Multigoal

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Multigoal {
    pub name: String,
    // Internal fields...
}
```

**Description**: Represents multiple goals that need to be achieved simultaneously. Used in HGN planning for complex goal configurations.

**Constructor**:
```rust
impl Multigoal {
    pub fn new(name: impl Into<String>) -> Self
}
```

**Goal Management**:
```rust
impl Multigoal {
    pub fn set_goal(&mut self, var_name: &str, arg: &str, value: StateValue)
    pub fn get_goal(&self, var_name: &str, arg: &str) -> Option<&StateValue>
    pub fn remove_goal(&mut self, var_name: &str, arg: &str) -> Option<StateValue>
    pub fn has_goal(&self, var_name: &str, arg: &str) -> bool
    pub fn clear_goals(&mut self)
}
```

**Display and Utility**:
```rust
impl Multigoal {
    pub fn display(&self, var_names: Option<&[&str]>)
    pub fn is_empty(&self) -> bool
    pub fn goal_count(&self) -> usize
}
```

**Example**:
```rust
// Create a multigoal for blocks world
let mut goal = Multigoal::new("stack_blocks");
goal.set_goal("pos", "a", string_value("b"));
goal.set_goal("pos", "b", string_value("c"));

// Check if a goal exists
if goal.has_goal("pos", "a") {
    println!("Block A has a position goal");
}

// Display the multigoal
goal.display(None);
```

---

## Hierarchical Goal Networks (HGN)

### Goal Methods

HGN planning uses **goal methods** to decompose goals into subgoals and actions. Unlike HTN task methods, goal methods focus on achieving specific state conditions.

#### Unigoal Methods

**Declaration**:
```rust
domain.declare_unigoal_method("var_name", |state: &State, arg: &str, value: &StateValue| {
    // Goal method implementation
    // Returns Some(Vec<PlanItem>) if goal can be achieved, None otherwise
})?;
```

**Parameters**:
- `state`: Current state of the world
- `arg`: The argument (e.g., object name) for the goal
- `value`: The desired value for the state variable

**Return Value**:
- `Some(Vec<PlanItem>)`: List of subgoals/actions to achieve the goal
- `None`: Goal cannot be achieved in current state

**Example - Logistics Domain**:
```rust
// Goal method for "at" goals (moving objects to locations)
domain.declare_unigoal_method("at", |state: &State, obj: &str, target_value: &StateValue| {
    if let Some(target_location) = target_value.as_str() {
        // Check if already at target
        if let Some(current_location) = state.get_var("at", obj) {
            if current_location.as_str() == Some(target_location) {
                return Some(vec![]); // Already satisfied
            }

            // Find transportation method
            if let Some(truck) = find_truck_in_city(state, get_city(state, target_location)) {
                return Some(vec![
                    PlanItem::unigoal("truck_at", &truck, current_location.clone()),
                    PlanItem::action("load_truck", vec![string_value(obj), string_value(&truck)]),
                    PlanItem::unigoal("truck_at", &truck, string_value(target_location)),
                    PlanItem::action("unload_truck", vec![string_value(obj), string_value(target_location)])
                ]);
            }
        }
    }
    None
})?;
```

#### Multigoal Methods

**Declaration**:
```rust
domain.declare_multigoal_method(|state: &State, mgoal: &Multigoal| {
    // Multigoal method implementation
    // Returns Some(Vec<PlanItem>) to work toward achieving all goals
})?;
```

**Parameters**:
- `state`: Current state of the world
- `mgoal`: The multigoal containing all goals to achieve

**Example - Blocks World**:
```rust
// Multigoal method for blocks world configurations
domain.declare_multigoal_method(|state: &State, mgoal: &Multigoal| {
    // Find blocks that can be moved to their final positions
    for block in get_clear_blocks(state) {
        let status = get_block_status(state, mgoal, &block);

        match status.as_str() {
            "move-to-block" => {
                if let Some(target) = mgoal.get_goal("pos", &block) {
                    return Some(vec![
                        PlanItem::unigoal("pos", &block, string_value("hand")),
                        PlanItem::unigoal("pos", &block, target.clone()),
                        PlanItem::Multigoal(mgoal.clone())
                    ]);
                }
            }
            "move-to-table" => {
                return Some(vec![
                    PlanItem::unigoal("pos", &block, string_value("hand")),
                    PlanItem::unigoal("pos", &block, string_value("table")),
                    PlanItem::Multigoal(mgoal.clone())
                ]);
            }
            _ => continue,
        }
    }

    Some(vec![]) // No more moves needed
})?;
```

### HGN Planning Process

1. **Goal Analysis**: Examine current state and desired goals
2. **Goal Decomposition**: Use goal methods to break down complex goals
3. **Subgoal Generation**: Create subgoals and actions to achieve main goals
4. **Recursive Planning**: Apply goal methods recursively until all goals are satisfied
5. **Action Execution**: Execute primitive actions to change state

### HGN vs HTN Comparison

| Aspect | HTN (Task-Oriented) | HGN (Goal-Oriented) |
|--------|---------------------|---------------------|
| **Focus** | How to perform tasks | What goals to achieve |
| **Methods** | Task methods | Goal methods |
| **Decomposition** | Tasks → Subtasks | Goals → Subgoals |
| **Planning Style** | Procedural | Declarative |
| **Best For** | Process domains | State-oriented domains |

### HGN Best Practices

#### 1. Goal Method Design
```rust
// Always check if goal is already satisfied
domain.declare_unigoal_method("location", |state, obj, target| {
    // Check satisfaction first
    if let Some(current) = state.get_var("location", obj) {
        if current == target {
            return Some(vec![]); // Already satisfied
        }
    }

    // Decompose goal into subgoals/actions
    // ...
})?;
```

#### 2. Multigoal Ordering
```rust
// Consider goal dependencies and conflicts
domain.declare_multigoal_method(|state, mgoal| {
    // Analyze which goals can be achieved without conflicts
    // Prioritize goals that enable other goals
    // Handle goal interactions carefully
})?;
```

#### 3. State Consistency
```rust
// Ensure actions maintain state consistency
domain.declare_action("move", |state, args| {
    // Validate preconditions
    if !preconditions_met(state, args) {
        return None;
    }

    // Apply effects consistently
    apply_effects(state, args);
    Some(state.clone())
})?;
```

---

## Thread-Safe Builder Pattern

### PlannerBuilder

```rust
pub struct PlannerBuilder {
    // Internal fields...
}
```

**Description**: Thread-safe builder for creating isolated planner instances. Eliminates race conditions and enables parallel planning.

**Constructor**:
```rust
impl PlannerBuilder {
    pub fn new() -> Self
}
```

**Configuration Methods**:
```rust
impl PlannerBuilder {
    pub fn with_domain(self, domain: Domain) -> Self
    pub fn with_verbose_level(self, level: i32) -> Result<Self>
    pub fn with_strategy(self, strategy: PlanningStrategy) -> Self
    pub fn with_goal_verification(self, enabled: bool) -> Self
    pub fn build(self) -> Result<Planner>
}
```

**Example**:
```rust
use gtrusthop::{PlannerBuilder, Domain, PlanningStrategy};

let domain = create_my_domain()?;

let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_verbose_level(1)?
    .with_strategy(PlanningStrategy::Iterative)
    .with_goal_verification(true)
    .build()?;
```

### Planner

```rust
pub struct Planner {
    // Internal fields...
}
```

**Description**: Thread-safe planner instance with isolated state. Each instance is completely independent.

**Planning Methods**:
```rust
impl Planner {
    pub fn find_plan(&self, state: State, todo_list: Vec<PlanItem>) -> Result<Option<Vec<PlanItem>>>
    pub fn run_lazy_lookahead(&self, state: State, todo_list: Vec<PlanItem>, max_tries: usize) -> Result<State>
    pub fn is_verbose(&self, level: i32) -> bool
}
```

---

## Planning Functions

### find_plan (Instance Method)

```rust
impl Planner {
    pub fn find_plan(&self, state: State, todo_list: Vec<PlanItem>) -> Result<Option<Vec<PlanItem>>>
}
```

**Description**: Main planning function that finds a sequence of actions to achieve the given goals/tasks.

**Parameters**:
- `state`: Initial state of the world
- `todo_list`: List of tasks, goals, or actions to achieve

**Returns**:
- `Ok(Some(plan))`: Planning succeeded with the given action sequence
- `Ok(None)`: No plan could be found
- `Err(error)`: Planning failed due to an error

**Thread Safety**: ✅ Each planner instance is completely isolated - no race conditions.

**Performance Characteristics**:
- Time complexity: Exponential in worst case (depends on domain complexity)
- Space complexity: O(depth × branching_factor)
- Iterative strategy generally more memory-efficient than recursive

**Example**:
```rust
use gtrusthop::{PlannerBuilder, State, PlanItem, core::string_value};

// Create planner with builder pattern
let domain = create_my_domain()?;
let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_verbose_level(1)?
    .build()?;

// Create initial state
let mut state = State::new("initial");
state.set_var("location", "package", string_value("warehouse"));

// Define goals
let todo_list = vec![
    PlanItem::task("transport", vec![
        string_value("package"),
        string_value("warehouse"),
        string_value("customer")
    ])
];

// Find plan
match planner.find_plan(state, todo_list)? {
    Some(plan) => {
        println!("Found plan with {} actions:", plan.len());
        for (i, action) in plan.iter().enumerate() {
            println!("  {}: {}", i + 1, action);
        }
    }
    None => println!("No plan found"),
}
```

### Planning Strategy Configuration

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanningStrategy {
    Recursive,
    Iterative,
}
```

**Description**: Planning algorithm strategy configuration.

**Strategies**:
- `Recursive`: Uses call stack for planning (simpler but limited by stack size)
- `Iterative`: Uses explicit stack (recommended for deep planning)

**Configuration via Builder**:
```rust
use gtrusthop::{PlannerBuilder, PlanningStrategy};

let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_strategy(PlanningStrategy::Iterative) // Recommended
    .build()?;
```

**Global Configuration** (still available for compatibility):
```rust
use gtrusthop::set_verbose_level;

// Enable detailed debugging globally
set_verbose_level(2)?;

// Disable all output globally
set_verbose_level(0)?;
```

---

## Pyhop Compatibility API

GTRusthop provides backward compatibility with the original Pyhop planner through the `pyhop()` function.

### pyhop (Module Function)

```rust
pub fn pyhop(domain: Domain, state: State, todo_list: Vec<PlanItem>) -> Result<Option<Vec<PlanItem>>>
```

**Description**: Legacy compatibility function that provides the same interface as the original Pyhop planner.

**Parameters**:
- `domain`: Planning domain containing actions and methods
- `state`: Initial state of the world
- `todo_list`: List of tasks, goals, or actions to achieve

**Returns**:
- Same as `find_plan()`: `Ok(Some(plan))`, `Ok(None)`, or `Err(error)`

**Deprecation Notice**: When `verbose_level > 0`, displays a message encouraging migration to the modern API.

**Thread Safety**: ✅ Creates isolated planner instance internally - no race conditions.

**Example**:
```rust
use gtrusthop::{pyhop, Domain, State, PlanItem, core::string_value};

// Legacy Pyhop-style usage
let mut domain = Domain::new("transport_domain");
// ... declare actions and methods ...

let mut state = State::new("initial");
state.set_var("loc", "package", string_value("warehouse"));

let todo_list = vec![
    PlanItem::task("transport", vec![
        string_value("package"),
        string_value("customer")
    ])
];

// Shows deprecation message if verbose > 0
match pyhop(domain, state, todo_list)? {
    Some(plan) => println!("Found plan: {:?}", plan),
    None => println!("No plan found"),
}
```

### pyhop (Instance Method)

```rust
impl Planner {
    pub fn pyhop(&self, state: State, todo_list: Vec<PlanItem>) -> Result<Option<Vec<PlanItem>>>
}
```

**Description**: Instance method version of the pyhop compatibility function.

**Example**:
```rust
let planner = PlannerBuilder::new().with_domain(domain).build()?;

// Legacy compatibility method
let plan = planner.pyhop(state, todo_list)?;

// Modern method (recommended)
let plan = planner.find_plan(state, todo_list)?;
```

### Migration Guide

**From Original Pyhop**:
```python
# Python Pyhop
import pyhop
plan = pyhop.pyhop(state, todo_list)
```

**To GTRusthop (Legacy)**:
```rust
// Rust GTRusthop (compatibility)
use gtrusthop::pyhop;
let plan = pyhop(domain, state, todo_list)?;
```

**To GTRusthop (Modern)**:
```rust
// Rust GTRusthop (recommended)
use gtrusthop::PlannerBuilder;
let planner = PlannerBuilder::new().with_domain(domain).build()?;
let plan = planner.find_plan(state, todo_list)?;
```

**Benefits of Modern API**:
- **Better Control**: Configure verbose level, planning strategy per instance
- **Thread Safety**: Explicit isolation, no global state
- **Performance**: Reuse planner instances for multiple planning calls
- **Flexibility**: Mix different configurations in the same application

---

## Lazy Lookahead Algorithm

### run_lazy_lookahead

```rust
impl Planner {
    pub fn run_lazy_lookahead(
        &self,
        state: State,
        todo_list: Vec<PlanItem>,
        max_tries: usize,
    ) -> Result<State>
}
```

**Description**: Implementation of the lazy lookahead algorithm from Ghallab et al. (2016), "Automated Planning and Acting". Combines planning and acting by executing plans step-by-step and replanning when commands fail.

**Algorithm Overview**:
1. **Plan**: Generate a plan for the current state and goals
2. **Execute**: Try to execute each action in the plan as a command
3. **Replan**: If a command fails, generate a new plan and try again
4. **Repeat**: Continue until goals are achieved or max attempts reached

**Parameters**:
- `state`: Initial state of the world
- `todo_list`: List of tasks, goals, or actions to achieve
- `max_tries`: Maximum number of planning attempts (prevents infinite loops)

**Returns**:
- `Ok(final_state)`: Final state after successful execution or max tries reached
- `Err(error)`: Critical error during planning or execution

**Key Features**:
- **Command vs Action Distinction**: Actions are for planning, commands (prefixed with `c_`) are for execution
- **Failure Handling**: Automatic replanning when commands fail during execution
- **Verbose Output**: Detailed logging of planning and execution steps
- **Thread-Safe**: Works with the builder pattern architecture

**Command Execution Logic**:
```rust
// For each action in plan:
// 1. Try to find command "c_{action_name}"
// 2. If not found, fall back to action function
// 3. Execute on state copy
// 4. If execution fails, trigger replanning
// 5. If execution succeeds, update state and continue
```

**Example**:
```rust
use gtrusthop::{PlannerBuilder, Domain, State, PlanItem, core::string_value};

// Create domain with actions and commands
let mut domain = Domain::new("taxi_domain");

// Add action (for planning)
domain.declare_action("call_taxi", |state, args| {
    // Planning logic - assume taxi always available
    Some(state.clone())
})?;

// Add command (for execution)
domain.declare_command("c_call_taxi", |state, args| {
    // Execution logic - might fail in real world
    if taxi_available() {
        Some(state.clone())
    } else {
        None // Command failed - will trigger replanning
    }
})?;

// Create planner
let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_verbose_level(1)?
    .build()?;

// Create initial state
let mut state = State::new("initial");
state.set_var("location", "alice", string_value("home"));

// Define goals
let todo_list = vec![
    PlanItem::task("travel", vec![
        string_value("alice"),
        string_value("home"),
        string_value("work")
    ])
];

// Run lazy lookahead
let final_state = planner.run_lazy_lookahead(state, todo_list, 10)?;

println!("Final location: {:?}", final_state.get_var("location", "alice"));
```

**Verbose Output Example**:
```
RLL> run_lazy_lookahead, verbose = 1, max_tries = 10
RLL> initial state: initial
RLL> To do: [Task("travel", ["alice", "home", "work"])]
RLL> 1st call to find_plan:
FP> result = [Action("call_taxi", ["alice", "home"]), Action("ride_taxi", ["alice", "home", "work"])]
RLL> Command: c_call_taxi ["alice", "home"]
RLL> Command: c_ride_taxi ["alice", "home", "work"]
RLL> Plan ended; will call find_plan again.
RLL> 2nd call to find_plan:
FP> result = []
RLL> Empty plan => success after 2 calls to find_plan.
```

**Error Handling**:
```rust
match planner.run_lazy_lookahead(state, todo_list, 5) {
    Ok(final_state) => {
        println!("Execution completed successfully");
        final_state.display(None);
    }
    Err(e) => {
        eprintln!("Lazy lookahead failed: {}", e);
        // Handle error appropriately
    }
}
```

**Benefits**:
- **Robust Execution**: Handles real-world command failures gracefully
- **Adaptive Planning**: Automatically adjusts plans based on execution results
- **Real-World Ready**: Bridges the gap between planning and acting
- **Debugging Support**: Verbose output helps understand execution flow

---

## Error Handling

### GTRustHopError

```rust
#[derive(Debug, thiserror::Error)]
pub enum GTRustHopError {
    #[error("Invalid verbose level: {level}. Must be 0-3")]
    InvalidVerboseLevel { level: i32 },
    
    #[error("Method verification failed for method '{method}' with goal '{goal}' at depth {depth}")]
    MethodVerificationFailed { method: String, goal: String, depth: usize },
    
    #[error("Multigoal verification failed for method '{method}' with multigoal '{multigoal}' at depth {depth}")]
    MultigoalVerificationFailed { method: String, multigoal: String, depth: usize },
    
    #[error("Invalid item type '{item}' at depth {depth}")]
    InvalidItemType { item: String, depth: usize },
    
    #[error("Generic error: {message}")]
    Generic { message: String },
}
```

**Description**: Comprehensive error type covering all planning failures.

**Error Handling Best Practices**:
```rust
use gtrusthop::{find_plan, GTRustHopError};

match find_plan(state, todo_list) {
    Ok(Some(plan)) => {
        // Handle successful planning
        execute_plan(plan);
    }
    Ok(None) => {
        // Handle planning failure - no solution exists
        log::warn!("No plan could be found for the given goals");
        fallback_strategy();
    }
    Err(GTRustHopError::InvalidVerboseLevel { level }) => {
        // Handle configuration error
        log::error!("Invalid verbose level: {}", level);
    }
    Err(GTRustHopError::MethodVerificationFailed { method, goal, depth }) => {
        // Handle domain logic error
        log::error!("Method '{}' failed to achieve goal '{}' at depth {}", method, goal, depth);
    }
    Err(e) => {
        // Handle other errors
        log::error!("Planning failed: {}", e);
    }
}
```

---

## Performance Considerations

### Memory Usage

**State Management**:
- States use `HashMap<String, HashMap<String, StateValue>>` internally
- Deep copying states can be expensive - minimize `state.copy()` calls
- Consider using `Arc<State>` for read-only sharing

**Planning Strategies**:
- Iterative strategy: O(depth) memory usage
- Recursive strategy: Limited by system stack size (~1MB typically)

### Time Complexity

**Planning Complexity**:
- Best case: O(n) where n is plan length
- Average case: O(b^d) where b is branching factor, d is depth
- Worst case: Exponential in domain complexity

**Optimization Strategies**:
```rust
// 1. Minimize state copying
let new_state = if action_modifies_state {
    let mut state_copy = state.copy(None);
    apply_action(&mut state_copy, action);
    state_copy
} else {
    state // Reuse existing state
};

// 2. Use efficient goal checking
if state.satisfies_unigoal(var_name, arg, value) {
    return Ok(Some(vec![])); // Goal already satisfied
}

// 3. Implement early termination in methods
domain.declare_task_method("transport", |state, args| {
    // Quick feasibility check
    if !is_feasible(state, args) {
        return None; // Fail fast
    }
    // ... rest of method
});
```

### Concurrent Planning

**Thread Safety**: ✅ **Complete Thread Safety with Builder Pattern**
- `PlannerBuilder` and `Planner` are completely thread-safe
- No global state - each planner instance is isolated
- No race conditions - perfect for parallel execution
- `State` is not thread-safe (clone for parallel access)

**Parallel Planning Example**:
```rust
use std::thread;
use gtrusthop::{PlannerBuilder, PlanningStrategy};

let domain = create_domain()?;
let initial_state = create_initial_state();

// Spawn multiple planning threads - no race conditions!
let handles: Vec<_> = (0..4).map(|i| {
    let domain_clone = domain.clone(); // Domain is cheaply cloneable
    let state_clone = initial_state.copy(Some(&format!("state_{}", i)));

    thread::spawn(move || {
        // Each thread gets its own isolated planner instance
        let planner = PlannerBuilder::new()
            .with_domain(domain_clone)
            .with_verbose_level(0).unwrap() // Avoid mixed output
            .with_strategy(PlanningStrategy::Iterative)
            .build().unwrap();

        // No global state - completely isolated planning
        planner.find_plan(state_clone, create_todo_list(i))
    })
}).collect();

// Collect results
for (i, handle) in handles.into_iter().enumerate() {
    match handle.join().unwrap() {
        Ok(Some(plan)) => println!("Thread {}: Found plan with {} actions", i, plan.len()),
        Ok(None) => println!("Thread {}: No plan found", i),
        Err(e) => println!("Thread {}: Error: {}", i, e),
    }
}
```

**Lazy Lookahead in Parallel**:
```rust
use std::thread;

let domain = create_domain_with_commands()?;
let initial_state = create_initial_state();

// Multiple agents acting in parallel
let handles: Vec<_> = (0..3).map(|agent_id| {
    let domain_clone = domain.clone();
    let mut state_clone = initial_state.copy(Some(&format!("agent_{}", agent_id)));

    // Each agent starts at different location
    state_clone.set_var("location", &format!("agent_{}", agent_id),
                       string_value(&format!("start_{}", agent_id)));

    thread::spawn(move || {
        let planner = PlannerBuilder::new()
            .with_domain(domain_clone)
            .with_verbose_level(1).unwrap()
            .build().unwrap();

        let goals = vec![PlanItem::task("transport", vec![
            string_value(&format!("agent_{}", agent_id)),
            string_value(&format!("start_{}", agent_id)),
            string_value("common_goal")
        ])];

        // Each agent plans and acts independently
        planner.run_lazy_lookahead(state_clone, goals, 10)
    })
}).collect();

// Collect final states
for (i, handle) in handles.into_iter().enumerate() {
    match handle.join().unwrap() {
        Ok(final_state) => {
            println!("Agent {} final state:", i);
            final_state.display(None);
        }
        Err(e) => println!("Agent {} failed: {}", i, e),
    }
}
```

---

## Integration Guidelines

### Project Setup

**Cargo.toml** (same for all platforms):
```toml
[dependencies]
gtrusthop = "1.2.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Project Initialization:**

**Windows (PowerShell/Command Prompt):**
```powershell
# Create new project
cargo new my_planning_project
cd my_planning_project

# Add GTRusthop dependency
cargo add gtrusthop
cargo add serde --features derive
cargo add serde_json

# Build project
cargo build
```

**Linux/macOS (bash/zsh):**
```bash
# Create new project
cargo new my_planning_project
cd my_planning_project

# Add GTRusthop dependency
cargo add gtrusthop
cargo add serde --features derive
cargo add serde_json

# Build project
cargo build
```

### Domain Design Patterns

**1. Layered Domain Architecture**:
```rust
// High-level domain
pub fn create_logistics_domain() -> Result<Domain> {
    let mut domain = Domain::new("logistics");
    
    // Add primitive actions
    add_movement_actions(&mut domain)?;
    add_manipulation_actions(&mut domain)?;
    
    // Add compound tasks
    add_transport_methods(&mut domain)?;
    add_delivery_methods(&mut domain)?;
    
    // Add goal methods
    add_location_goals(&mut domain)?;
    
    Ok(domain)
}

// Modular action groups
fn add_movement_actions(domain: &mut Domain) -> Result<()> {
    domain.declare_action("move", move_action)?;
    domain.declare_action("drive", drive_action)?;
    Ok(())
}
```

**2. State Validation**:
```rust
impl State {
    pub fn validate(&self) -> Result<()> {
        // Check state invariants
        for (var_name, var_map) in &self.variables {
            match var_name.as_str() {
                "location" => validate_locations(var_map)?,
                "capacity" => validate_capacities(var_map)?,
                _ => {}
            }
        }
        Ok(())
    }
}
```

**3. Error Recovery**:
```rust
pub fn robust_planning(state: State, goals: Vec<PlanItem>) -> Result<Vec<PlanItem>> {
    // Try primary strategy
    match find_plan(state.clone(), goals.clone()) {
        Ok(Some(plan)) => return Ok(plan),
        Ok(None) => {
            // Try relaxed goals
            let relaxed_goals = relax_goals(goals);
            if let Ok(Some(plan)) = find_plan(state.clone(), relaxed_goals) {
                return Ok(plan);
            }
        }
        Err(e) => {
            log::warn!("Primary planning failed: {}", e);
        }
    }
    
    // Fallback to greedy approach
    greedy_planning(state, goals)
}
```

### Testing Strategies

**Unit Testing Domains**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_action() {
        let mut domain = Domain::new("test");
        domain.declare_action("move", move_action).unwrap();

        let mut state = State::new("test_state");
        state.set_var("location", "robot", string_value("room1"));

        let action_fn = domain.get_action("move").unwrap();
        let result = action_fn(&mut state, &[
            string_value("robot"),
            string_value("room1"),
            string_value("room2")
        ]);

        assert!(result.is_some());
        let new_state = result.unwrap();
        assert_eq!(
            new_state.get_var("location", "robot"),
            Some(&string_value("room2"))
        );
    }

    #[test]
    fn test_planning_integration() {
        let domain = create_test_domain().unwrap();
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0).unwrap()
            .build().unwrap();

        let state = create_test_state();
        let goals = vec![PlanItem::unigoal("location", "robot", string_value("goal"))];

        let plan = planner.find_plan(state, goals).unwrap();
        assert!(plan.is_some());

        let plan = plan.unwrap();
        assert!(!plan.is_empty());
        assert!(plan.iter().all(|item| item.is_action()));
    }

    #[test]
    fn test_lazy_lookahead_integration() {
        let domain = create_test_domain_with_commands().unwrap();
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0).unwrap()
            .build().unwrap();

        let state = create_test_state();
        let goals = vec![PlanItem::task("transport", vec![
            string_value("robot"),
            string_value("start"),
            string_value("goal")
        ])];

        let final_state = planner.run_lazy_lookahead(state, goals, 5).unwrap();
        assert_eq!(
            final_state.get_var("location", "robot"),
            Some(&string_value("goal"))
        );
    }
}
```

**Running Tests:**

**Windows (PowerShell/Command Prompt):**
```powershell
# Run all tests
cargo test

# Run specific test
cargo test test_move_action

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release

# Generate test coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

**Linux/macOS (bash/zsh):**
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_move_action

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release

# Generate test coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Running Individual Examples

GTRusthop includes several example modules that demonstrate different planning scenarios. These examples are organized as library modules with test functions, making them easy to execute individually.

**Available Example Modules:**

| Module | Description | Test Command |
|--------|-------------|--------------|
| `simple_htn_example` | Basic HTN planning with travel scenarios | `cargo test simple_htn_example` |
| `blocks_htn_example` | Classic blocks world planning problems | `cargo test blocks_htn_example` |
| `simple_hgn_example` | Hierarchical Goal Network examples | `cargo test simple_hgn_example` |
| `lazy_lookahead_example` | Planning and acting with command failures | `cargo test lazy_lookahead_example` |
| `regression_tests` | Comprehensive regression test suite | `cargo test regression_tests` |

**Windows (PowerShell/Command Prompt):**
```powershell
# Run individual example modules
GTRusthop> cargo test simple_htn_example
GTRusthop> cargo test blocks_htn_example
GTRusthop> cargo test simple_hgn_example
GTRusthop> cargo test lazy_lookahead_example
GTRusthop> cargo test regression_tests

# Run with detailed output
GTRusthop> cargo test simple_htn_example -- --nocapture

# Run specific test within a module
GTRusthop> cargo test simple_htn_example::tests::test_run_simple_htn_examples
```

**Linux/macOS (bash/zsh):**
```bash
# Run individual example modules
GTRusthop$ cargo test simple_htn_example
GTRusthop$ cargo test blocks_htn_example
GTRusthop$ cargo test simple_hgn_example
GTRusthop$ cargo test lazy_lookahead_example
GTRusthop$ cargo test regression_tests

# Run with detailed output
GTRusthop$ cargo test simple_htn_example -- --nocapture

# Run specific test within a module
GTRusthop$ cargo test simple_htn_example::tests::test_run_simple_htn_examples
```

**Integration with Development Workflow:**
```rust
// Example: Running examples programmatically in your code
use gtrusthop::examples::{
    run_simple_htn_examples,
    run_blocks_htn_examples,
    run_simple_hgn_examples,
    run_lazy_lookahead_examples,
    run_regression_tests
};

fn run_all_examples() -> gtrusthop::Result<()> {
    println!("Running all GTRusthop examples...");

    run_simple_htn_examples()?;
    run_blocks_htn_examples()?;
    run_simple_hgn_examples()?;
    run_lazy_lookahead_examples()?;
    run_regression_tests()?;

    println!("All examples completed successfully!");
    Ok(())
}
```

### Production Deployment

**Configuration Management**:
```rust
#[derive(serde::Deserialize)]
pub struct PlanningConfig {
    pub strategy: PlanningStrategy,
    pub verbose_level: i32,
    pub max_depth: usize,
    pub timeout_ms: u64,
}

pub fn configure_planning(config: &PlanningConfig) -> Result<()> {
    set_planning_strategy(config.strategy)?;
    set_verbose_level(config.verbose_level)?;
    Ok(())
}
```

**Configuration File Examples:**

**Windows (config.toml):**
```toml
# Windows-specific paths use backslashes
strategy = "Iterative"
verbose_level = 1
max_depth = 100
timeout_ms = 5000

[paths]
domain_dir = "C:\\MyProject\\domains"
output_dir = "C:\\MyProject\\output"
log_file = "C:\\MyProject\\logs\\planning.log"
```

**Linux/macOS (config.toml):**
```toml
# Unix-like paths use forward slashes
strategy = "Iterative"
verbose_level = 1
max_depth = 100
timeout_ms = 5000

[paths]
domain_dir = "/home/user/myproject/domains"
output_dir = "/home/user/myproject/output"
log_file = "/home/user/myproject/logs/planning.log"
```

**Loading Configuration:**

**Windows (PowerShell/Command Prompt):**
```powershell
# Set environment variables
$env:GTRUSTHOP_CONFIG = "C:\MyProject\config.toml"
$env:GTRUSTHOP_LOG_LEVEL = "debug"

# Run with configuration
cargo run
```

**Linux/macOS (bash/zsh):**
```bash
# Set environment variables
export GTRUSTHOP_CONFIG="/home/user/myproject/config.toml"
export GTRUSTHOP_LOG_LEVEL="debug"

# Run with configuration
cargo run
```

**Monitoring and Metrics**:
```rust
use std::time::Instant;

pub struct PlanningMetrics {
    pub planning_time: std::time::Duration,
    pub plan_length: usize,
    pub nodes_explored: usize,
}

pub fn plan_with_metrics(state: State, goals: Vec<PlanItem>) -> Result<(Option<Vec<PlanItem>>, PlanningMetrics)> {
    let start = Instant::now();
    
    let result = find_plan(state, goals)?;
    
    let metrics = PlanningMetrics {
        planning_time: start.elapsed(),
        plan_length: result.as_ref().map_or(0, |p| p.len()),
        nodes_explored: get_nodes_explored(), // Custom metric
    };
    
    Ok((result, metrics))
}
```

This API reference provides the technical depth needed for professional integration while maintaining clarity for complex planning scenarios.
