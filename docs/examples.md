# GTRusthop Examples and Tutorials

This comprehensive guide teaches Goal-Task-Network planning through progressive examples, designed with a pedagogical approach to build understanding from fundamental concepts to advanced applications.

**🔒 Thread-Safe Architecture**: All examples use GTRusthop's thread-safe builder pattern that eliminates race conditions and enables parallel planning.

## Learning Objectives

By the end of this tutorial, you will understand:
- **Planning Paradigms**: HTN (task-based) vs HGN (goal-based) planning approaches
- **Core Concepts**: States, actions, tasks, and goals in planning
- **Thread-Safe Builder Pattern**: How to create isolated planner instances
- **Pyhop Compatibility**: How to migrate from original Pyhop planner
- **Domain Design**: How to model real-world problems as planning domains
- **Planning vs Acting**: Difference between planning actions and execution commands
- **Lazy Lookahead**: How to combine planning and acting for robust execution
- **Planning Strategies**: When to use recursive vs iterative approaches
- **Error Handling**: How to debug and handle planning failures
- **Performance**: How to optimize planning for complex domains

## Table of Contents

1. [Fundamental Concepts](#1-fundamental-concepts)
2. [Thread-Safe Builder Pattern](#2-thread-safe-builder-pattern)
3. [Your First Planning Domain](#3-your-first-planning-domain)
4. [Understanding States and Goals](#3-understanding-states-and-goals)
5. [Planning vs Acting with Lazy Lookahead](#5-planning-vs-acting-with-lazy-lookahead)
6. [Hierarchical Goal Networks (HGN)](#6-hierarchical-goal-networks-hgn)
7. [Building Complex Behaviors](#7-building-complex-behaviors)
8. [Advanced Planning Techniques](#8-advanced-planning-techniques)
9. [Common Pitfalls and Solutions](#9-common-pitfalls-and-solutions)
10. [Practice Exercises](#10-practice-exercises)

---

## Running Individual Examples

Before diving into the tutorials, you may want to explore GTRusthop's built-in examples. These examples demonstrate different planning scenarios and can be executed individually using `cargo test` commands.

### Available Example Modules

GTRusthop includes five main example modules:

| Example Module | Planning Paradigm | Description | Key Concepts Demonstrated |
|----------------|-------------------|-------------|---------------------------|
| **`simple_htn_example`** | **HTN** | Basic HTN planning with travel scenarios and Pyhop compatibility | Task methods, `pyhop()` function, migration patterns |
| **`blocks_htn_example`** | **HTN** | Classic blocks world planning using Gupta-Nau algorithm | Complex HTN decomposition, blocks world domain |
| **`backtracking_htn_example`** | **HTN** | HTN planning with backtracking capabilities | Method ordering, failure propagation, backtracking |
| **`simple_hgn_example`** | **HGN** | Hierarchical Goal Network examples | Multigoal methods, goal-oriented planning |
| **`logistics_hgn_example`** | **HGN** | HGN planning for logistics domain problems | Complex HGN domains, multi-modal transportation |
| **`lazy_lookahead_example`** | **Both** | Planning and acting with command failures | Commands vs actions, replanning, robust execution |
| **`regression_tests`** | **Both** | Comprehensive regression test suite | Edge cases, error handling, robustness |

### Planning Paradigm Guide

**HTN (Hierarchical Task Network) Planning**:
- Uses **task methods exclusively** (`declare_task_method`)
- Decomposes abstract tasks into subtasks and primitive actions
- Procedural approach: "How to do something"
- Examples: `simple_htn_example`, `blocks_htn_example`

**HGN (Hierarchical Goal Network) Planning**:
- Uses **multigoal methods** (`declare_multigoal_method`)
- Decomposes goals into subgoals and actions
- Declarative approach: "What to achieve"
- Examples: `simple_hgn_example`, `logistics_hgn_example`

### New Examples Overview

#### `backtracking_htn_example` - HTN Backtracking

This example demonstrates how HTN planning handles backtracking when methods fail:

```rust
// Test backtracking behavior
cargo test backtracking_htn_example -- --nocapture
```

**Key Features**:
- Multiple task methods for the same task
- Method failure and automatic backtracking
- Complex backtracking scenarios with interdependent tasks
- Verbose output showing backtracking decisions

#### `logistics_hgn_example` - Complex HGN Domain

This example shows HGN planning for a logistics domain with trucks, planes, and packages:

```rust
// Test logistics planning
cargo test logistics_hgn_example -- --nocapture
```

**Key Features**:
- Multi-modal transportation (trucks for intra-city, planes for inter-city)
- Complex state representation with locations, cities, and vehicles
- Hierarchical goal decomposition for package transportation
- Demonstrates the power and complexity of HGN planning

### Execution Commands

**Windows (PowerShell/Command Prompt):**
```powershell
# Navigate to GTRusthop directory first
GTRusthop> cd path\to\GTRusthop

# Run individual example modules
GTRusthop> cargo test simple_htn_example
GTRusthop> cargo test blocks_htn_example
GTRusthop> cargo test simple_hgn_example
GTRusthop> cargo test lazy_lookahead_example
GTRusthop> cargo test regression_tests

# Run with detailed output to see planning steps
GTRusthop> cargo test simple_htn_example -- --nocapture
```

**Linux/macOS (bash/zsh):**
```bash
# Navigate to GTRusthop directory first
GTRusthop$ cd path/to/GTRusthop

# Run individual example modules
GTRusthop$ cargo test simple_htn_example
GTRusthop$ cargo test blocks_htn_example
GTRusthop$ cargo test simple_hgn_example
GTRusthop$ cargo test lazy_lookahead_example
GTRusthop$ cargo test regression_tests

# Run with detailed output to see planning steps
GTRusthop$ cargo test simple_htn_example -- --nocapture
```

### Why Use This Approach?

- **No setup required**: Examples run immediately without creating new projects
- **Isolated execution**: Each example runs independently with its own test environment
- **Detailed output**: Use `-- --nocapture` to see full planning traces and debug information
- **Educational value**: Examples include comprehensive comments and explanations

### Recommended Learning Path

1. **Start with `simple_htn_example`**: Learn basic HTN planning concepts and Pyhop compatibility
2. **Explore `blocks_htn_example`**: Understand complex HTN state management
3. **Study `backtracking_htn_example`**: Learn how HTN backtracking works with multiple methods
4. **Try `simple_hgn_example`**: Learn goal-oriented HGN planning
5. **Examine `logistics_hgn_example`**: Understand complex HGN domains with multi-modal transportation
6. **Practice `lazy_lookahead_example`**: Learn planning and acting integration
7. **Review `regression_tests`**: See edge cases and error handling

### Pyhop Compatibility

GTRusthop includes a `pyhop()` function for backward compatibility with the original Pyhop planner:

```rust
// Test the pyhop compatibility function
cargo test test_run_pyhop_simple_travel_example -- --nocapture
```

This demonstrates:
- **Legacy API**: How to use the `pyhop()` function
- **Migration Path**: Deprecation messages guiding toward modern API
- **Functional Equivalence**: Same results as original Pyhop planner

**Migration Example**:
```rust
// Old Pyhop style (shows deprecation message)
use gtrusthop::pyhop;
let plan = pyhop(domain, state, todo_list)?;

// Modern GTRusthop style (recommended)
use gtrusthop::PlannerBuilder;
let planner = PlannerBuilder::new().with_domain(domain).build()?;
let plan = planner.find_plan(state, todo_list)?;
```

Now let's dive into the fundamental concepts...

---

## 1. Fundamental Concepts

### What is HTN Planning?

Hierarchical Task Network (HTN) planning breaks down complex goals into simpler subtasks until primitive actions can be executed. Think of it like following a recipe:

- **Goal**: "Make dinner"
- **Tasks**: "Prepare ingredients", "Cook meal", "Set table"
- **Actions**: "Chop onions", "Heat pan", "Place plates"

### Why Use GTRusthop?

**Traditional Planning Problems**:
- State explosion in complex domains
- Difficulty expressing domain knowledge
- Poor performance on real-world problems

**HTN Planning Solutions**:
- Hierarchical decomposition reduces complexity
- Domain knowledge guides search
- Natural expression of human-like reasoning

### Core Components

```rust
use gtrusthop::{Domain, State, PlanItem, PlannerBuilder};
use gtrusthop::core::string_value;

// 1. Domain: Contains actions and methods
let mut domain = Domain::new("my_domain");

// 2. Planner: Thread-safe planner instance
let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_verbose_level(1)?
    .build()?;

// 3. State: Current world state
let mut state = State::new("initial_state");

// 4. Goals: What we want to achieve
let goals = vec![PlanItem::task("achieve_goal", vec![])];

// 5. Planning: Find action sequence
let plan = planner.find_plan(state, goals)?;
```

**Why This Structure?**
- **Thread Safety**: Each planner instance is completely isolated
- **No Race Conditions**: Multiple planners can run in parallel
- **Separation of Concerns**: Domain knowledge separate from problem instances
- **Reusability**: Same domain can solve multiple problems
- **Modularity**: Actions and methods can be developed independently

---

## 2. Thread-Safe Builder Pattern

### Why Builder Pattern?

GTRusthop uses a thread-safe builder pattern that eliminates race conditions and enables parallel planning:

**❌ Old Global State Approach (Deprecated)**:
```rust
// DEPRECATED - Don't use this!
use gtrusthop::{find_plan, planning::initialize_planning};

let domain = create_domain();
initialize_planning(domain); // Global state - race conditions!

let plan = find_plan(state, goals)?; // Uses global context
```

**✅ New Builder Pattern Approach**:
```rust
use gtrusthop::PlannerBuilder;

let domain = create_domain();
let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_verbose_level(1)?
    .build()?;

let plan = planner.find_plan(state, goals)?; // Isolated instance
```

### Builder Pattern Benefits

- **🔒 Thread-Safe**: No global state, no race conditions
- **🎯 Isolated**: Each planner instance is completely independent
- **🔧 Configurable**: Fluent API for easy configuration
- **⚡ Performance**: No synchronization overhead
- **🧪 Testable**: Perfect for parallel testing

### Configuration Options

```rust
use gtrusthop::{PlannerBuilder, PlanningStrategy};

let planner = PlannerBuilder::new()
    .with_domain(domain)                           // Required: domain knowledge
    .with_verbose_level(2)?                        // Optional: debug output (0-3)
    .with_strategy(PlanningStrategy::Iterative)    // Optional: planning algorithm
    .with_goal_verification(true)                  // Optional: verify goal achievement
    .build()?;                                     // Create isolated planner
```

### Parallel Planning Example

```rust
use std::thread;

let domain = create_domain();

// Spawn multiple planning threads - no race conditions!
let handles: Vec<_> = (0..4).map(|i| {
    let domain_clone = domain.clone();

    thread::spawn(move || {
        let planner = PlannerBuilder::new()
            .with_domain(domain_clone)
            .with_verbose_level(0)? // Avoid mixed output
            .build()?;

        let state = create_state_for_thread(i);
        let goals = create_goals_for_thread(i);

        planner.find_plan(state, goals)
    })
}).collect();

// All threads run in parallel without interference
for handle in handles {
    match handle.join().unwrap() {
        Ok(Some(plan)) => println!("Found plan: {:?}", plan),
        Ok(None) => println!("No plan found"),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## 3. Your First Planning Domain

Let's build a simple robot navigation domain step by step.

### Step 1: Define the Problem

**Scenario**: A robot needs to move between rooms in a house.

**Components**:
- **Objects**: Robot, rooms
- **Relations**: Robot location
- **Actions**: Move between adjacent rooms
- **Goals**: Robot should be in a specific room

### Step 2: Create the Domain

```rust
use gtrusthop::{Domain, State, PlanItem, PlannerBuilder};
use gtrusthop::core::string_value;

fn create_robot_domain() -> gtrusthop::Result<Domain> {
    let mut domain = Domain::new("robot_navigation");

    // Define the move action
    domain.declare_action("move", |state: &mut State, args: &[gtrusthop::core::StateValue]| {
        // Extract arguments
        if args.len() != 3 {
            return None; // Wrong number of arguments
        }

        let robot = args[0].as_str()?;
        let from_room = args[1].as_str()?;
        let to_room = args[2].as_str()?;

        // Check precondition: robot must be in from_room
        if state.get_var("location", robot) != Some(&string_value(from_room)) {
            return None; // Precondition not met
        }

        // Check if rooms are adjacent (simplified - assume all rooms adjacent)
        if from_room == to_room {
            return None; // Can't move to same room
        }

        // Apply effect: update robot location
        state.set_var("location", robot, string_value(to_room));

        Some(state.clone())
    })?;

    Ok(domain)
}
```

**Why This Design?**
- **Precondition Checking**: Ensures action only applies when valid
- **Effect Application**: Changes state to reflect action outcome
- **Error Handling**: Returns `None` for invalid applications

### Step 3: Create Initial State

```rust
fn create_initial_state() -> State {
    let mut state = State::new("house_state");
    
    // Robot starts in the kitchen
    state.set_var("location", "robot", string_value("kitchen"));
    
    // We could add more objects and relations here
    state.set_var("adjacent", "kitchen", vec![
        string_value("living_room"),
        string_value("dining_room")
    ].into());
    
    state
}
```

**Why Set Up State This Way?**
- **Clear Initial Conditions**: Explicitly define starting state
- **Extensible**: Easy to add more objects and relations
- **Debuggable**: Can inspect state at any point

### Step 4: Test Basic Planning

```rust
fn test_basic_navigation() -> gtrusthop::Result<()> {
    // Create domain and planner with builder pattern
    let domain = create_robot_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Create initial state
    let state = create_initial_state();

    // Define goal: robot should be in living room
    let goals = vec![
        PlanItem::action("move", vec![
            string_value("robot"),
            string_value("kitchen"),
            string_value("living_room")
        ])
    ];

    // Find plan using isolated planner instance
    match planner.find_plan(state, goals)? {
        Some(plan) => {
            println!("Found plan:");
            for (i, action) in plan.iter().enumerate() {
                println!("  {}: {}", i + 1, action);
            }
        }
        None => println!("No plan found!"),
    }

    Ok(())
}
```

**Running the Example:**

**Windows (PowerShell/Command Prompt):**
```powershell
# Create new project
cargo new robot_navigation
cd robot_navigation

# Add dependencies
cargo add gtrusthop
cargo add serde_json

# Copy the code above into src/main.rs
# Then run:
cargo run

# For debugging:
$env:RUST_LOG = "debug"
cargo run
```

**Linux/macOS (bash/zsh):**
```bash
# Create new project
cargo new robot_navigation
cd robot_navigation

# Add dependencies
cargo add gtrusthop
cargo add serde_json

# Copy the code above into src/main.rs
# Then run:
cargo run

# For debugging:
RUST_LOG=debug cargo run
```

**Expected Output**:
```
Found plan:
  1: (move "robot" "kitchen" "living_room")
```

**Why This Works**:
- Robot is initially in kitchen
- Goal requires robot in living_room
- Move action can achieve this directly

---

## 3. Understanding States and Goals

### State Representation Deep Dive

States in GTRusthop use a flexible key-value structure:

```rust
// State structure: variable_name -> object -> value
state.set_var("location", "robot", string_value("kitchen"));
state.set_var("battery", "robot", 85.0.into());
state.set_var("carrying", "robot", serde_json::Value::Null);

// Complex values are supported
state.set_var("inventory", "robot", vec![
    string_value("key"),
    string_value("map")
].into());
```

**Why This Structure?**
- **Flexibility**: Can represent any domain
- **Efficiency**: O(1) lookup for variable access
- **Type Safety**: JSON values provide runtime type checking

### Goal Types and When to Use Them

#### 1. Direct Actions (Immediate Execution)

```rust
let direct_goal = PlanItem::action("move", vec![
    string_value("robot"),
    string_value("kitchen"),
    string_value("bedroom")
]);
```

**When to Use**: When you know exactly what action to perform.

#### 2. Tasks (Hierarchical Decomposition)

```rust
let task_goal = PlanItem::task("go_to", vec![
    string_value("robot"),
    string_value("bedroom")
]);
```

**When to Use**: When the solution method may vary based on current state.

#### 3. Unigoals (State Conditions)

```rust
let state_goal = PlanItem::unigoal("location", "robot", string_value("bedroom"));
```

**When to Use**: When you care about the end state, not how to achieve it.

### Building Task Methods

Task methods provide the "how" for achieving tasks:

```rust
domain.declare_task_method("go_to", |state: &State, args: &[StateValue]| {
    if args.len() != 2 {
        return None;
    }
    
    let robot = args[0].as_str()?;
    let target_room = args[1].as_str()?;
    
    // Check current location
    let current_location = state.get_var("location", robot)?.as_str()?;
    
    // If already there, no action needed
    if current_location == target_room {
        return Some(vec![]); // Empty plan
    }
    
    // Simple case: direct movement
    Some(vec![
        PlanItem::action("move", vec![
            string_value(robot),
            string_value(current_location),
            string_value(target_room)
        ])
    ])
})?;
```

**Why Methods Matter**:
- **Adaptability**: Same task can be solved differently based on state
- **Reusability**: One method handles many similar situations
- **Abstraction**: Hide implementation details from higher-level planning

### Advanced State Checking

```rust
// Check multiple conditions
fn robot_can_move(state: &State, robot: &str, from: &str, to: &str) -> bool {
    // Check robot location
    if state.get_var("location", robot) != Some(&string_value(from)) {
        return false;
    }
    
    // Check battery level
    if let Some(battery) = state.get_var("battery", robot) {
        if battery.as_f64().unwrap_or(0.0) < 10.0 {
            return false; // Low battery
        }
    }
    
    // Check if path is blocked
    if let Some(blocked_paths) = state.get_var("blocked", "paths") {
        if let Some(blocked_array) = blocked_paths.as_array() {
            let path = format!("{}->{}", from, to);
            if blocked_array.iter().any(|p| p.as_str() == Some(&path)) {
                return false;
            }
        }
    }
    
    true
}
```

**Why Complex Checking?**
- **Realism**: Real domains have multiple constraints
- **Robustness**: Prevents invalid plans
- **Modularity**: Separate checking logic for reuse

---

## 5. Planning vs Acting with Lazy Lookahead

### The Planning vs Acting Problem

Traditional planning assumes a perfect world where:
- All actions succeed as planned
- The environment doesn't change
- Execution matches the plan exactly

**Reality is different**:
- Commands can fail during execution
- The environment changes dynamically
- Plans need to adapt to unexpected situations

### Enter Lazy Lookahead

The lazy lookahead algorithm from Ghallab et al. (2016) bridges this gap by combining planning and acting:

1. **Plan**: Generate a plan for current state and goals
2. **Execute**: Try to execute each action as a command
3. **Replan**: If a command fails, generate a new plan
4. **Repeat**: Continue until goals achieved or max attempts reached

### Commands vs Actions

**Actions** (for planning):
```rust
domain.declare_action("call_taxi", |state, args| {
    // Planning logic - assume taxi always available
    Some(state.clone())
})?;
```

**Commands** (for execution):
```rust
domain.declare_command("c_call_taxi", |state, args| {
    // Execution logic - might fail in real world
    if taxi_available() {
        Some(state.clone())
    } else {
        None // Command failed - triggers replanning
    }
})?;
```

### Lazy Lookahead Example

```rust
use gtrusthop::{PlannerBuilder, Domain, State, PlanItem, core::string_value};

fn create_taxi_domain() -> gtrusthop::Result<Domain> {
    let mut domain = Domain::new("taxi_domain");

    // Action for planning (optimistic)
    domain.declare_action("call_taxi", |state, args| {
        // Assume taxi always comes during planning
        Some(state.clone())
    })?;

    // Command for execution (realistic)
    domain.declare_command("c_call_taxi", |state, args| {
        // Real execution - taxi might not be available
        if taxi_service_available() {
            Some(state.clone())
        } else {
            None // Failed - will trigger replanning
        }
    })?;

    // Alternative action for walking
    domain.declare_action("walk", |state, args| {
        if args.len() >= 3 {
            let person = args[0].as_str()?;
            let from = args[1].as_str()?;
            let to = args[2].as_str()?;

            state.set_var("location", person, string_value(to));
            Some(state.clone())
        } else {
            None
        }
    })?;

    // Corresponding command (walking usually works)
    domain.declare_command("c_walk", |state, args| {
        if args.len() >= 3 {
            let person = args[0].as_str()?;
            let from = args[1].as_str()?;
            let to = args[2].as_str()?;

            state.set_var("location", person, string_value(to));
            Some(state.clone())
        } else {
            None
        }
    })?;

    // Task method with multiple strategies
    domain.declare_task_method("travel", |state, args| {
        if args.len() >= 3 {
            let person = args[0].as_str()?;
            let from = args[1].as_str()?;
            let to = args[2].as_str()?;

            // Check if person has money for taxi
            if let Some(money) = state.get_var("money", person) {
                if money.as_f64().unwrap_or(0.0) >= 10.0 {
                    // Try taxi first
                    return Some(vec![
                        PlanItem::action("call_taxi", vec![string_value(person)]),
                        PlanItem::action("ride_taxi", vec![
                            string_value(person),
                            string_value(from),
                            string_value(to)
                        ])
                    ]);
                }
            }

            // Fall back to walking
            Some(vec![
                PlanItem::action("walk", vec![
                    string_value(person),
                    string_value(from),
                    string_value(to)
                ])
            ])
        } else {
            None
        }
    })?;

    Ok(domain)
}

fn test_lazy_lookahead() -> gtrusthop::Result<()> {
    let domain = create_taxi_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Create initial state
    let mut state = State::new("initial");
    state.set_var("location", "alice", string_value("home"));
    state.set_var("money", "alice", 15.0.into());

    // Goal: get to work
    let goals = vec![
        PlanItem::task("travel", vec![
            string_value("alice"),
            string_value("home"),
            string_value("work")
        ])
    ];

    // Run lazy lookahead - handles command failures automatically
    let final_state = planner.run_lazy_lookahead(state, goals, 5)?;

    println!("Final location: {:?}",
             final_state.get_var("location", "alice"));

    Ok(())
}
```

### Benefits of Lazy Lookahead

- **Robust Execution**: Handles real-world command failures gracefully
- **Adaptive Planning**: Automatically adjusts plans based on execution results
- **Real-World Ready**: Bridges the gap between planning and acting
- **Debugging Support**: Verbose output helps understand execution flow

### When to Use Lazy Lookahead

**Use lazy lookahead when**:
- Commands can fail during execution
- Environment changes during plan execution
- You need robust, adaptive behavior
- Planning and acting happen in the same system

**Use regular planning when**:
- Actions are deterministic and reliable
- Plans are executed by external systems
- You only need to generate plans, not execute them
- Performance is critical and failures are rare

---

## 6. Hierarchical Goal Networks (HGN)

### Understanding Goal-Oriented Planning

While HTN planning focuses on **how** to achieve goals through task decomposition, HGN planning focuses on **what** goals to achieve and how they relate to each other. HGN is particularly powerful for:

- **Goal-oriented domains**: Where the focus is on achieving specific states
- **Logistics and transportation**: Moving objects to desired locations
- **Resource allocation**: Distributing resources to meet requirements
- **Configuration problems**: Setting up systems to meet specifications

### HGN vs HTN: Key Differences

| Aspect | HTN (Task-Oriented) | HGN (Goal-Oriented) |
|--------|---------------------|---------------------|
| **Primary Focus** | How to do tasks | What goals to achieve |
| **Decomposition** | Tasks → Subtasks → Actions | Goals → Subgoals → Actions |
| **Planning Style** | Procedural | Declarative |
| **Best For** | Process-oriented domains | State-oriented domains |

### Logistics HGN Example

Let's build a logistics domain using HGN planning:

```rust
use gtrusthop::{Domain, State, PlanItem, PlannerBuilder, string_value};

fn create_logistics_hgn_domain() -> gtrusthop::Result<Domain> {
    let mut domain = Domain::new("logistics_hgn");

    // Actions for moving objects
    domain.declare_action("drive_truck", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(truck), Some(location)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("truck_at", truck, string_value(location));
                return Some(state.clone());
            }
        }
        None
    })?;

    domain.declare_action("load_truck", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(obj), Some(truck)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("at", obj, string_value(truck));
                return Some(state.clone());
            }
        }
        None
    })?;

    // Goal methods for achieving "at" goals
    domain.declare_unigoal_method("at", |state: &State, obj: &str, target_value: &StateValue| {
        if let Some(target_location) = target_value.as_str() {
            // Check if already at target
            if let Some(current_location) = state.get_var("at", obj) {
                if current_location.as_str() == Some(target_location) {
                    return Some(vec![]); // Already satisfied
                }

                // Find a truck in the same city and create subgoals
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

    Ok(domain)
}
```

### Running the Logistics HGN Example

```rust
fn test_logistics_hgn() -> gtrusthop::Result<()> {
    let domain = create_logistics_hgn_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Create initial state
    let mut state = State::new("logistics_initial");
    state.set_var("at", "package1", string_value("location1"));
    state.set_var("truck_at", "truck1", string_value("location3"));
    state.set_var("in_city", "location1", string_value("city1"));
    state.set_var("in_city", "location2", string_value("city1"));
    state.set_var("in_city", "location3", string_value("city1"));

    // Goal: Move package1 to location2
    let goals = vec![
        PlanItem::unigoal("at", "package1", string_value("location2"))
    ];

    let plan = planner.find_plan(state, goals)?;

    match plan {
        Some(actions) => {
            println!("Found logistics plan with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("No plan found"),
    }

    Ok(())
}
```

### Blocks World HGN Example

The blocks world is a classic HGN domain where goals specify desired block configurations:

```rust
fn create_blocks_hgn_domain() -> gtrusthop::Result<Domain> {
    let mut domain = Domain::new("blocks_hgn");

    // Standard blocks world actions
    domain.declare_action("pickup", |state: &mut State, args: &[StateValue]| {
        if let Some(block) = args[0].as_str() {
            // Check preconditions and apply effects
            if can_pickup(state, block) {
                state.set_var("pos", block, string_value("hand"));
                state.set_var("clear", block, false.into());
                state.set_var("holding", "hand", string_value(block));
                return Some(state.clone());
            }
        }
        None
    })?;

    // Goal methods for "pos" goals
    domain.declare_unigoal_method("pos", |state: &State, block: &str, target_value: &StateValue| {
        if let Some(target) = target_value.as_str() {
            // Check if already satisfied
            if let Some(current_pos) = state.get_var("pos", block) {
                if current_pos.as_str() == Some(target) {
                    return Some(vec![]); // Already satisfied
                }

                // If target is "hand", pick up the block
                if target == "hand" {
                    if can_pickup(state, block) {
                        if current_pos.as_str() == Some("table") {
                            return Some(vec![
                                PlanItem::action("pickup", vec![string_value(block)])
                            ]);
                        } else if let Some(under_block) = current_pos.as_str() {
                            return Some(vec![
                                PlanItem::action("unstack", vec![string_value(block), string_value(under_block)])
                            ]);
                        }
                    }
                }
                // If currently holding the block, put it down
                else if current_pos.as_str() == Some("hand") {
                    if target == "table" {
                        return Some(vec![
                            PlanItem::action("putdown", vec![string_value(block)])
                        ]);
                    } else {
                        return Some(vec![
                            PlanItem::action("stack", vec![string_value(block), string_value(target)])
                        ]);
                    }
                }
            }
        }
        None
    })?;

    // Multigoal method for complex block configurations
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

    Ok(domain)
}
```

### Solving the Sussman Anomaly with HGN

The Sussman Anomaly is a classic planning problem that demonstrates the power of goal-oriented planning:

```rust
fn solve_sussman_anomaly() -> gtrusthop::Result<()> {
    let domain = create_blocks_hgn_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Initial state: C on A, A and B on table
    let mut state = State::new("sussman_initial");
    state.set_var("pos", "a", string_value("table"));
    state.set_var("pos", "b", string_value("table"));
    state.set_var("pos", "c", string_value("a"));
    state.set_var("clear", "a", false.into());
    state.set_var("clear", "b", true.into());
    state.set_var("clear", "c", true.into());
    state.set_var("holding", "hand", false.into());

    // Goal: A on B, B on C
    let mut goal = Multigoal::new("sussman_goal");
    goal.set_goal("pos", "a", string_value("b"));
    goal.set_goal("pos", "b", string_value("c"));

    let plan = planner.find_plan(state, vec![PlanItem::Multigoal(goal)])?;

    match plan {
        Some(actions) => {
            println!("Sussman Anomaly solution with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("No solution found for Sussman Anomaly"),
    }

    Ok(())
}
```

### HGN Best Practices

#### 1. Goal Method Design
- **Check satisfaction first**: Always check if the goal is already satisfied
- **Decompose systematically**: Break complex goals into simpler subgoals
- **Handle edge cases**: Consider all possible states and transitions

#### 2. Multigoal Planning
- **Analyze dependencies**: Understand which goals depend on others
- **Plan incrementally**: Achieve goals in the right order
- **Avoid conflicts**: Ensure achieving one goal doesn't undo another

#### 3. State Management
- **Clear state representation**: Use meaningful variable names
- **Consistent updates**: Ensure actions update state correctly
- **Validation**: Check preconditions before applying effects

### When to Use HGN vs HTN

**Use HGN when**:
- Goals are the primary concern
- State configuration is important
- Multiple ways to achieve the same goal exist
- Planning is more declarative than procedural

**Use HTN when**:
- Procedures and processes are well-defined
- Task decomposition is natural
- Domain expertise is procedural
- Planning is more about "how" than "what"

---

## 7. Building Complex Behaviors

### Multi-Step Task Decomposition

Let's extend our robot domain to handle complex tasks:

```rust
// Task: Clean the house
domain.declare_task_method("clean_house", |state: &State, args: &[StateValue]| {
    let robot = args[0].as_str()?;
    
    Some(vec![
        PlanItem::task("go_to", vec![string_value(robot), string_value("storage")]),
        PlanItem::task("get_supplies", vec![string_value(robot)]),
        PlanItem::task("clean_room", vec![string_value(robot), string_value("living_room")]),
        PlanItem::task("clean_room", vec![string_value(robot), string_value("kitchen")]),
        PlanItem::task("clean_room", vec![string_value(robot), string_value("bedroom")]),
        PlanItem::task("return_supplies", vec![string_value(robot)]),
    ])
})?;

// Subtask: Clean a specific room
domain.declare_task_method("clean_room", |state: &State, args: &[StateValue]| {
    let robot = args[0].as_str()?;
    let room = args[1].as_str()?;
    
    // Check if room is already clean
    if state.get_var("cleanliness", room) == Some(&string_value("clean")) {
        return Some(vec![]); // Already clean
    }
    
    Some(vec![
        PlanItem::task("go_to", vec![string_value(robot), string_value(room)]),
        PlanItem::action("vacuum", vec![string_value(robot), string_value(room)]),
        PlanItem::action("dust", vec![string_value(robot), string_value(room)]),
    ])
})?;
```

**Why Hierarchical Decomposition?**
- **Manageable Complexity**: Break large tasks into smaller pieces
- **Reusable Components**: `clean_room` can be used in different contexts
- **Natural Planning**: Mirrors human problem-solving approach

### Conditional Planning

Sometimes the plan depends on the current state:

```rust
domain.declare_task_method("get_supplies", |state: &State, args: &[StateValue]| {
    let robot = args[0].as_str()?;
    
    // Check what supplies are needed
    let mut tasks = vec![];
    
    // Always need to go to storage first
    tasks.push(PlanItem::task("go_to", vec![
        string_value(robot), 
        string_value("storage")
    ]));
    
    // Check if vacuum is available
    if state.get_var("available", "vacuum") == Some(&true.into()) {
        tasks.push(PlanItem::action("pick_up", vec![
            string_value(robot),
            string_value("vacuum")
        ]));
    }
    
    // Check if cleaning supplies are available
    if state.get_var("available", "cleaning_supplies") == Some(&true.into()) {
        tasks.push(PlanItem::action("pick_up", vec![
            string_value(robot),
            string_value("cleaning_supplies")
        ]));
    }
    
    Some(tasks)
})?;
```

**Why Conditional Planning?**
- **Adaptability**: Plans adjust to current circumstances
- **Efficiency**: Don't plan for unavailable resources
- **Robustness**: Handle varying initial conditions

### Error Recovery and Alternative Methods

Multiple methods for the same task provide robustness:

```rust
// Primary method: Use elevator
domain.declare_task_method("go_to_floor", |state: &State, args: &[StateValue]| {
    let robot = args[0].as_str()?;
    let target_floor = args[1].as_str()?;
    
    // Check if elevator is working
    if state.get_var("status", "elevator") == Some(&string_value("working")) {
        return Some(vec![
            PlanItem::task("go_to", vec![string_value(robot), string_value("elevator")]),
            PlanItem::action("use_elevator", vec![
                string_value(robot),
                string_value(target_floor)
            ]),
        ]);
    }
    
    None // This method can't be used
})?;

// Backup method: Use stairs
domain.declare_task_method("go_to_floor", |state: &State, args: &[StateValue]| {
    let robot = args[0].as_str()?;
    let target_floor = args[1].as_str()?;
    
    // Check if robot can use stairs
    if state.get_var("capability", robot).and_then(|c| c.get("stairs")) == Some(&true.into()) {
        return Some(vec![
            PlanItem::task("go_to", vec![string_value(robot), string_value("stairs")]),
            PlanItem::action("climb_stairs", vec![
                string_value(robot),
                string_value(target_floor)
            ]),
        ]);
    }
    
    None
})?;
```

**Why Multiple Methods?**
- **Fault Tolerance**: System continues working when components fail
- **Optimization**: Choose best method based on current conditions
- **Flexibility**: Handle different scenarios with same interface

---

## 8. Advanced Planning Techniques

### Goal Methods for State-Based Planning

Goal methods specify how to achieve desired states:

```rust
// Goal method: Achieve robot location
domain.declare_unigoal_method("location", |state: &State, arg: &str, value: &StateValue| {
    let robot = arg;
    let target_location = value.as_str()?;
    
    // Check if already at target
    if state.get_var("location", robot) == Some(value) {
        return Some(vec![]); // Goal already achieved
    }
    
    // Get current location
    let current_location = state.get_var("location", robot)?.as_str()?;
    
    // Plan path (simplified - direct movement)
    Some(vec![
        PlanItem::action("move", vec![
            string_value(robot),
            string_value(current_location),
            string_value(target_location)
        ])
    ])
})?;
```

**When to Use Goal Methods**:
- **State-Focused Planning**: When you care about end state, not process
- **Multiple Achievement Paths**: Different ways to reach same state
- **Declarative Goals**: Express what you want, not how to get it

### Performance Optimization Strategies

#### 1. Early Termination

```rust
domain.declare_task_method("transport_all", |state: &State, args: &[StateValue]| {
    let robot = args[0].as_str()?;
    
    // Quick check: are all items already transported?
    let items = ["item1", "item2", "item3"];
    let target_location = "destination";
    
    let all_transported = items.iter().all(|item| {
        state.get_var("location", item) == Some(&string_value(target_location))
    });
    
    if all_transported {
        return Some(vec![]); // Nothing to do
    }
    
    // Generate transport tasks for items not at destination
    let mut tasks = vec![];
    for item in items {
        if state.get_var("location", item) != Some(&string_value(target_location)) {
            tasks.push(PlanItem::task("transport", vec![
                string_value(robot),
                string_value(item),
                string_value(target_location)
            ]));
        }
    }
    
    Some(tasks)
})?;
```

#### 2. State Caching and Reuse

```rust
use std::collections::HashMap;

// Cache expensive computations
fn get_shortest_path(
    state: &State, 
    from: &str, 
    to: &str,
    cache: &mut HashMap<(String, String), Vec<String>>
) -> Option<Vec<String>> {
    let key = (from.to_string(), to.to_string());
    
    if let Some(cached_path) = cache.get(&key) {
        return Some(cached_path.clone());
    }
    
    // Compute path (expensive operation)
    let path = compute_shortest_path(state, from, to)?;
    cache.insert(key, path.clone());
    
    Some(path)
}
```

### Debugging Complex Plans

#### 1. Verbose Output

```rust
use gtrusthop::PlannerBuilder;

// Create planner with detailed debugging
let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_verbose_level(3)?  // Enable detailed debugging
    .build()?;

// Run planning with full trace
let plan = planner.find_plan(state, goals)?;
```

**Environment-Based Debugging:**

**Windows (PowerShell/Command Prompt):**
```powershell
# Set verbose logging
$env:RUST_LOG = "gtrusthop=debug"
cargo run

# Or set GTRusthop-specific verbosity
$env:GTRUSTHOP_VERBOSE = "3"
cargo run

# Capture output to file
cargo run > planning_output.txt 2>&1
```

**Linux/macOS (bash/zsh):**
```bash
# Set verbose logging
RUST_LOG=gtrusthop=debug cargo run

# Or set GTRusthop-specific verbosity
GTRUSTHOP_VERBOSE=3 cargo run

# Capture output to file
cargo run > planning_output.txt 2>&1

# Use tee to see output and save to file
cargo run 2>&1 | tee planning_output.txt
```

#### 2. Plan Validation

```rust
fn validate_plan(initial_state: &State, plan: &[PlanItem], domain: &Domain) -> gtrusthop::Result<bool> {
    let mut current_state = initial_state.copy(Some("validation"));
    
    for (step, action) in plan.iter().enumerate() {
        if let PlanItem::Action(action_name, args) = action {
            if let Some(action_fn) = domain.get_action(action_name) {
                match action_fn(&mut current_state, args) {
                    Some(new_state) => {
                        current_state = new_state;
                        println!("Step {}: {} succeeded", step + 1, action);
                    }
                    None => {
                        println!("Step {}: {} failed!", step + 1, action);
                        return Ok(false);
                    }
                }
            } else {
                println!("Unknown action: {}", action_name);
                return Ok(false);
            }
        }
    }
    
    Ok(true)
}
```

#### 3. State Inspection

```rust
fn debug_state(state: &State, label: &str) {
    println!("=== State: {} ===", label);
    state.display(None);
    println!("==================");
}

// Use in methods
domain.declare_task_method("debug_task", |state: &State, args: &[StateValue]| {
    debug_state(state, "Before task execution");
    
    // ... task logic ...
    
    Some(subtasks)
})?;
```

---

## 9. Common Pitfalls and Solutions

### Pitfall 1: Infinite Recursion

**Problem**: Task methods that call themselves without termination condition.

```rust
// BAD: Can cause infinite recursion
domain.declare_task_method("bad_search", |state: &State, args: &[StateValue]| {
    Some(vec![
        PlanItem::task("bad_search", args.to_vec()), // Calls itself!
    ])
})?;
```

**Solution**: Always include termination conditions.

```rust
// GOOD: Proper termination
domain.declare_task_method("good_search", |state: &State, args: &[StateValue]| {
    let target = args[0].as_str()?;
    
    // Termination condition
    if state.get_var("found", target) == Some(&true.into()) {
        return Some(vec![]); // Goal achieved
    }
    
    // Progress toward goal
    Some(vec![
        PlanItem::action("search_step", args.to_vec()),
        PlanItem::task("good_search", args.to_vec()), // Recursive call with progress
    ])
})?;
```

### Pitfall 2: Inconsistent State Updates

**Problem**: Actions that don't properly update state.

```rust
// BAD: Doesn't update state consistently
domain.declare_action("bad_move", |state: &mut State, args: &[StateValue]| {
    let robot = args[0].as_str()?;
    let to_room = args[2].as_str()?;
    
    // Only sets new location, doesn't clear old one
    state.set_var("location", robot, string_value(to_room));
    
    Some(state.clone())
})?;
```

**Solution**: Ensure all state changes are consistent.

```rust
// GOOD: Consistent state updates
domain.declare_action("good_move", |state: &mut State, args: &[StateValue]| {
    let robot = args[0].as_str()?;
    let from_room = args[1].as_str()?;
    let to_room = args[2].as_str()?;
    
    // Verify preconditions
    if state.get_var("location", robot) != Some(&string_value(from_room)) {
        return None;
    }
    
    // Update location
    state.set_var("location", robot, string_value(to_room));
    
    // Update any derived state (e.g., room occupancy)
    state.set_var("occupied", from_room, false.into());
    state.set_var("occupied", to_room, true.into());
    
    Some(state.clone())
})?;
```

### Pitfall 3: Overly Complex Methods

**Problem**: Methods that try to do too much.

```rust
// BAD: Overly complex method
domain.declare_task_method("complex_task", |state: &State, args: &[StateValue]| {
    // 50+ lines of complex logic
    // Multiple responsibilities
    // Hard to debug and maintain
    // ...
})?;
```

**Solution**: Break complex methods into smaller, focused methods.

```rust
// GOOD: Decomposed into smaller methods
domain.declare_task_method("main_task", |state: &State, args: &[StateValue]| {
    Some(vec![
        PlanItem::task("subtask_1", args.to_vec()),
        PlanItem::task("subtask_2", args.to_vec()),
        PlanItem::task("subtask_3", args.to_vec()),
    ])
})?;

domain.declare_task_method("subtask_1", |state: &State, args: &[StateValue]| {
    // Focused, single responsibility
    // Easy to test and debug
    // ...
})?;
```

### Pitfall 4: Poor Error Handling

**Problem**: Methods that fail silently or with unclear errors.

```rust
// BAD: Silent failures
domain.declare_task_method("bad_method", |state: &State, args: &[StateValue]| {
    let result = some_complex_computation(state, args);
    if result.is_ok() {
        Some(vec![/* ... */])
    } else {
        None // No indication of what went wrong
    }
})?;
```

**Solution**: Provide clear error information.

```rust
// GOOD: Clear error handling
domain.declare_task_method("good_method", |state: &State, args: &[StateValue]| {
    // Validate inputs
    if args.len() != 2 {
        eprintln!("Error: good_method requires exactly 2 arguments, got {}", args.len());
        return None;
    }
    
    let robot = match args[0].as_str() {
        Some(r) => r,
        None => {
            eprintln!("Error: first argument must be a string (robot name)");
            return None;
        }
    };
    
    // Check preconditions with clear messages
    if state.get_var("battery", robot).and_then(|b| b.as_f64()).unwrap_or(0.0) < 10.0 {
        eprintln!("Error: robot {} has insufficient battery", robot);
        return None;
    }
    
    Some(vec![/* ... */])
})?;
```

---

## 10. Practice Exercises

### Exercise 1: Basic Robot Tasks (Beginner)

**Objective**: Extend the robot domain with basic manipulation capabilities.

**Requirements**:
1. Add actions for `pick_up` and `put_down` objects
2. Add a task method for `transport_object`
3. Handle the case where robot is already carrying something

**Starter Code**:
```rust
use gtrusthop::PlannerBuilder;

fn exercise_1() -> gtrusthop::Result<()> {
    let mut domain = Domain::new("robot_manipulation");

    // TODO: Add pick_up action
    // TODO: Add put_down action
    // TODO: Add transport_object task method

    // Create planner with domain
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Test your implementation
    let mut state = State::new("test_state");
    state.set_var("location", "robot", string_value("room1"));
    state.set_var("location", "box", string_value("room1"));
    state.set_var("carrying", "robot", serde_json::Value::Null);

    let goals = vec![
        PlanItem::task("transport_object", vec![
            string_value("robot"),
            string_value("box"),
            string_value("room2")
        ])
    ];

    // Should produce plan: pick_up, move, put_down
    let plan = planner.find_plan(state, goals)?;
    println!("Plan: {:?}", plan);

    Ok(())
}
```

**Setting Up the Exercise:**

**Windows (PowerShell/Command Prompt):**
```powershell
# Create exercise project
cargo new --bin exercise_1
cd exercise_1

# Add dependencies
cargo add gtrusthop
cargo add serde_json

# Copy starter code to src\main.rs
# Edit the file with your favorite editor:
notepad src\main.rs
# Or use VS Code:
code src\main.rs

# Test your solution
cargo run
```

**Linux/macOS (bash/zsh):**
```bash
# Create exercise project
cargo new --bin exercise_1
cd exercise_1

# Add dependencies
cargo add gtrusthop
cargo add serde_json

# Copy starter code to src/main.rs
# Edit the file with your favorite editor:
nano src/main.rs
# Or use VS Code:
code src/main.rs

# Test your solution
cargo run
```

**Expected Learning**: Understanding action preconditions and effects.

### Exercise 2: Multi-Robot Coordination (Intermediate)

**Objective**: Handle multiple robots working together.

**Requirements**:
1. Prevent robots from occupying the same location
2. Add a task for coordinated object transport
3. Handle resource conflicts (e.g., both robots want the same object)

**Starter Code**:
```rust
use gtrusthop::PlannerBuilder;

fn exercise_2() -> gtrusthop::Result<()> {
    let mut domain = Domain::new("multi_robot");

    // TODO: Modify move action to check for conflicts
    // TODO: Add coordinated_transport task
    // TODO: Add resource allocation logic

    // Create planner with domain
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    let mut state = State::new("multi_robot_state");
    state.set_var("location", "robot1", string_value("room1"));
    state.set_var("location", "robot2", string_value("room2"));
    state.set_var("location", "heavy_box", string_value("room1"));

    let goals = vec![
        PlanItem::task("coordinated_transport", vec![
            string_value("robot1"),
            string_value("robot2"),
            string_value("heavy_box"),
            string_value("room3")
        ])
    ];

    let plan = planner.find_plan(state, goals)?;
    println!("Coordinated plan: {:?}", plan);

    Ok(())
}
```

**Expected Learning**: Handling interactions between multiple agents.

### Exercise 3: Dynamic Replanning (Advanced)

**Objective**: Handle plan failures and dynamic replanning.

**Requirements**:
1. Implement a monitoring system that detects plan failures
2. Add replanning capability when actions fail
3. Handle dynamic changes to the environment

**Starter Code**:
```rust
fn exercise_3() -> gtrusthop::Result<()> {
    // TODO: Implement ExecutionMonitor
    // TODO: Add failure detection
    // TODO: Implement replanning strategy
    
    struct ExecutionMonitor {
        domain: Domain,
        current_state: State,
        remaining_plan: Vec<PlanItem>,
    }
    
    impl ExecutionMonitor {
        fn execute_step(&mut self) -> gtrusthop::Result<bool> {
            // TODO: Execute next action
            // TODO: Check for failures
            // TODO: Trigger replanning if needed
            todo!()
        }
    }
    
    Ok(())
}
```

**Expected Learning**: Real-world planning considerations and robustness.

### Challenge: Complete Logistics Domain

**Objective**: Build a complete logistics planning domain.

**Requirements**:
- Multiple vehicles with different capabilities
- Package routing and scheduling
- Resource constraints (fuel, capacity, time)
- Multi-modal transportation (truck, plane, ship)

**Success Criteria**:
- Handle 10+ packages across 5+ locations
- Optimize for time and cost
- Handle vehicle breakdowns gracefully

This challenge integrates all concepts learned and provides experience with real-world complexity.

---

## Next Steps

After completing these tutorials:

1. **Study the Source Code**: Examine the built-in domains in `src/domains/`
2. **Read the API Documentation**: Deep dive into advanced features
3. **Build Your Own Domain**: Apply GTRusthop to your specific problem

Remember: HTN planning is as much art as science. The key is understanding your domain deeply and expressing that knowledge clearly in your methods and actions.
