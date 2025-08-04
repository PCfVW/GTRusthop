# GTRusthop: Rust Port of GTPyhop

GTRusthop is a complete Rust port of the GTPyhop Goal-Task-Network (GTN) planning library with significant architectural improvements. This document provides comprehensive information about the Rust implementation, including setup instructions, architectural differences, and migration considerations.

**🔒 Thread-Safe Architecture**: GTRusthop features a thread-safe builder pattern that eliminates race conditions and enables parallel planning without global state.

**⚡ Lazy Lookahead**: Complete implementation of the lazy lookahead algorithm for robust planning and acting.

## Table of Contents

1. [Thread-Safe Builder Pattern Migration](#thread-safe-builder-pattern-migration)
2. [Lazy Lookahead Algorithm](#lazy-lookahead-algorithm)
3. [Detailed Pedagogical Steps](#detailed-pedagogical-steps)
4. [Non-Portable Features Analysis](#non-portable-features-analysis)
5. [Major Architectural Differences](#major-architectural-differences)
6. [Minor Differences Summary](#minor-differences-summary)
7. [Platform-Specific Considerations](#platform-specific-considerations)

---

## 1. Thread-Safe Builder Pattern Migration

### Overview

GTRusthop has evolved from a global state architecture to a thread-safe builder pattern that eliminates race conditions and enables parallel planning. This section guides you through migrating from the old approach to the new architecture.

### ❌ Old Global State Approach (Deprecated)

The original GTRusthop used global state similar to the Python version:

```rust
// DEPRECATED - Don't use this approach!
use gtrusthop::{find_plan, planning::initialize_planning};

// Global state - causes race conditions
let domain = create_domain();
initialize_planning(domain);

// Uses global context - not thread-safe
let plan = find_plan(state, goals)?;
```

**Problems with Global State**:
- **Race Conditions**: Multiple threads interfere with each other
- **Unpredictable Behavior**: Tests fail randomly in parallel execution
- **Hard to Debug**: Global state makes issues difficult to reproduce
- **Not Scalable**: Cannot run multiple planners simultaneously

### ✅ New Builder Pattern Approach

The new architecture uses isolated planner instances:

```rust
use gtrusthop::{PlannerBuilder, PlanningStrategy};

// Create isolated planner instance
let domain = create_domain();
let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_verbose_level(1)?
    .with_strategy(PlanningStrategy::Iterative)
    .build()?;

// Thread-safe planning - no global state
let plan = planner.find_plan(state, goals)?;
```

**Benefits of Builder Pattern**:
- **🔒 Thread-Safe**: No global state, no race conditions
- **🎯 Isolated**: Each planner instance is completely independent
- **🔧 Configurable**: Fluent API for easy configuration
- **⚡ Performance**: No synchronization overhead
- **🧪 Testable**: Perfect for parallel testing

### Migration Examples

#### Before: Global State Planning

```rust
// OLD WAY - DEPRECATED
use gtrusthop::{find_plan, planning::initialize_planning, set_verbose_level};

fn old_planning_approach() -> gtrusthop::Result<()> {
    // Global configuration
    set_verbose_level(1)?;

    // Global domain initialization
    let domain = create_logistics_domain()?;
    initialize_planning(domain);

    // Planning uses global state
    let state = create_initial_state();
    let goals = create_goals();
    let plan = find_plan(state, goals)?;

    Ok(())
}
```

#### After: Builder Pattern Planning

```rust
// NEW WAY - RECOMMENDED
use gtrusthop::{PlannerBuilder, PlanningStrategy};

fn new_planning_approach() -> gtrusthop::Result<()> {
    // Create isolated planner instance
    let domain = create_logistics_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .with_strategy(PlanningStrategy::Iterative)
        .build()?;

    // Thread-safe planning
    let state = create_initial_state();
    let goals = create_goals();
    let plan = planner.find_plan(state, goals)?;

    Ok(())
}
```

### Parallel Planning Migration

#### Before: Race Conditions

```rust
// OLD WAY - RACE CONDITIONS!
use std::thread;

fn parallel_planning_old() {
    let domain = create_domain().unwrap();
    initialize_planning(domain); // Global state!

    let handles: Vec<_> = (0..4).map(|i| {
        thread::spawn(move || {
            // All threads share global state - RACE CONDITIONS!
            let state = create_state_for_thread(i);
            let goals = create_goals_for_thread(i);
            find_plan(state, goals) // Unpredictable results
        })
    }).collect();

    // Results are unpredictable due to race conditions
    for handle in handles {
        println!("{:?}", handle.join().unwrap());
    }
}
```

#### After: Thread-Safe Parallel Planning

```rust
// NEW WAY - THREAD-SAFE!
use std::thread;

fn parallel_planning_new() {
    let domain = create_domain().unwrap();

    let handles: Vec<_> = (0..4).map(|i| {
        let domain_clone = domain.clone(); // Cheap clone

        thread::spawn(move || {
            // Each thread gets isolated planner - NO RACE CONDITIONS!
            let planner = PlannerBuilder::new()
                .with_domain(domain_clone)
                .with_verbose_level(0).unwrap() // Avoid mixed output
                .build().unwrap();

            let state = create_state_for_thread(i);
            let goals = create_goals_for_thread(i);
            planner.find_plan(state, goals) // Predictable results
        })
    }).collect();

    // All results are predictable and correct
    for (i, handle) in handles.into_iter().enumerate() {
        println!("Thread {}: {:?}", i, handle.join().unwrap());
    }
}
```

### Testing Migration

#### Before: Flaky Tests

```rust
// OLD WAY - TESTS FAIL RANDOMLY
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning() {
        let domain = create_test_domain().unwrap();
        initialize_planning(domain); // Global state

        let plan = find_plan(state, goals).unwrap();
        assert!(plan.is_some()); // Might fail randomly in parallel
    }
}
```

#### After: Reliable Tests

```rust
// NEW WAY - TESTS ALWAYS PASS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning() {
        let domain = create_test_domain().unwrap();
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0).unwrap() // Quiet for tests
            .build().unwrap();

        let plan = planner.find_plan(state, goals).unwrap();
        assert!(plan.is_some()); // Always reliable
    }
}
```

---

## 2. Lazy Lookahead Algorithm

### What is Lazy Lookahead?

The lazy lookahead algorithm from Ghallab et al. (2016), "Automated Planning and Acting", combines planning and acting by executing plans step-by-step and replanning when commands fail.

### Python vs Rust Implementation

#### Python GTPyhop

```python
# Python implementation
def run_lazy_lookahead(state, todo_list, max_tries=10):
    for tries in range(1, max_tries + 1):
        plan = find_plan(state, todo_list)
        if not plan:
            return state
        if plan == []:
            return state

        for action in plan:
            command_name = f"c_{action[0]}"
            if command_name in domain._command_dict:
                result = domain._command_dict[command_name](state, *action[1:])
                if result:
                    state = result
                else:
                    break  # Command failed, replan
    return state
```

#### Rust GTRusthop

```rust
// Rust implementation - thread-safe and robust
impl Planner {
    pub fn run_lazy_lookahead(
        &self,
        mut state: State,
        todo_list: Vec<PlanItem>,
        max_tries: usize,
    ) -> Result<State> {
        for tries in 1..=max_tries {
            let plan = self.find_plan(state.clone(), todo_list.clone())?;

            match plan {
                None => return Ok(state),
                Some(plan) if plan.is_empty() => return Ok(state),
                Some(plan) => {
                    let mut plan_failed = false;
                    for action in &plan {
                        if let PlanItem::Action(action_name, args) = action {
                            let command_name = format!("c_{}", action_name);

                            // Try command, fall back to action
                            let command_fn = self.domain.get_command(&command_name)
                                .or_else(|| self.domain.get_action(action_name));

                            if let Some(cmd_fn) = command_fn {
                                let mut state_copy = state.copy(None);
                                if let Some(new_state) = cmd_fn(&mut state_copy, args) {
                                    state = new_state;
                                } else {
                                    plan_failed = true;
                                    break; // Command failed, replan
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(state)
    }
}
```

### Usage Example

```rust
use gtrusthop::{PlannerBuilder, Domain, State, PlanItem, core::string_value};

fn lazy_lookahead_example() -> gtrusthop::Result<()> {
    // Create domain with actions and commands
    let mut domain = Domain::new("taxi_domain");

    // Action for planning (optimistic)
    domain.declare_action("call_taxi", |state, _args| {
        Some(state.clone()) // Assume taxi always comes
    })?;

    // Command for execution (realistic)
    domain.declare_command("c_call_taxi", |state, _args| {
        if taxi_available() {
            Some(state.clone())
        } else {
            None // Taxi not available - triggers replanning
        }
    })?;

    // Create planner
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Initial state
    let mut state = State::new("initial");
    state.set_var("location", "alice", string_value("home"));

    // Goals
    let goals = vec![
        PlanItem::task("travel", vec![
            string_value("alice"),
            string_value("home"),
            string_value("work")
        ])
    ];

    // Run lazy lookahead - handles failures automatically
    let final_state = planner.run_lazy_lookahead(state, goals, 10)?;

    println!("Final location: {:?}",
             final_state.get_var("location", "alice"));

    Ok(())
}
```

### Key Advantages in Rust

- **Type Safety**: Compile-time guarantees prevent runtime errors
- **Memory Safety**: No memory leaks or dangling pointers
- **Thread Safety**: Can run multiple lazy lookahead instances in parallel
- **Performance**: Zero-cost abstractions and efficient execution
- **Error Handling**: Robust error propagation with `Result` types

---

## 3. Detailed Pedagogical Steps

### Prerequisites

Before working with GTRusthop, ensure you have the following installed:

#### Installing Rust and Cargo

**Windows:**
1. **Download and run rustup-init.exe** from [rustup.rs](https://rustup.rs/)
2. **Or install via PowerShell**:
   ```powershell
   # Download and run rustup installer
   Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
   .\rustup-init.exe

   # Restart PowerShell or reload environment
   refreshenv  # If using Chocolatey
   # Or restart your terminal
   ```

3. **Verify installation**:
   ```powershell
   rustc --version
   cargo --version
   ```

4. **Update Rust**:
   ```powershell
   rustup update
   ```

**Linux/macOS:**
1. **Install Rust via rustup** (recommended):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Verify installation**:
   ```bash
   rustc --version
   cargo --version
   ```

3. **Update Rust** (if already installed):
   ```bash
   rustup update
   ```

4. **Add to shell profile** (if not done automatically):
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   # Or for zsh:
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
   ```

### Building the GTRusthop Project

**Windows (PowerShell/Command Prompt):**
```powershell
# Navigate to the project directory
cd GTRusthop

# Build the project
cargo build

# Build in release mode (optimized)
cargo build --release

# Check project structure
dir src\
```

**Linux/macOS (bash/zsh):**
```bash
# Navigate to the project directory
cd GTRusthop

# Build the project
cargo build

# Build in release mode (optimized)
cargo build --release

# Check project structure
ls -la src/
```

### Running the Planner with Example Problems

**Windows (PowerShell/Command Prompt):**
```powershell
# Run the basic examples
cargo run

# Run with verbose output (PowerShell)
$env:RUST_LOG = "debug"
cargo run

# Or using Command Prompt
cmd /c "set RUST_LOG=debug && cargo run"

# Run in release mode for better performance
cargo run --release

# Time the execution
Measure-Command { cargo run --release }
```

**Linux/macOS (bash/zsh):**
```bash
# Run the basic examples
cargo run

# Run with verbose output
RUST_LOG=debug cargo run

# Run in release mode for better performance
cargo run --release

# Time the execution
time cargo run --release
```

**Programmatic Usage** (same for all platforms):
```rust
use gtrusthop::examples::run_simple_htn_examples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_simple_htn_examples()?;
    Ok(())
}
```

### Executing All Test Suites

**Windows (PowerShell/Command Prompt):**
```powershell
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test modules
cargo test core::state::tests
cargo test planning::tests
cargo test domains::tests

# Run integration tests
cargo test --test integration_tests

# Run documentation tests
cargo test --doc

# Run tests in release mode
cargo test --release

# Run tests with timing
Measure-Command { cargo test }
```

**Linux/macOS (bash/zsh):**
```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test modules
cargo test core::state::tests
cargo test planning::tests
cargo test domains::tests

# Run integration tests
cargo test --test integration_tests

# Run documentation tests
cargo test --doc

# Run tests in release mode
cargo test --release

# Run tests with timing
time cargo test
```

### Understanding the Codebase Structure for Learning

The GTRusthop codebase is organized into several key modules:

```
src/
├── lib.rs              # Main library entry point
├── main.rs             # Example executable
├── error.rs            # Error handling types
├── core/               # Core data structures
│   ├── mod.rs          # Module exports
│   ├── state.rs        # State representation
│   ├── multigoal.rs    # Multigoal representation
│   └── domain.rs       # Domain with actions/methods
├── planning/           # Planning algorithms
│   ├── mod.rs          # Planning utilities
│   ├── planner.rs      # Main planning functions
│   ├── strategy.rs     # Planning strategies
│   └── verification.rs # Goal verification
├── domains/            # Example domains
│   ├── mod.rs          # Domain utilities
│   ├── simple_htn.rs   # Simple HTN domain
│   ├── simple_hgn.rs   # Simple HGN domain
│   └── blocks_htn.rs   # Blocks world domain
└── examples/           # Example usage
    ├── mod.rs          # Example utilities
    ├── simple_htn_example.rs
    ├── simple_hgn_example.rs
    ├── blocks_htn_example.rs
    └── regression_tests.rs
```

#### Key Learning Points:

1. **Start with `src/core/`**: Understand the basic data structures (State, Multigoal, Domain)
2. **Examine `src/planning/`**: Learn how the planning algorithms work
3. **Study `src/domains/simple_htn.rs`**: See how to define actions and methods
4. **Run `src/examples/`**: Observe the planner in action
5. **Read tests**: Each module has comprehensive tests showing usage patterns

#### Basic Usage Pattern:

```rust
use gtrusthop::{Domain, State, PlanItem, PlannerBuilder, PlanningStrategy};
use gtrusthop::core::string_value;

// 1. Create a domain
let mut domain = Domain::new("my_domain");

// 2. Declare actions
domain.declare_action("move", |state: &mut State, args: &[StateValue]| {
    // Action implementation
    Some(state.clone())
})?;

// 3. Declare task methods
domain.declare_task_method("transport", |state: &State, args: &[StateValue]| {
    // Method implementation
    Some(vec![PlanItem::action("move", args.to_vec())])
})?;

// 4. Create planner with builder pattern (thread-safe)
let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_strategy(PlanningStrategy::Iterative)
    .with_verbose_level(1)?
    .build()?;

// 5. Create initial state
let mut state = State::new("initial");
state.set_var("loc", "obj1", string_value("loc1"));

// 6. Define goals
let todo_list = vec![PlanItem::task("transport", vec![
    string_value("obj1"),
    string_value("loc2")
])];

// 7. Find plan (thread-safe, no global state)
let plan = planner.find_plan(state, todo_list)?;
```

---

## 4. Non-Portable Features Analysis

Several Python features cannot be directly ported to Rust due to fundamental language differences:

### Dynamic Typing and Runtime Reflection

**Python Feature**: Dynamic attribute access and runtime type inspection
```python
# Python - Dynamic attribute access
state.loc['alice'] = 'park'
hasattr(state, 'loc')
getattr(state, 'loc', {})
```

**Technical Reason**: Rust is statically typed and doesn't support runtime attribute creation or dynamic property access.

**Rust Alternative**: Use structured data with HashMap or custom getter/setter methods:
```rust
// Rust - Structured access
state.set_var("loc", "alice", string_value("park"));
state.has_var("loc");
state.get_var("loc", "alice");
```

### Function Introspection and Dynamic Dispatch

**Python Feature**: Inspecting function signatures and dynamic method calls
```python
# Python - Function introspection
import inspect
sig = inspect.signature(action_function)
params = sig.parameters
```

**Technical Reason**: Rust doesn't have runtime reflection capabilities for function introspection.

**Rust Alternative**: Use trait objects and explicit type definitions:
```rust
// Rust - Trait-based dispatch
type ActionFn = Arc<dyn Fn(&mut State, &[StateValue]) -> Option<State> + Send + Sync>;
```

### Global Mutable State

**Python Feature**: Module-level global variables that can be modified
```python
# Python - Global state
current_domain = None
verbose_level = 1

def set_domain(domain):
    global current_domain
    current_domain = domain
```

**Technical Reason**: Rust's ownership system prevents shared mutable global state without explicit synchronization.

**Rust Alternative**: Use thread-safe global state with Mutex or Arc:
```rust
// Rust - Thread-safe global state
static VERBOSE_LEVEL: Mutex<i32> = Mutex::new(1);
static PLANNING_CONTEXT: Mutex<Option<PlanningContext>> = Mutex::new(None);
```

### Exception-Based Control Flow

**Python Feature**: Using exceptions for control flow in planning
```python
# Python - Exception for verification failure
def verify_goal(state, goal):
    if not goal_satisfied(state, goal):
        raise GoalNotAchieved(f"Goal {goal} not achieved")
```

**Technical Reason**: Rust uses Result types instead of exceptions for error handling.

**Rust Alternative**: Use Result types for explicit error handling:
```rust
// Rust - Result-based error handling
fn verify_goal(state: &State, goal: &Goal) -> Result<(), GTRustHopError> {
    if !goal_satisfied(state, goal) {
        Err(GTRustHopError::method_verification_failed("method", "goal", 0))
    } else {
        Ok(())
    }
}
```

### Duck Typing and Flexible Interfaces

**Python Feature**: Duck typing allows objects with similar interfaces to be used interchangeably
```python
# Python - Duck typing
def process_item(item):
    if hasattr(item, 'execute'):
        return item.execute()
    elif callable(item):
        return item()
```

**Technical Reason**: Rust requires explicit type definitions and trait implementations.

**Rust Alternative**: Use enums or trait objects for polymorphism:
```rust
// Rust - Explicit polymorphism
enum PlanItem {
    Task(String, Vec<StateValue>),
    Action(String, Vec<StateValue>),
    Unigoal(String, String, StateValue),
    Multigoal(Multigoal),
}
```

---

## 5. Major Architectural Differences

### Memory Management

**Python**: Garbage collection handles memory automatically
- Objects are reference-counted with cycle detection
- Memory is freed automatically when references reach zero
- No explicit memory management required

**Rust**: Ownership system with compile-time memory safety
- Each value has a single owner
- Memory is freed when owner goes out of scope
- Borrowing system allows temporary access without ownership transfer
- No runtime garbage collection overhead

**Impact**: Rust code requires careful consideration of ownership and lifetimes, but provides better performance and memory safety guarantees.

### Type System

**Python**: Dynamic typing with runtime type checking
- Variables can hold any type of value
- Type errors discovered at runtime
- Duck typing allows flexible interfaces

**Rust**: Static typing with compile-time type checking
- All types must be known at compile time
- Type errors caught during compilation
- Explicit trait implementations required for polymorphism

**Impact**: Rust catches more errors at compile time but requires more explicit type annotations and interface definitions.

### Error Handling

**Python**: Exception-based error handling
- Exceptions can be raised and caught anywhere in the call stack
- Unhandled exceptions terminate the program
- Try/except blocks for error recovery

**Rust**: Result-based error handling
- Functions return Result<T, E> for operations that can fail
- Errors must be explicitly handled or propagated
- No hidden control flow through exceptions

**Impact**: Rust error handling is more explicit and predictable, but requires more boilerplate code for error propagation.

### Concurrency Model

**Python**: Global Interpreter Lock (GIL) limits true parallelism
- Threading is cooperative, not truly parallel for CPU-bound tasks
- Asyncio for asynchronous I/O operations
- Multiprocessing for CPU-bound parallelism

**Rust**: Fearless concurrency with ownership guarantees
- True parallelism with thread safety enforced at compile time
- Send and Sync traits ensure safe data sharing
- No data races possible due to ownership system

**Impact**: Rust enables safer and more efficient concurrent programming, but requires understanding of ownership and lifetime concepts.

### Performance Characteristics

**Python**: Interpreted language with runtime overhead
- Dynamic dispatch and type checking at runtime
- Memory allocation through garbage collector
- Generally slower execution speed

**Rust**: Compiled language with zero-cost abstractions
- Static dispatch and compile-time optimizations
- Manual memory management without GC overhead
- Performance comparable to C/C++

**Impact**: Rust provides significantly better performance for CPU-intensive planning tasks, especially with large state spaces.

---

## 6. Minor Differences Summary

### Syntax Variations

| Aspect | Python | Rust |
|--------|--------|------|
| Variable declaration | `x = 5` | `let x = 5;` |
| Mutable variables | `x = 5` (always mutable) | `let mut x = 5;` |
| Function definition | `def func(x):` | `fn func(x: i32) -> i32 {` |
| String literals | `"hello"` or `'hello'` | `"hello"` |
| Comments | `# comment` | `// comment` |
| Block structure | Indentation | Braces `{}` |

### Standard Library Differences

| Feature | Python | Rust |
|---------|--------|------|
| Hash maps | `dict` | `HashMap` or `IndexMap` |
| Dynamic arrays | `list` | `Vec` |
| String type | `str` | `String` or `&str` |
| JSON handling | `json` module | `serde_json` crate |
| Regular expressions | `re` module | `regex` crate |

### Naming Convention Changes

| Python Convention | Rust Convention | Example |
|-------------------|-----------------|---------|
| snake_case | snake_case | `find_plan` (same) |
| PascalCase | PascalCase | `Domain` (same) |
| UPPER_CASE | UPPER_CASE | `VERSION` (same) |
| Module names | snake_case | `simple_htn` |

### File Organization Differences

**Python**: 
- Modules are files (`.py`)
- Packages are directories with `__init__.py`
- Imports use dot notation

**Rust**:
- Modules declared with `mod` keyword
- File structure mirrors module structure
- `mod.rs` files define module contents
- `use` statements for imports

### Build System Differences

| Aspect | Python | Rust |
|--------|--------|------|
| Package manager | pip, poetry, conda | Cargo |
| Dependency file | requirements.txt, pyproject.toml | Cargo.toml |
| Build command | `python setup.py build` | `cargo build` |
| Test command | `python -m pytest` | `cargo test` |
| Run command | `python script.py` | `cargo run` |
| Virtual environments | venv, virtualenv | Built into Cargo |

### Documentation Differences

**Python**: 
- Docstrings with triple quotes
- Sphinx for documentation generation
- Type hints with `typing` module

**Rust**:
- Documentation comments with `///`
- rustdoc for documentation generation
- Built-in type system with explicit types

### Testing Framework Differences

**Python**:
- unittest, pytest frameworks
- Separate test files or test classes
- Dynamic test discovery

**Rust**:
- Built-in test framework
- `#[test]` attribute for test functions
- Tests can be in same file or separate `tests/` directory
- `cargo test` runs all tests

These differences reflect the fundamental design philosophies of the two languages: Python prioritizes developer productivity and flexibility, while Rust prioritizes safety, performance, and correctness.

---

## 7. Platform-Specific Considerations

### Windows-Specific Notes

#### File Paths and Separators
```rust
// Windows uses backslashes in paths
let config_path = r"C:\MyProject\config.toml";
let domain_dir = r"C:\MyProject\domains\";

// Or use forward slashes (Rust normalizes them)
let config_path = "C:/MyProject/config.toml";

// Use std::path for cross-platform compatibility
use std::path::PathBuf;
let mut path = PathBuf::from("C:");
path.push("MyProject");
path.push("config.toml");
```

#### Environment Variables
```powershell
# PowerShell syntax
$env:RUST_LOG = "debug"
$env:GTRUSTHOP_CONFIG = "C:\MyProject\config.toml"

# Command Prompt syntax
set RUST_LOG=debug
set GTRUSTHOP_CONFIG=C:\MyProject\config.toml
```

#### Performance Considerations
- **Windows Defender**: May slow down compilation. Consider adding Rust directories to exclusions:
  - `%USERPROFILE%\.cargo`
  - `%USERPROFILE%\.rustup`
  - Your project directories

#### Development Tools
```powershell
# Install useful Windows tools
winget install Microsoft.VisualStudioCode
winget install Git.Git

# Cargo extensions
cargo install cargo-watch
cargo install cargo-edit
cargo install cargo-audit
```

### Linux/macOS-Specific Notes

#### File Paths and Permissions
```rust
// Unix-like systems use forward slashes
let config_path = "/home/user/myproject/config.toml";
let domain_dir = "/home/user/myproject/domains/";

// Use std::path for cross-platform compatibility
use std::path::PathBuf;
let mut path = PathBuf::from(env::var("HOME").unwrap());
path.push("myproject");
path.push("config.toml");
```

#### Environment Variables
```bash
# Bash/Zsh syntax
export RUST_LOG="debug"
export GTRUSTHOP_CONFIG="/home/user/myproject/config.toml"

# Temporary for single command
RUST_LOG=debug cargo run
```

#### Package Manager Integration
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config

# macOS with Homebrew
brew install pkg-config

# Arch Linux
sudo pacman -S base-devel
```

#### Development Tools
```bash
# Install useful tools
# VS Code
sudo snap install code --classic  # Ubuntu
brew install --cask visual-studio-code  # macOS

# Cargo extensions
cargo install cargo-watch
cargo install cargo-edit
cargo install cargo-audit
```

### Cross-Platform Best Practices

#### Path Handling
```rust
use std::path::{Path, PathBuf};
use std::env;

// Always use PathBuf for file operations
fn get_config_path() -> PathBuf {
    let mut path = PathBuf::new();

    if cfg!(windows) {
        path.push(env::var("APPDATA").unwrap_or_else(|_| "C:\\".to_string()));
    } else {
        path.push(env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()));
        path.push(".config");
    }

    path.push("gtrusthop");
    path.push("config.toml");
    path
}
```

#### Environment Variable Handling
```rust
use std::env;

fn get_log_level() -> String {
    env::var("GTRUSTHOP_LOG_LEVEL")
        .or_else(|_| env::var("RUST_LOG"))
        .unwrap_or_else(|_| "info".to_string())
}
```

#### Conditional Compilation
```rust
#[cfg(windows)]
fn platform_specific_setup() {
    // Windows-specific initialization
    println!("Running on Windows");
}

#[cfg(unix)]
fn platform_specific_setup() {
    // Unix-like specific initialization
    println!("Running on Unix-like system");
}

#[cfg(target_os = "macos")]
fn macos_specific_setup() {
    // macOS-specific code
}
```

### Performance Optimization by Platform

#### Windows Optimizations
```toml
# Cargo.toml - Windows-specific optimizations
[profile.release]
opt-level = 3
lto = true
codegen-units = 1

# Windows-specific target features
[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser", "processthreadsapi"] }
```

#### Linux/macOS Optimizations
```toml
# Cargo.toml - Unix optimizations
[profile.release]
opt-level = 3
lto = true
codegen-units = 1

# Unix-specific dependencies
[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

#### Build Scripts
```bash
#!/bin/bash
# build.sh - Cross-platform build script

if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    echo "Building for Windows..."
    cargo build --release --target x86_64-pc-windows-msvc
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Building for macOS..."
    cargo build --release --target x86_64-apple-darwin
else
    echo "Building for Linux..."
    cargo build --release --target x86_64-unknown-linux-gnu
fi
```

```powershell
# build.ps1 - Windows PowerShell build script
Write-Host "Building GTRusthop for Windows..."

# Set optimization flags
$env:RUSTFLAGS="-C target-cpu=native"

# Build
cargo build --release --target x86_64-pc-windows-msvc

Write-Host "Build complete!"
```

These platform-specific considerations ensure optimal development experience and performance across all supported operating systems.
