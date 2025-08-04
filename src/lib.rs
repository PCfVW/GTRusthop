//! # GTRusthop
//!
//! GTRusthop is a Goal-Task-Network (GTN) planning system written in Rust,
//! ported from the Python GTPyhop library by Dana Nau.
//!
//! This library provides both **HTN (Hierarchical Task Network)** and **HGN (Hierarchical Goal Network)**
//! planning capabilities, featuring recursive and iterative planning strategies, and backward
//! compatibility with the original Pyhop planner.
//!
//! ## Planning Paradigms
//!
//! GTRusthop supports two distinct planning approaches:
//!
//! ### HTN (Hierarchical Task Network) Planning
//!
//! HTN planning uses **task methods exclusively** to decompose abstract tasks into subtasks
//! and primitive actions. This approach is procedural and follows a step-by-step decomposition.
//!
//! ```rust
//! use gtrusthop::{Domain, State, PlanItem, PlannerBuilder, core::string_value};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create domain with task methods
//! let mut domain = Domain::new("travel_domain");
//!
//! // Declare a task method (HTN approach)
//! domain.declare_task_method("travel", |state, args| {
//!     if args.len() >= 3 {
//!         if let (Some(person), Some(from), Some(to)) =
//!             (args[0].as_str(), args[1].as_str(), args[2].as_str()) {
//!             return Some(vec![
//!                 PlanItem::task("get_taxi", vec![string_value(person)]),
//!                 PlanItem::task("ride_taxi", vec![string_value(person), string_value(from), string_value(to)]),
//!                 PlanItem::task("pay_taxi", vec![string_value(person)])
//!             ]);
//!         }
//!     }
//!     None
//! })?;
//!
//! // Create planner and plan with task
//! let planner = PlannerBuilder::new().with_domain(domain).build()?;
//! let mut state = State::new("initial");
//! state.set_var("loc", "alice", string_value("home"));
//!
//! let todo_list = vec![PlanItem::task("travel", vec![
//!     string_value("alice"), string_value("home"), string_value("park")
//! ])];
//!
//! let plan = planner.find_plan(state, todo_list)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### HGN (Hierarchical Goal Network) Planning
//!
//! HGN planning uses **multigoal methods** to decompose goals into subgoals and actions.
//! This approach is declarative and focuses on desired states.
//!
//! ```rust
//! use gtrusthop::{Domain, State, PlanItem, Multigoal, PlannerBuilder, core::string_value};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create domain with multigoal methods
//! let mut domain = Domain::new("goal_domain");
//!
//! // Declare a multigoal method (HGN approach)
//! domain.declare_multigoal_method(|state, mgoal| {
//!     // Decompose goals into actions and subgoals
//!     if let Some(target_loc) = mgoal.get_goal("loc", "alice") {
//!         return Some(vec![
//!             PlanItem::action("move", vec![string_value("alice"), target_loc.clone()]),
//!             // Additional subgoals would go here
//!         ]);
//!     }
//!     None
//! })?;
//!
//! // Create planner and plan with multigoal
//! let planner = PlannerBuilder::new().with_domain(domain).build()?;
//! let mut state = State::new("initial");
//! state.set_var("loc", "alice", string_value("home"));
//!
//! let mut goal = Multigoal::new("travel_goal");
//! goal.set_goal("loc", "alice", string_value("park"));
//! let todo_list = vec![PlanItem::multigoal(goal)];
//!
//! let plan = planner.find_plan(state, todo_list)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## API Overview
//!
//! GTRusthop provides three main APIs:
//!
//! ### 1. Modern Builder Pattern API (Recommended)
//!
//! ```rust,no_run
//! use gtrusthop::{PlannerBuilder, PlanningStrategy, Domain, State, PlanItem};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let domain = Domain::new("test");
//! # let state = State::new("test");
//! # let todo_list: Vec<PlanItem> = vec![];
//! let planner = PlannerBuilder::new()
//!     .with_domain(domain)
//!     .with_strategy(PlanningStrategy::Iterative)
//!     .with_verbose_level(1)?
//!     .build()?;
//!
//! let plan = planner.find_plan(state, todo_list)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### 2. Pyhop Compatibility API (Legacy)
//!
//! For backward compatibility with the original Pyhop planner:
//!
//! ```rust,no_run
//! use gtrusthop::{pyhop, Domain, State, PlanItem};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let domain = Domain::new("test");
//! # let state = State::new("test");
//! # let todo_list: Vec<PlanItem> = vec![];
//! // Legacy pyhop function (shows deprecation message)
//! let plan = pyhop(domain, state, todo_list)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### 3. Instance Method API
//!
//! ```rust
//! # use gtrusthop::{PlannerBuilder, Domain, State, PlanItem};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let domain = Domain::new("test");
//! # let state = State::new("test");
//! # let todo_list = vec![];
//! let planner = PlannerBuilder::new().with_domain(domain).build()?;
//!
//! // Modern approach
//! let plan = planner.find_plan(state.clone(), todo_list.clone())?;
//!
//! // Legacy compatibility method
//! let plan = planner.pyhop(state, todo_list)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Examples
//!
//! GTRusthop includes comprehensive examples demonstrating both planning paradigms:
//!
//! - **`simple_htn_example`**: Basic HTN planning with travel scenarios and Pyhop compatibility
//! - **`blocks_htn_example`**: Classic blocks world planning using the Gupta-Nau algorithm
//! - **`simple_hgn_example`**: Hierarchical Goal Network examples
//!
//! Run examples with: `cargo test simple_htn_example -- --nocapture`
//!
//! ## Key Features
//!
//! - **🔄 Dual Paradigms**: Support for both HTN (task-based) and HGN (goal-based) planning
//! - **🔙 Backward Compatibility**: `pyhop()` function for original Pyhop planner compatibility
//! - **🔒 Thread Safety**: Builder pattern eliminates global state and race conditions
//! - **⚡ Performance**: Compiled Rust code with multiple planning strategies
//! - **🎯 Type Safety**: Compile-time error checking and memory safety
//! - **🧪 Comprehensive Testing**: Extensive test suite with parallel execution support

pub mod core;
pub mod planning;
pub mod domains;
pub mod examples;
pub mod error;

// Re-export main types for convenience
pub use core::{Domain, State, Multigoal, PlanItem};
pub use planning::{
    // New builder pattern API
    PlannerBuilder, Planner,
    // Global configuration (still needed for some functionality)
    set_verbose_level,
    // Pyhop compatibility
    pyhop,
    // Common types
    PlanningStrategy
};
pub use error::{GTRustHopError, Result};

/// Current version of GTRusthop
pub const VERSION: &str = "1.2.1";
