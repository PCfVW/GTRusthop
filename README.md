# GTRusthop

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

GTRusthop is a Goal-Task-Network (GTN) planning library written in Rust, ported from the Python [pip branch of GTPyhop](https://github.com/PCfVW/GTPyhop/tree/pip), itself a fork of [Dana Nau's GTPyhop main branch](https://github.com/dananau/GTPyhop).

## Table of Contents

- [Overview](#overview)
  - [Key Features](#key-features)
  - [Planning Paradigms](#planning-paradigms)
    - [HTN (Hierarchical Task Network) Planning](#htn-hierarchical-task-network-planning)
    - [HGN (Hierarchical Goal Network) Planning](#hgn-hierarchical-goal-network-planning)
- [Let's Begin with the Examples](#lets-begin-with-the-examples)
  - [Running Individual Examples](#running-individual-examples)
- [Quick Start](#quick-start)
  - [Installation](#installation)
  - [Basic Usage](#basic-usage)
    - [Modern Builder Pattern API (Recommended)](#modern-builder-pattern-api-recommended)
    - [Builder Pattern Benefits](#builder-pattern-benefits)
    - [Multigoal Planning with Builder Pattern](#multigoal-planning-with-builder-pattern)
    - [Pyhop Compatibility API (Legacy)](#pyhop-compatibility-api-legacy)
  - [Planning Paradigms in Practice](#planning-paradigms-in-practice)
  - [Lazy Lookahead Algorithm](#lazy-lookahead-algorithm)
    - [Key Features](#key-features-1)
    - [Example Usage](#example-usage)
- [Documentation](#documentation)
  - [For Developers](#for-developers)
  - [For Migration and Setup](#for-migration-and-setup)
- [Architecture](#architecture)
- [Planning Strategies](#planning-strategies)
- [Verbose Output](#verbose-output)
- [Development Workflow](#development-workflow)
- [Comparison with GTPyhop](#comparison-with-gtpyhop)
- [Performance Benchmarks](#performance-benchmarks)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Citation](#citation)
- [Related Projects](#related-projects)

## Overview

GTRusthop provides Hierarchical Task Network (HTN) planning capabilities with support for both goals and tasks. It features both recursive and iterative planning strategies, comprehensive goal verification, and a flexible domain definition system, thanks to [Dana Nau](https://en.wikipedia.org/wiki/Dana_S._Nau)'s work.

### Key Features

- **Dual Planning Paradigms**: Support for both HTN (task-based) and HGN (goal-based) planning
- **Pyhop Compatibility**: Backward compatibility with original Pyhop planner via `pyhop()` function
- **Multiple Planning Strategies**: Choose between recursive and iterative algorithms (see [Performance Benchmarks](docs/runtime_benchmark.md))
- **Goal Verification**: Automatic verification that methods achieve their intended goals
- **Flexible Domain Definition**: Easy-to-use API for defining actions, task methods, and goal methods
- **Type Safety**: Rust's type system ensures memory safety and prevents common planning errors
- **Performance**: Compiled Rust code provides excellent performance for large planning problems
- **Comprehensive Testing**: Extensive test suite ensures reliability and correctness

### Planning Paradigms

GTRusthop supports two distinct planning approaches:

#### HTN (Hierarchical Task Network) Planning
- **Uses**: Task methods exclusively (`declare_task_method`)
- **Approach**: Decomposes abstract tasks into subtasks and primitive actions
- **Examples**: `simple_htn_example`, `blocks_htn_example`
- **Best for**: Procedural knowledge, step-by-step task decomposition

#### HGN (Hierarchical Goal Network) Planning
- **Uses**: Multigoal methods (`declare_multigoal_method`)
- **Approach**: Decomposes goals into subgoals and actions
- **Examples**: `simple_hgn_example`
- **Best for**: Declarative goals, state-based planning


## Let's Begin with the Examples

To run the included examples, first clone the repository and navigate to the project directory:

```bash
git clone https://github.com/PCfVW/GTRusthop.git
cd GTRusthop
```

**Important**: All commands below must be executed from the GTRusthop project root directory.

**Windows (PowerShell/Command Prompt):**
```powershell
# Ensure you're in the GTRusthop directory
GTRusthop> pwd
# Should show: ...\GTRusthop

# Run all examples
GTRusthop> cargo run

# Run with verbose output (PowerShell)
GTRusthop> $env:RUST_LOG = "debug"
GTRusthop> cargo run

# Or in a single line
GTRusthop> cmd /c "set RUST_LOG=debug && cargo run"

# Run tests
GTRusthop> cargo test

# Run specific example
GTRusthop> cargo run --example simple_htn
```

**Linux/macOS (bash/zsh):**
```bash
# Ensure you're in the GTRusthop directory
GTRusthop$ pwd
# Should show: .../GTRusthop

# Run all examples
GTRusthop$ cargo run

# Run with verbose output
GTRusthop$ RUST_LOG=debug cargo run

# Run tests
GTRusthop$ cargo test

# Run specific example
GTRusthop$ cargo run --example simple_htn
```

### Running Individual Examples

GTRusthop organizes examples as library modules with test functions. The recommended approach for running specific example modules is to use `cargo test` commands, which leverage the existing test infrastructure and provide the most straightforward execution method.

**Available Example Modules:**
- **`simple_htn_example`**: Basic HTN planning with travel scenarios and Pyhop compatibility
- **`blocks_htn_example`**: Classic blocks world planning using the Gupta-Nau algorithm
- **`backtracking_htn_example`**: HTN planning with backtracking capabilities
- **`simple_hgn_example`**: Hierarchical Goal Network examples
- **`logistics_hgn_example`**: HGN planning for logistics domain problems
- **`lazy_lookahead_example`**: Planning and acting with command failures
- **`regression_tests`**: Comprehensive regression test suite

**Planning Paradigms Demonstrated:**
- **HTN (Hierarchical Task Network)**: Uses task methods exclusively (`simple_htn_example`, `blocks_htn_example`, `backtracking_htn_example`)
- **HGN (Hierarchical Goal Network)**: Uses multigoal methods (`simple_hgn_example`, `logistics_hgn_example`)
- **Mixed**: Combines both approaches (`lazy_lookahead_example`)

**Windows (PowerShell/Command Prompt):**
```powershell
# Run simple HTN examples (travel scenarios) with output
GTRusthop> cargo test simple_htn_example -- --nocapture

# Run blocks world HTN examples with output
GTRusthop> cargo test blocks_htn_example -- --nocapture

# Run simple HGN examples with output
GTRusthop> cargo test simple_hgn_example -- --nocapture

# Run lazy lookahead examples with output
GTRusthop> cargo test lazy_lookahead_example -- --nocapture

# Run regression tests with output
GTRusthop> cargo test regression_tests -- --nocapture

# Run all tests with output (no race conditions due to builder pattern)
GTRusthop> cargo test -- --nocapture
```

**Linux/macOS (bash/zsh):**
```bash
# Run simple HTN examples (travel scenarios) with output
GTRusthop$ cargo test simple_htn_example -- --nocapture

# Run blocks world HTN examples with output
GTRusthop$ cargo test blocks_htn_example -- --nocapture

# Run simple HGN examples with output
GTRusthop$ cargo test simple_hgn_example -- --nocapture

# Run lazy lookahead examples with output
GTRusthop$ cargo test lazy_lookahead_example -- --nocapture

# Run regression tests with output
GTRusthop$ cargo test regression_tests -- --nocapture

# Run all tests with output (no race conditions due to builder pattern)
GTRusthop$ cargo test -- --nocapture
```

GTRusthop uses a thread-safe builder pattern architecture that eliminates race conditions. You can run all tests in parallel without any special flags!

#### Understanding the `-- --nocapture` Syntax

The `-- --nocapture` syntax may look confusing with its double dashes, but both are necessary and serve different purposes in the Cargo command structure:

**Syntax Breakdown:**
```bash
cargo test simple_htn_example -- --nocapture
│     │    │                 │  │
│     │    │                 │  └── Test runner flag (shows output)
│     │    │                 └────── Argument separator
│     │    └──────────────────────── Test filter (run specific test)
│     └───────────────────────────── Cargo subcommand
└─────────────────────────────────── Cargo binary
```

**Why Both Dashes Are Required:**
- **First `--`**: Cargo's argument separator - tells Cargo to stop parsing its own arguments
- **Second `--nocapture`**: Test runner flag - tells the test executable to display output instead of capturing it

This is standard Cargo/Rust testing syntax used across the Rust ecosystem.

**Why This Method is Recommended:**
- **Leverages existing infrastructure**: Uses the built-in test framework
- **No file modifications needed**: Doesn't require changing `main.rs` or creating custom binaries
- **Consistent execution**: Each example runs in a controlled test environment
- **Detailed output available**: Use `-- --nocapture` flag to see full example output

## Quick Start

### Installation

Add GTRusthop to your `Cargo.toml`:

```toml
[dependencies]
gtrusthop = "1.2.1"
```

### Basic Usage

GTRusthop provides two main APIs: the modern **Builder Pattern API** (recommended) and the legacy **Pyhop Compatibility API** for backward compatibility.

#### Modern Builder Pattern API (Recommended)

```rust
use gtrusthop::{Domain, State, PlanItem, PlannerBuilder, PlanningStrategy};
use gtrusthop::core::string_value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a domain
    let mut domain = Domain::new("transport_domain");

    // Declare an action
    domain.declare_action("move", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(obj), Some(dest)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("loc", obj, string_value(dest));
                return Some(state.clone());
            }
        }
        None
    })?;
    
    // Declare a task method
    domain.declare_task_method("transport", |state: &State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(obj), Some(dest)) = (args[0].as_str(), args[1].as_str()) {
                return Some(vec![
                    PlanItem::action("move", vec![string_value(obj), string_value(dest)])
                ]);
            }
        }
        None
    })?;
    
    // Create planner with builder pattern
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_strategy(PlanningStrategy::Iterative)
        .with_verbose_level(1)?
        .build()?;

    // Create initial state
    let mut state = State::new("initial");
    state.set_var("loc", "package", string_value("warehouse"));

    // Define task
    let todo_list = vec![
        PlanItem::task("transport", vec![
            string_value("package"),
            string_value("customer")
        ])
    ];

    // Find plan
    match planner.find_plan(state, todo_list)? {
        Some(plan) => {
            println!("Found plan:");
            for (i, action) in plan.iter().enumerate() {
                println!("  {}: {}", i + 1, action);
            }
        }
        None => println!("No plan found"),
    }
    
    Ok(())
}
```

#### Builder Pattern Benefits

- **🔒 Thread-Safe**: No global state, no race conditions
- **🎯 Isolated**: Each planner instance is completely independent
- **🔧 Configurable**: Fluent API for easy configuration
- **⚡ Performance**: No synchronization overhead
- **🧪 Testable**: Perfect for parallel testing

#### Multigoal Planning with Builder Pattern

GTRusthop supports complex multigoal planning using the builder pattern. Multigoals are registered directly with planner instances, eliminating global state:

```rust
use gtrusthop::{PlannerBuilder, Multigoal, State, PlanItem, string_value};
use gtrusthop::examples::blocks_htn_example::create_blocks_htn_domain;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a multigoal
    let mut goal = Multigoal::new("my_goal");
    goal.set_goal("pos", "a", string_value("table"));
    goal.set_goal("pos", "b", string_value("a"));

    // Create planner with multigoal
    let domain = create_blocks_htn_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_multigoal(goal)  // Register multigoal with planner
        .with_verbose_level(1)?
        .build()?;

    // Create initial state
    let mut state = State::new("initial");
    state.set_var("pos", "a", string_value("b"));
    state.set_var("pos", "b", string_value("table"));
    state.set_var("clear", "a", true.into());
    state.set_var("clear", "b", false.into());
    state.set_var("holding", "hand", false.into());

    // Plan to achieve the multigoal
    let plan = planner.find_plan(state, vec![
        PlanItem::task("achieve", vec![string_value("goal_my_goal")])
    ])?;

    match plan {
        Some(actions) => {
            println!("Found plan with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {}", i + 1, action);
            }
        }
        None => println!("No plan found"),
    }

    Ok(())
}
```

**Key Advantages of Builder Pattern Multigoals:**
- **Instance Isolation**: Each planner has its own multigoals
- **Thread Safety**: No global state, fully concurrent
- **No Manual Cleanup**: Automatic memory management
- **Type Safety**: Compile-time multigoal validation
- **Zero Thread-Local Storage**: Complete elimination of TLS dependencies
- **Clean Codebase**: All deprecated code removed, no legacy remnants

#### Pyhop Compatibility API (Legacy)

For users migrating from the original Pyhop planner, GTRusthop provides a `pyhop()` function that maintains backward compatibility:

```rust
use gtrusthop::{pyhop, Domain, State, PlanItem, string_value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create domain (same as above)
    let mut domain = Domain::new("transport_domain");

    // Declare actions and task methods (same as above)
    domain.declare_action("move", |state, args| { /* ... */ })?;
    domain.declare_task_method("transport", |state, args| { /* ... */ })?;

    // Create initial state
    let mut state = State::new("initial");
    state.set_var("loc", "package", string_value("warehouse"));

    // Define task
    let todo_list = vec![
        PlanItem::task("transport", vec![
            string_value("package"),
            string_value("customer")
        ])
    ];

    // Use legacy pyhop function (shows deprecation message)
    match pyhop(domain, state, todo_list)? {
        Some(plan) => println!("Found plan: {:?}", plan),
        None => println!("No plan found"),
    }

    Ok(())
}
```

**Migration Path**: The `pyhop()` function displays a deprecation message encouraging users to migrate to the modern `PlannerBuilder` + `find_plan()` approach for better control and thread safety.

#### HTN vs HGN Planning Examples

**HTN Planning (Task-Based)**:
```rust
// HTN uses task methods exclusively
domain.declare_task_method("travel", |state, args| {
    // Decompose travel task into subtasks
    Some(vec![
        PlanItem::task("get_taxi", args.to_vec()),
        PlanItem::task("ride_taxi", args.to_vec()),
        PlanItem::task("pay_taxi", args.to_vec())
    ])
})?;

// Call with task
let todo_list = vec![PlanItem::task("travel", vec![string_value("alice"), string_value("park")])];
```

**HGN Planning (Goal-Based)**:
```rust
// HGN uses multigoal methods
domain.declare_multigoal_method(|state, mgoal| {
    // Decompose goals into subgoals and actions
    Some(vec![
        PlanItem::action("move", vec![string_value("alice"), string_value("park")]),
        PlanItem::multigoal(remaining_goals)
    ])
})?;

// Call with multigoal
let mut goal = Multigoal::new("travel_goal");
goal.set_goal("loc", "alice", string_value("park"));
let todo_list = vec![PlanItem::multigoal(goal)];
```

### Lazy Lookahead Algorithm

GTRusthop includes the `run_lazy_lookahead` algorithm from Ghallab et al. (2016), "Automated Planning and Acting". This algorithm combines planning and acting by executing plans step-by-step and replanning when commands fail.

#### How It Works

1. **Plan**: Generate a plan for the current state and goals
2. **Execute**: Try to execute each action in the plan as a command
3. **Replan**: If a command fails, generate a new plan and try again
4. **Repeat**: Continue until goals are achieved or max attempts reached

#### Key Features

- **Command vs Action Distinction**: Actions are for planning, commands (prefixed with `c_`) are for execution
- **Failure Handling**: Automatic replanning when commands fail during execution
- **Verbose Output**: Detailed logging of planning and execution steps
- **Thread-Safe**: Works with the builder pattern architecture

#### Example Usage

```rust
use gtrusthop::{Domain, State, PlanItem, PlannerBuilder, string_value};

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

// Run lazy lookahead
let final_state = planner.run_lazy_lookahead(
    initial_state,
    todo_list,
    max_tries: 10
)?;
```

#### Benefits

- **Robust Execution**: Handles real-world command failures gracefully
- **Adaptive Planning**: Automatically adjusts plans based on execution results
- **Real-World Ready**: Bridges the gap between planning and acting
- **Debugging Support**: Verbose output helps understand execution flow

## Documentation

### For Developers
- **[API Reference](docs/api.md)**: Complete technical reference with function signatures, error handling, performance considerations, and integration guidelines
- **[Examples and Tutorials](docs/examples.md)**: Progressive learning guide with step-by-step tutorials, common pitfalls, and practice exercises
- **[Cross-Platform Commands](docs/cross-platform-commands.md)**: Platform-specific development setup, build commands, and cross-platform best practices for Windows, Linux, and macOS

### For Migration and Setup
- **[Rust Migration Guide](docs/Rust.md)**: Comprehensive guide covering installation, architectural differences from Python, and detailed comparison
- **[Working Examples](src/examples/)**: Functional code examples demonstrating domain creation, planning strategies, and real-world usage patterns
- **[Documentation Index](docs/DocumentationIndex.md)**: Comprehensive navigation guide with learning paths and detailed cross-references for all documentation
- **[Changelog](CHANGELOG.md)**: Version history, breaking changes, and migration notes for each release

## Architecture

GTRusthop is organized into several key modules:

- **`core`**: Core data structures (State, Domain, Multigoal)
- **`planning`**: Planning algorithms and strategies
- **`domains`**: Example domain implementations
- **`examples`**: Usage examples and test cases
- **`error`**: Error types and handling

## Planning Strategies

GTRusthop supports two planning strategies:

1. **Iterative**: Uses an explicit stack for planning (default)
2. **Recursive**: Uses the call stack for planning

```rust
use gtrusthop::planning::{set_planning_strategy, PlanningStrategy};

// Set iterative strategy (recommended)
set_planning_strategy(PlanningStrategy::Iterative);

// Set recursive strategy
set_planning_strategy(PlanningStrategy::Recursive);
```

## Verbose Output

Control planning output verbosity:

```rust
use gtrusthop::planning::set_verbose_level;

set_verbose_level(0)?; // No output
set_verbose_level(1)?; // Basic output (default)
set_verbose_level(2)?; // Detailed output
set_verbose_level(3)?; // Debug output
```

## Development Workflow

**Note**: All development commands must be executed from the GTRusthop project root directory.

**Windows (PowerShell/Command Prompt):**
```powershell
# Clean build artifacts
GTRusthop> cargo clean

# Build in debug mode
GTRusthop> cargo build

# Build in release mode
GTRusthop> cargo build --release

# Run tests
GTRusthop> cargo test

# Run clippy linter
GTRusthop> cargo clippy

# Format code
GTRusthop> cargo fmt

# Generate documentation
GTRusthop> cargo doc --open
```

**Linux/macOS (bash/zsh):**
```bash
# Clean build artifacts
GTRusthop$ cargo clean

# Build in debug mode
GTRusthop$ cargo build

# Build in release mode
GTRusthop$ cargo build --release

# Run tests
GTRusthop$ cargo test

# Run clippy linter
GTRusthop$ cargo clippy

# Format code
GTRusthop$ cargo fmt

# Generate documentation
GTRusthop$ cargo doc --open
```

## Comparison with GTPyhop

GTRusthop maintains API compatibility with GTPyhop while providing:

- **Better Performance**: Compiled Rust code is significantly faster
- **Memory Safety**: Rust's ownership system prevents memory errors
- **Type Safety**: Compile-time type checking catches errors early
- **Concurrency**: Safe parallel planning capabilities
- **No Runtime Dependencies**: Single binary with no interpreter required
- **Pyhop Compatibility**: Direct support for original Pyhop syntax via `pyhop()` function

### Migration from Python

**From GTPyhop**:
```python
# Python GTPyhop
import gtpyhop
plan = gtpyhop.find_plan(state, todo_list)
```

```rust
// Rust GTRusthop (modern)
let planner = PlannerBuilder::new().with_domain(domain).build()?;
let plan = planner.find_plan(state, todo_list)?;
```

**From Original Pyhop**:
```python
# Python Pyhop
import pyhop
plan = pyhop.pyhop(state, todo_list)
```

```rust
// Rust GTRusthop (compatibility)
use gtrusthop::pyhop;
let plan = pyhop(domain, state, todo_list)?;
```

See [Rust.md](docs/Rust.md) for detailed architectural differences and migration guide.

## Performance Benchmarks

GTRusthop includes comprehensive performance benchmarks comparing iterative vs recursive planning strategies across different problem sizes. Key findings:

- **Recursive strategy is 20-50% faster** than iterative strategy
- **Performance advantage increases** with problem complexity
- **Recursive strategy scales better** for large planning problems
- **Memory efficiency** is better with recursive approach

### Quick Benchmark Results

| Problem Size | Iterative (µs) | Recursive (µs) | Speedup |
|--------------|----------------|----------------|---------|
| 3 blocks     | 48.7           | 37.0           | 1.32x   |
| 8 blocks     | 215.9          | 142.6          | 1.51x   |
| 16 blocks    | 1441.2         | 757.5          | 1.90x   |

**Recommendation**: Use recursive strategy (default) for best performance.

### Running Benchmarks

```bash
# Run all performance benchmarks
cargo bench

# Quick test run
cargo bench -- --test
```

For detailed analysis, methodology, and complete results, see [Runtime Benchmark Report](docs/runtime_benchmark.md).

## License

GTRusthop is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

Many thanks to Dana Nau for all his work on HTN Planning in general and for Pythonizing HTN planning in particular.

## Citation

If you use GTRusthop in your research, please cite:

```bibtex
@software{gtrusthop2025,
  title = {GTRusthop: Goal-Task-Network Planning in Rust},
  author = {\'{E}ric Jacopin},
  year = {2025},
  month = {August},
  url = {https://github.com/PCfVW/GTRusthop},
  note = {Rust port of the pip branch of GTPyhop by \'{E}ric Jacopin with the help of Augment Code runnning on Claude Sonnet 4}
}
```

## Related Projects

Dana Nau's original Python projects:

- [GTPyhop](https://github.com/dananau/GTPyhop): Original Python implementation
- [Pyhop](https://github.com/dananau/pyhop): Classical HTN planner in Python
- [SHOP2](https://www.cs.umd.edu/projects/shop/): Simple Hierarchical Ordered Planner
