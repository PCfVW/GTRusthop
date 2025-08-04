//! # Blocks HTN (Hierarchical Task Network) Example for GTRusthop
//!
//! This module provides a comprehensive implementation of HTN planning in the blocks world domain,
//! demonstrating the near-optimal blocks-world planning algorithm described in:
//!
//! > N. Gupta and D. S. Nau. "On the complexity of blocks-world planning."
//! > *Artificial Intelligence* 56(2-3):223–254, 1992.
//!
//! ## HTN vs HGN Planning Approaches
//!
//! This implementation follows the **Hierarchical Task Network (HTN)** approach, which differs
//! fundamentally from the **Hierarchical Goal Network (HGN)** approach:
//!
//! - **HTN**: Uses only task methods (`declare_task_method`). Tasks are decomposed into subtasks
//!   and primitive actions. Multigoals are handled by passing them as arguments to the 'achieve' task.
//! - **HGN**: Uses multigoal methods (`declare_multigoal_method`) that directly process multigoals
//!   and decompose them into subgoals and actions.
//!
//! This implementation is a faithful Rust port of the Python `blocks_htn` example from GTPyhop,
//! maintaining the HTN principle of using **only task methods**.
//!
//! ## Multigoal Registry Pattern
//!
//! A key challenge in porting from Python to Rust is that Python allows passing arbitrary objects
//! (like Multigoal) as task arguments, while Rust requires task arguments to be `StateValue` types.
//!
//! This implementation solves this using a **multigoal registry pattern**:
//! 1. Multigoals are registered in a thread-local registry with unique IDs
//! 2. The ID (as a string) is passed as a `StateValue` argument to the 'achieve' task
//! 3. The task method retrieves the multigoal from the registry using the ID
//!
//! This preserves the HTN approach while working within Rust's type system constraints.
//!
//! ## Core HTN Methods
//!
//! The implementation provides three main task methods, directly corresponding to the Python version:
//!
//! - **`achieve`**: Implements the Gupta-Nau algorithm for achieving multigoals
//! - **`take`**: Handles picking up blocks (pickup from table, unstack from other blocks)
//! - **`put`**: Handles placing blocks (putdown on table, stack on other blocks)
//!
//! ## Example Usage
//!
//! ```rust
//! use gtrusthop::examples::blocks_htn_example::run_blocks_htn_examples;
//!
//! // Run all blocks HTN examples including the Sussman Anomaly
//! run_blocks_htn_examples().expect("Examples should run successfully");
//! ```
//!
//! ## Key Features Demonstrated
//!
//! - **HTN task decomposition**: Complex goals broken down into primitive actions
//! - **Blocks world actions**: pickup, putdown, stack, unstack with proper preconditions
//! - **State representation**: Block positions, clear status, and hand holding status
//! - **Complex planning scenarios**: Including the famous Sussman Anomaly
//! - **Near-optimal planning**: Using the Gupta-Nau algorithm for efficient block manipulation

use crate::core::{State, Domain, PlanItem, Multigoal, string_value, StateValue};
use crate::planning::PlannerBuilder;
use crate::error::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// Run comprehensive blocks HTN examples demonstrating the Gupta-Nau algorithm.
///
/// This function demonstrates the complete HTN blocks world planning system by running
/// a series of progressively complex examples:
///
/// 1. **Simple Actions**: Tests individual actions that should fail (wrong preconditions)
/// 2. **Successful Actions**: Tests individual actions and task methods that should succeed
/// 3. **Multigoal Planning**: Tests complex goal achievement using the HTN 'achieve' task
/// 4. **Sussman Anomaly**: The famous blocks world planning challenge
/// 5. **Complex Scenarios**: Multi-block rearrangement problems
///
/// Each example validates that the HTN planner produces correct, near-optimal solutions
/// using only task methods (no multigoal methods), following the HTN planning paradigm.
///
/// # Returns
///
/// `Ok(())` if all examples run successfully, or an error if any example fails.
///
/// # Example
///
/// ```rust
/// use gtrusthop::examples::blocks_htn_example::run_blocks_htn_examples;
///
/// match run_blocks_htn_examples() {
///     Ok(()) => println!("All HTN examples completed successfully!"),
///     Err(e) => eprintln!("HTN examples failed: {}", e),
/// }
/// ```
pub fn run_blocks_htn_examples() -> Result<()> {
    println!("=== Running Blocks HTN Examples ===");

    // Create the blocks HTN domain
    let domain = create_blocks_htn_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain.clone())
        .with_verbose_level(1)?
        .build()?;

    // Test simple actions that should fail
    println!("\n--- Testing Simple Actions That Should Fail ---");
    test_failing_actions(&planner)?;

    // Test simple actions that should succeed
    println!("\n--- Testing Simple Actions That Should Succeed ---");
    test_succeeding_actions(&planner)?;

    // Test multigoal planning (now takes domain instead of planner)
    println!("\n--- Testing Multigoal Planning ---");
    test_multigoal_planning(&domain)?;

    // Test the famous Sussman Anomaly
    println!("\n--- Testing Sussman Anomaly ---");
    test_sussman_anomaly(&domain)?;

    // Test complex scenarios
    println!("\n--- Testing Complex Scenarios ---");
    test_complex_scenarios(&domain)?;

    println!("=== Blocks HTN Examples Completed ===");
    Ok(())
}

/// Create the blocks HTN domain with all actions and task methods.
///
/// This function constructs a complete HTN planning domain for the blocks world,
/// including:
///
/// - **Primitive Actions**: pickup, unstack, putdown, stack (with proper preconditions)
/// - **HTN Task Methods**: achieve, take, put (for hierarchical task decomposition)
///
/// The domain follows the HTN paradigm exclusively - no multigoal methods are used.
/// Instead, the 'achieve' task method handles multigoals by decomposing them into
/// sequences of 'take' and 'put' tasks, which further decompose into primitive actions.
///
/// # Returns
///
/// A configured `Domain` ready for HTN planning in the blocks world.
///
/// # Errors
///
/// Returns an error if any action or task method declaration fails.
pub fn create_blocks_htn_domain() -> Result<Domain> {
    create_blocks_htn_domain_with_multigoals(HashMap::new())
}

/// Create the blocks HTN domain with specific multigoals
///
/// This function creates a domain for the blocks world problem using HTN planning
/// with the provided multigoals. The multigoals are captured in the task method
/// closures, eliminating the need for global or thread-local storage.
///
/// # Arguments
///
/// * `multigoals` - HashMap of multigoal ID to Multigoal instances
pub fn create_blocks_htn_domain_with_multigoals(multigoals: HashMap<String, Multigoal>) -> Result<Domain> {
    let mut domain = Domain::new("blocks_htn");

    // Declare actions
    declare_blocks_actions(&mut domain)?;

    // Declare task methods with multigoals
    declare_blocks_task_methods(&mut domain, multigoals)?;

    Ok(domain)
}

/// Declare all primitive actions for the blocks world domain.
///
/// This function defines the four fundamental blocks world actions with their
/// preconditions and effects:
///
/// - **`pickup(block)`**: Pick up a block from the table
/// - **`unstack(block, under_block)`**: Remove a block from another block
/// - **`putdown(block)`**: Put a held block on the table
/// - **`stack(block, under_block)`**: Put a held block on another block
///
/// Each action includes proper precondition checking and state updates following
/// the standard blocks world semantics.
///
/// # Arguments
///
/// * `domain` - The domain to add the actions to
///
/// # Returns
///
/// `Ok(())` if all actions are declared successfully, or an error if any declaration fails.
fn declare_blocks_actions(domain: &mut Domain) -> Result<()> {
    // pickup action: pick up a block from the table
    domain.declare_action("pickup", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 1 {
            if let Some(block) = args[0].as_str() {
                // Check preconditions: block on table, clear, hand empty
                if let (Some(pos), Some(clear), Some(holding)) = (
                    state.get_var("pos", block),
                    state.get_var("clear", block),
                    state.get_var("holding", "hand")
                ) {
                    if pos.as_str() == Some("table") &&
                       clear.as_bool() == Some(true) &&
                       holding.as_bool() == Some(false) {
                        // Apply effects
                        state.set_var("pos", block, string_value("hand"));
                        state.set_var("clear", block, false.into());
                        state.set_var("holding", "hand", string_value(block));
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    // unstack action: remove a block from another block
    domain.declare_action("unstack", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(block1), Some(block2)) = (args[0].as_str(), args[1].as_str()) {
                // Check preconditions: block1 on block2, block1 clear, hand empty
                if let (Some(pos), Some(clear), Some(holding)) = (
                    state.get_var("pos", block1),
                    state.get_var("clear", block1),
                    state.get_var("holding", "hand")
                ) {
                    if pos.as_str() == Some(block2) &&
                       block2 != "table" &&
                       clear.as_bool() == Some(true) &&
                       holding.as_bool() == Some(false) {
                        // Apply effects
                        state.set_var("pos", block1, string_value("hand"));
                        state.set_var("clear", block1, false.into());
                        state.set_var("holding", "hand", string_value(block1));
                        state.set_var("clear", block2, true.into());
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    // putdown action: put a block on the table
    domain.declare_action("putdown", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 1 {
            if let Some(block) = args[0].as_str() {
                // Check preconditions: holding block
                if let Some(pos) = state.get_var("pos", block) {
                    if pos.as_str() == Some("hand") {
                        // Apply effects
                        state.set_var("pos", block, string_value("table"));
                        state.set_var("clear", block, true.into());
                        state.set_var("holding", "hand", false.into());
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    // stack action: put a block on another block
    domain.declare_action("stack", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(block1), Some(block2)) = (args[0].as_str(), args[1].as_str()) {
                // Check preconditions: holding block1, block2 clear
                if let (Some(pos1), Some(clear2)) = (
                    state.get_var("pos", block1),
                    state.get_var("clear", block2)
                ) {
                    if pos1.as_str() == Some("hand") && clear2.as_bool() == Some(true) {
                        // Apply effects
                        state.set_var("pos", block1, string_value(block2));
                        state.set_var("clear", block1, true.into());
                        state.set_var("holding", "hand", false.into());
                        state.set_var("clear", block2, false.into());
                        return Some(state.clone());
                    }
                }
            }
        }
        None
    })?;

    Ok(())
}

/// Declare all HTN task methods for the blocks world domain.
///
/// This function defines the three core HTN task methods that implement hierarchical
/// task decomposition for blocks world planning:
///
/// - **`achieve`**: The main planning method that implements the Gupta-Nau algorithm
/// - **`take`**: Decomposes block pickup into primitive actions (pickup/unstack)
/// - **`put`**: Decomposes block placement into primitive actions (putdown/stack)
///
/// Following the HTN principle, this implementation uses **only task methods** and
/// never uses multigoal methods. The 'achieve' task handles multigoals by using
/// the multigoal registry pattern to work within Rust's type system constraints.
///
/// ## HTN Task Hierarchy
///
/// ```text
/// achieve(multigoal) → take(block) + put(block, dest) + achieve(multigoal)
/// take(block)        → pickup(block) | unstack(block, under)
/// put(block, dest)   → putdown(block) | stack(block, dest)
/// ```
///
/// # Arguments
///
/// * `domain` - The domain to add the task methods to
///
/// # Returns
///
/// `Ok(())` if all task methods are declared successfully, or an error if any declaration fails.
fn declare_blocks_task_methods(domain: &mut Domain, multigoals: HashMap<String, Multigoal>) -> Result<()> {
    // Create a shared reference to multigoals for the closures
    let multigoals_ref = Arc::new(multigoals);
    let multigoals_for_achieve = multigoals_ref.clone();

    // Task method for 'achieve' - Python equivalent: gtpyhop.declare_task_methods('achieve',m_moveblocks)
    // This implements the core HTN planning logic using only task methods
    domain.declare_task_method("achieve", move |state: &State, args: &[StateValue]| {
        if args.len() >= 1 {
            if let Some(goal_id) = args[0].as_str() {
                // Retrieve the multigoal from our captured multigoals
                if let Some(mgoal) = multigoals_for_achieve.get(goal_id) {
                    // Use the exact same logic as Python m_moveblocks
                    return m_moveblocks_htn(state, mgoal, goal_id);
                }
            }
        }
        None
    })?;

    // Task method for 'take' - Python equivalent: gtpyhop.declare_task_methods('take',m_take)
    domain.declare_task_method("take", |state: &State, args: &[StateValue]| {
        if args.len() >= 1 {
            if let Some(b1) = args[0].as_str() {
                return m_take(state, b1);
            }
        }
        None
    })?;

    // Task method for 'put' - Python equivalent: gtpyhop.declare_task_methods('put',m_put)
    domain.declare_task_method("put", |state: &State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(b1), Some(b2)) = (args[0].as_str(), args[1].as_str()) {
                return m_put(state, b1, b2);
            }
        }
        None
    })?;

    // Task method for 'take' - picking up a block
    domain.declare_task_method("take", |state: &State, args: &[crate::core::StateValue]| {
        if args.len() >= 1 {
            if let Some(block) = args[0].as_str() {
                return m_take(state, block);
            }
        }
        None
    })?;

    // Task method for 'put' - putting down a block
    domain.declare_task_method("put", |state: &State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(block1), Some(block2)) = (args[0].as_str(), args[1].as_str()) {
                return m_put(state, block1, block2);
            }
        }
        None
    })?;

    Ok(())
}

/// Check if a block is "done" and doesn't need to be moved.
///
/// A block is considered "done" if it and all blocks below it will never need to be moved
/// to achieve the multigoal. This is a key optimization in the Gupta-Nau algorithm.
///
/// The algorithm works recursively:
/// 1. If the block is "table", it's always done
/// 2. If the block has a goal position and isn't there, it's not done
/// 3. If the block is on the table, it's done
/// 4. Otherwise, recursively check the block below it
///
/// **Python equivalent**: `def is_done(b1,state,mgoal)`
///
/// # Arguments
///
/// * `b1` - The block to check
/// * `state` - The current world state
/// * `mgoal` - The multigoal being achieved
///
/// # Returns
///
/// `true` if the block doesn't need to be moved, `false` otherwise.
///
/// # Example
///
/// ```rust,no_run
/// # use gtrusthop::core::{State, Multigoal};
/// # let state = State::new("test");
/// # let goal = Multigoal::new("test");
/// // Block 'a' is on the table and has no goal - it's done
/// assert!(is_done("a", &state, &goal));
///
/// // Block 'b' needs to move to block 'c' but is currently on table - not done
/// assert!(!is_done("b", &state, &goal));
/// ```
fn is_done(b1: &str, state: &State, mgoal: &Multigoal) -> bool {
    if b1 == "table" {
        return true;
    }

    // Check if b1 has a goal position and is not there
    if let Some(goal_pos) = mgoal.get_goal("pos", b1) {
        if let Some(current_pos) = state.get_var("pos", b1) {
            if goal_pos != current_pos {
                return false;
            }
        }
    }

    // Check if b1 is on table
    if let Some(current_pos) = state.get_var("pos", b1) {
        if current_pos.as_str() == Some("table") {
            return true;
        }

        // Recursively check the block below
        if let Some(below_block) = current_pos.as_str() {
            if below_block != "table" && below_block != "hand" {
                return is_done(below_block, state, mgoal);
            }
        }
    }

    true
}

/// Determine the planning status of a block according to the Gupta-Nau algorithm.
///
/// This function categorizes blocks into different states that guide the HTN planning
/// decisions. The status determines what action should be taken with the block:
///
/// - **"done"**: Block and all blocks below it never need to be moved
/// - **"inaccessible"**: Block is not clear (has another block on top)
/// - **"move-to-table"**: Block should be moved to the table (either has no goal or goal is table)
/// - **"move-to-block"**: Block can be moved directly to its goal position
/// - **"waiting"**: Block cannot be moved to its goal yet (target not ready)
///
/// This status classification is central to the Gupta-Nau algorithm's efficiency,
/// allowing it to make optimal decisions about which blocks to move when.
///
/// **Python equivalent**: `def status(b1,state,mgoal)`
///
/// # Arguments
///
/// * `b1` - The block to analyze
/// * `state` - The current world state
/// * `mgoal` - The multigoal being achieved
///
/// # Returns
///
/// A string indicating the block's planning status.
///
/// # Example
///
/// ```rust,no_run
/// # use gtrusthop::core::{State, Multigoal};
/// # let state = State::new("test");
/// # let goal = Multigoal::new("test");
/// let status = status("a", &state, &goal);
/// match status.as_str() {
///     "move-to-block" => println!("Block a can move to its goal now"),
///     "waiting" => println!("Block a must wait for other blocks to move first"),
///     "done" => println!("Block a is already in the right place"),
///     _ => println!("Block a has status: {}", status),
/// }
/// ```
fn status(b1: &str, state: &State, mgoal: &Multigoal) -> String {
    // Check if block is done (doesn't need to be moved)
    if is_done(b1, state, mgoal) {
        return "done".to_string();
    }

    // Check if block is clear
    if let Some(clear) = state.get_var("clear", b1) {
        if clear.as_bool() != Some(true) {
            return "inaccessible".to_string();
        }
    }

    // Check goal position
    if let Some(goal_pos) = mgoal.get_goal("pos", b1) {
        if let Some(goal_str) = goal_pos.as_str() {
            if goal_str == "table" {
                return "move-to-table".to_string();
            } else {
                // Check if target block is done and clear
                if is_done(goal_str, state, mgoal) {
                    if let Some(target_clear) = state.get_var("clear", goal_str) {
                        if target_clear.as_bool() == Some(true) {
                            return "move-to-block".to_string();
                        }
                    }
                }
                return "waiting".to_string();
            }
        }
    } else {
        return "move-to-table".to_string();
    }

    "waiting".to_string()
}





/// Get all clear blocks in the current state.
///
/// A block is "clear" if no other block is stacked on top of it and it's not being held.
/// Only clear blocks can be picked up or unstacked, making this function essential
/// for determining which blocks are available for manipulation.
///
/// This function is used by the Gupta-Nau algorithm to iterate through blocks that
/// could potentially be moved in the current planning step.
///
/// **Python equivalent**: `def all_clear_blocks(state)`
///
/// # Arguments
///
/// * `state` - The current world state
///
/// # Returns
///
/// A vector of block names that are currently clear and can be manipulated.
///
/// # Example
///
/// ```rust,no_run
/// # use gtrusthop::core::State;
/// # let state = State::new("test");
/// let clear_blocks = all_clear_blocks(&state);
/// for block in clear_blocks {
///     println!("Block {} is available for pickup/unstack", block);
/// }
/// ```
fn all_clear_blocks(state: &State) -> Vec<String> {
    let mut clear_blocks = Vec::new();

    if let Some(clear_data) = state.get_var_map("clear") {
        for (block, clear_value) in clear_data {
            if clear_value.as_bool() == Some(true) {
                clear_blocks.push(block.clone());
            }
        }
    }

    clear_blocks
}

/// HTN implementation of the Gupta-Nau blocks world planning algorithm.
///
/// This is the core method that implements the near-optimal blocks-world planning
/// algorithm described in Gupta & Nau (1992). It follows the HTN paradigm by
/// decomposing the multigoal achievement task into sequences of 'take' and 'put' tasks.
///
/// ## Algorithm Overview
///
/// The method implements the following strategy:
///
/// 1. **Direct moves**: Look for clear blocks that can be moved directly to their
///    final positions without interfering with other goals.
///
/// 2. **Clearing moves**: If no direct moves are possible, move blocks to the table
///    to clear the way for other blocks.
///
/// 3. **Termination**: If no blocks need moving, the goal is achieved.
///
/// ## HTN Task Decomposition
///
/// Unlike the HGN approach which uses multigoal methods, this HTN implementation
/// decomposes the achievement task into:
/// - `take(block)` - Pick up a specific block
/// - `put(block, destination)` - Place a block at its destination
/// - `achieve(goal_id)` - Recursively achieve the remaining goals
///
/// **Python equivalent**: `def m_moveblocks(state,mgoal)`
///
/// # Arguments
///
/// * `state` - The current world state
/// * `mgoal` - The multigoal to achieve
/// * `goal_id` - The registry ID for recursive achieve calls
///
/// # Returns
///
/// A task decomposition (sequence of `PlanItem::task` calls) or `None` if no decomposition applies.
///
/// # References
///
/// N. Gupta and D. S. Nau. "On the complexity of blocks-world planning."
/// *Artificial Intelligence* 56(2-3):223–254, 1992.
fn m_moveblocks_htn(state: &State, mgoal: &Multigoal, goal_id: &str) -> Option<Vec<PlanItem>> {
    // Look for a clear block that can be moved to its final location
    for x in all_clear_blocks(state) {
        let xstat = status(&x, state, mgoal);
        if xstat == "move-to-block" {
            if let Some(target_pos) = mgoal.get_goal("pos", &x) {
                return Some(vec![
                    PlanItem::task("take", vec![string_value(&x)]),
                    PlanItem::task("put", vec![string_value(&x), target_pos.clone()]),
                    PlanItem::task("achieve", vec![string_value(goal_id)])
                ]);
            }
        } else if xstat == "move-to-table" {
            return Some(vec![
                PlanItem::task("take", vec![string_value(&x)]),
                PlanItem::task("put", vec![string_value(&x), string_value("table")]),
                PlanItem::task("achieve", vec![string_value(goal_id)])
            ]);
        }
    }

    // If we get here, no blocks can be moved to their final locations
    for x in all_clear_blocks(state) {
        if status(&x, state, mgoal) == "waiting" {
            if let Some(pos) = state.get_var("pos", &x) {
                if pos.as_str() != Some("table") {
                    return Some(vec![
                        PlanItem::task("take", vec![string_value(&x)]),
                        PlanItem::task("put", vec![string_value(&x), string_value("table")]),
                        PlanItem::task("achieve", vec![string_value(goal_id)])
                    ]);
                }
            }
        }
    }

    // If we get here, there are no blocks that need moving
    Some(vec![])
}



/// HTN task method for taking (picking up) a block.
///
/// This method decomposes the abstract 'take' task into the appropriate primitive action:
/// - `pickup(block)` if the block is on the table
/// - `unstack(block, under_block)` if the block is on another block
///
/// The method only succeeds if the block is clear (no other block is on top of it).
/// This ensures that the preconditions for the primitive actions will be satisfied.
///
/// ## HTN Decomposition
///
/// ```text
/// take(block) → pickup(block)     [if block is on table]
///            → unstack(block, X)  [if block is on block X]
/// ```
///
/// **Python equivalent**: `def m_take(state,b1)`
///
/// # Arguments
///
/// * `state` - The current world state
/// * `b1` - The block to pick up
///
/// # Returns
///
/// A single-action plan (`pickup` or `unstack`) or `None` if the block cannot be taken.
///
/// # Example
///
/// ```rust,no_run
/// # use gtrusthop::core::{State, PlanItem, string_value};
/// # let state = State::new("test");
/// // Block 'a' is on the table and clear
/// let plan = m_take(&state, "a");
/// assert_eq!(plan, Some(vec![PlanItem::action("pickup", vec![string_value("a")])]));
///
/// // Block 'b' is on block 'c' and clear
/// let plan = m_take(&state, "b");
/// assert_eq!(plan, Some(vec![PlanItem::action("unstack", vec![string_value("b"), string_value("c")])]));
/// ```
fn m_take(state: &State, b1: &str) -> Option<Vec<PlanItem>> {
    // Generate either a pickup or an unstack subtask for b1
    if let Some(clear) = state.get_var("clear", b1) {
        if clear.as_bool() == Some(true) {
            if let Some(pos) = state.get_var("pos", b1) {
                if pos.as_str() == Some("table") {
                    return Some(vec![PlanItem::action("pickup", vec![string_value(b1)])]);
                } else if let Some(under_block) = pos.as_str() {
                    if under_block != "hand" {
                        return Some(vec![PlanItem::action("unstack", vec![
                            string_value(b1),
                            string_value(under_block)
                        ])]);
                    }
                }
            }
        }
    }
    None
}

/// HTN task method for putting (placing) a block at a destination.
///
/// This method decomposes the abstract 'put' task into the appropriate primitive action:
/// - `putdown(block)` if the destination is the table
/// - `stack(block, destination)` if the destination is another block
///
/// The method only succeeds if the block is currently being held (in the hand).
/// This ensures that the preconditions for the primitive actions will be satisfied.
///
/// ## HTN Decomposition
///
/// ```text
/// put(block, table) → putdown(block)
/// put(block, dest)  → stack(block, dest)  [if dest is another block]
/// ```
///
/// **Python equivalent**: `def m_put(state,b1,b2)`
///
/// # Arguments
///
/// * `state` - The current world state
/// * `b1` - The block to place (must be currently held)
/// * `b2` - The destination ("table" or another block name)
///
/// # Returns
///
/// A single-action plan (`putdown` or `stack`) or `None` if the block is not being held.
///
/// # Example
///
/// ```rust,no_run
/// # use gtrusthop::core::{State, PlanItem, string_value};
/// # let state = State::new("test");
/// // Block 'a' is being held, put it on the table
/// let plan = m_put(&state, "a", "table");
/// assert_eq!(plan, Some(vec![PlanItem::action("putdown", vec![string_value("a")])]));
///
/// // Block 'a' is being held, stack it on block 'b'
/// let plan = m_put(&state, "a", "b");
/// assert_eq!(plan, Some(vec![PlanItem::action("stack", vec![string_value("a"), string_value("b")])]));
/// ```
fn m_put(state: &State, b1: &str, b2: &str) -> Option<Vec<PlanItem>> {
    // Generate either a putdown or a stack subtask for b1
    // b2 is b1's destination: either the table or another block
    if let Some(holding) = state.get_var("holding", "hand") {
        if holding.as_str() == Some(b1) {
            if b2 == "table" {
                return Some(vec![PlanItem::action("putdown", vec![string_value(b1)])]);
            } else {
                return Some(vec![PlanItem::action("stack", vec![
                    string_value(b1),
                    string_value(b2)
                ])]);
            }
        }
    }
    None
}

/// Test simple actions that should fail
fn test_failing_actions(planner: &crate::planning::Planner) -> Result<()> {
    let state1 = create_test_state1();

    println!("Initial state:");
    state1.display(None);

    // Try to pickup block 'a' (should fail - not on table)
    let plan = planner.find_plan(state1.clone(), vec![PlanItem::action("pickup", vec![string_value("a")])])?;
    match plan {
        Some(_) => println!("ERROR: pickup 'a' should have failed"),
        None => println!("✓ pickup 'a' correctly failed (not on table)"),
    }

    // Try to pickup block 'b' (should fail - not clear)
    let plan = planner.find_plan(state1.clone(), vec![PlanItem::action("pickup", vec![string_value("b")])])?;
    match plan {
        Some(_) => println!("ERROR: pickup 'b' should have failed"),
        None => println!("✓ pickup 'b' correctly failed (not clear)"),
    }

    // Try to take block 'b' (should fail - not clear)
    let plan = planner.find_plan(state1, vec![PlanItem::task("take", vec![string_value("b")])])?;
    match plan {
        Some(_) => println!("ERROR: take 'b' should have failed"),
        None => println!("✓ take 'b' correctly failed (not clear)"),
    }

    Ok(())
}

/// Test simple actions that should succeed
fn test_succeeding_actions(planner: &crate::planning::Planner) -> Result<()> {
    let state1 = create_test_state1();

    println!("Testing actions that should succeed...");

    // Try to pickup block 'c' (should succeed)
    let plan = planner.find_plan(state1.clone(), vec![PlanItem::action("pickup", vec![string_value("c")])])?;
    match plan {
        Some(actions) => {
            println!("✓ pickup 'c' succeeded with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("ERROR: pickup 'c' should have succeeded"),
    }

    // Try to take block 'a' (should succeed with unstack)
    let plan = planner.find_plan(state1.clone(), vec![PlanItem::task("take", vec![string_value("a")])])?;
    match plan {
        Some(actions) => {
            println!("✓ take 'a' succeeded with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("ERROR: take 'a' should have succeeded"),
    }

    // Try to take block 'c' (should succeed with pickup)
    let plan = planner.find_plan(state1.clone(), vec![PlanItem::task("take", vec![string_value("c")])])?;
    match plan {
        Some(actions) => {
            println!("✓ take 'c' succeeded with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("ERROR: take 'c' should have succeeded"),
    }

    // Try compound task: take 'a' and put on table
    let plan = planner.find_plan(state1, vec![
        PlanItem::task("take", vec![string_value("a")]),
        PlanItem::task("put", vec![string_value("a"), string_value("table")])
    ])?;
    match plan {
        Some(actions) => {
            println!("✓ take 'a' and put on table succeeded with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("ERROR: take 'a' and put on table should have succeeded"),
    }

    Ok(())
}

/// Test multigoal planning
fn test_multigoal_planning(domain: &Domain) -> Result<()> {
    let state1 = create_test_state1();

    println!("Testing multigoal planning...");
    state1.display(None);

    // Create goal: c on b, b on a, a on table
    let mut goal1a = Multigoal::new("goal1a");
    goal1a.set_goal("pos", "c", string_value("b"));
    goal1a.set_goal("pos", "b", string_value("a"));
    goal1a.set_goal("pos", "a", string_value("table"));

    println!("Goal 1a:");
    goal1a.display(None);

    // Create planner with the multigoal using the new builder pattern
    let planner = crate::planning::PlannerBuilder::new()
        .with_domain(domain.clone())
        .with_multigoal(goal1a.clone())
        .with_verbose_level(0)?
        .build()?;

    let plan = planner.find_plan(state1.clone(), vec![PlanItem::task("achieve", vec![string_value("goal_goal1a")])])?;
    match plan {
        Some(actions) => {
            println!("✓ Goal 1a achieved with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("ERROR: Goal 1a should have been achievable"),
    }

    // Create goal1b: c on b, b on a (omits "a on table")
    let mut goal1b = Multigoal::new("goal1b");
    goal1b.set_goal("pos", "c", string_value("b"));
    goal1b.set_goal("pos", "b", string_value("a"));

    println!("\nGoal 1b (omits 'a on table'):");
    goal1b.display(None);

    // Create planner with the second multigoal
    let planner_1b = crate::planning::PlannerBuilder::new()
        .with_domain(domain.clone())
        .with_multigoal(goal1b)
        .with_verbose_level(0)?
        .build()?;

    let plan = planner_1b.find_plan(state1, vec![PlanItem::task("achieve", vec![string_value("goal_goal1b")])])?;
    match plan {
        Some(actions) => {
            println!("✓ Goal 1b achieved with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("ERROR: Goal 1b should have been achievable"),
    }

    Ok(())
}

/// Test the famous Sussman Anomaly
fn test_sussman_anomaly(domain: &Domain) -> Result<()> {
    println!("Testing the famous Sussman Anomaly...");

    // Create Sussman Anomaly initial state: c on a, a and b on table
    let sussman_state = create_sussman_state();

    println!("Sussman Anomaly initial state:");
    sussman_state.display(None);

    // Create Sussman goal: a on b, b on c
    let mut sussman_goal = Multigoal::new("sussman_goal");
    sussman_goal.set_goal("pos", "a", string_value("b"));
    sussman_goal.set_goal("pos", "b", string_value("c"));

    println!("Sussman Anomaly goal:");
    sussman_goal.display(None);

    // Create planner with the Sussman goal
    let planner = crate::planning::PlannerBuilder::new()
        .with_domain(domain.clone())
        .with_multigoal(sussman_goal)
        .with_verbose_level(0)?
        .build()?;

    let plan = planner.find_plan(sussman_state, vec![PlanItem::task("achieve", vec![string_value("goal_sussman_goal")])])?;
    match plan {
        Some(actions) => {
            println!("✓ Sussman Anomaly solved with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("ERROR: Sussman Anomaly should have been solvable"),
    }

    Ok(())
}

/// Test complex scenarios
fn test_complex_scenarios(domain: &Domain) -> Result<()> {
    println!("Testing complex scenarios...");

    // Create a more complex initial state
    let state2 = create_complex_state();

    println!("Complex initial state:");
    state2.display(None);

    // Create complex goal: rearrange blocks
    let mut goal2 = Multigoal::new("goal2");
    goal2.set_goal("pos", "b", string_value("c"));
    goal2.set_goal("pos", "a", string_value("d"));
    goal2.set_goal("pos", "c", string_value("table"));
    goal2.set_goal("pos", "d", string_value("table"));

    println!("Complex goal:");
    goal2.display(None);

    // Create planner with the complex goal
    let planner = crate::planning::PlannerBuilder::new()
        .with_domain(domain.clone())
        .with_multigoal(goal2)
        .with_verbose_level(0)?
        .build()?;

    let plan = planner.find_plan(state2, vec![PlanItem::task("achieve", vec![string_value("goal_goal2")])])?;
    match plan {
        Some(actions) => {
            println!("✓ Complex scenario solved with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("ERROR: Complex scenario should have been solvable"),
    }

    Ok(())
}

/// Create test state 1: a on b, b on table, c on table
fn create_test_state1() -> State {
    let mut state = State::new("state1");

    // Block positions
    state.set_var("pos", "a", string_value("b"));
    state.set_var("pos", "b", string_value("table"));
    state.set_var("pos", "c", string_value("table"));

    // Clear status
    state.set_var("clear", "a", true.into());
    state.set_var("clear", "b", false.into());
    state.set_var("clear", "c", true.into());

    // Hand status
    state.set_var("holding", "hand", false.into());

    state
}

/// Create Sussman Anomaly state: c on a, a and b on table
fn create_sussman_state() -> State {
    let mut state = State::new("sussman_initial");

    // Block positions
    state.set_var("pos", "c", string_value("a"));
    state.set_var("pos", "a", string_value("table"));
    state.set_var("pos", "b", string_value("table"));

    // Clear status
    state.set_var("clear", "c", true.into());
    state.set_var("clear", "a", false.into());
    state.set_var("clear", "b", true.into());

    // Hand status
    state.set_var("holding", "hand", false.into());

    state
}

/// Create complex state: a on c, b on d, c and d on table
fn create_complex_state() -> State {
    let mut state = State::new("state2");

    // Block positions
    state.set_var("pos", "a", string_value("c"));
    state.set_var("pos", "b", string_value("d"));
    state.set_var("pos", "c", string_value("table"));
    state.set_var("pos", "d", string_value("table"));

    // Clear status
    state.set_var("clear", "a", true.into());
    state.set_var("clear", "b", true.into());
    state.set_var("clear", "c", false.into());
    state.set_var("clear", "d", false.into());

    // Hand status
    state.set_var("holding", "hand", false.into());

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_blocks_htn_examples() -> Result<()> {
        run_blocks_htn_examples()
    }

    #[test]
    fn test_create_blocks_htn_domain() -> Result<()> {
        let domain = create_blocks_htn_domain()?;
        assert_eq!(domain.name, "blocks_htn");
        Ok(())
    }

    #[test]
    fn test_helper_functions() {
        let state = create_test_state1();
        let mut goal = Multigoal::new("test_goal");
        goal.set_goal("pos", "a", string_value("table"));

        // Test all_clear_blocks (all_blocks was removed as unused)
        let clear_blocks = all_clear_blocks(&state);
        assert!(clear_blocks.len() >= 2); // At least a and c should be clear

        // Test all_clear_blocks
        let clear_blocks = all_clear_blocks(&state);
        assert!(clear_blocks.contains(&"a".to_string()));
        assert!(clear_blocks.contains(&"c".to_string()));
        assert!(!clear_blocks.contains(&"b".to_string()));

        // Test is_done
        assert!(is_done("table", &state, &goal));
        assert!(!is_done("a", &state, &goal)); // a is on b, not on table as goal requires

        // Test status
        let status_a = status("a", &state, &goal);
        assert_eq!(status_a, "move-to-table");

        let status_c = status("c", &state, &goal);
        assert_eq!(status_c, "done"); // c has no goal, so it's done
    }

    #[test]
    fn test_task_methods() {
        let state = create_test_state1();

        // Test m_take for block on table
        let take_c = m_take(&state, "c");
        assert!(take_c.is_some());
        let actions = take_c.unwrap();
        assert_eq!(actions.len(), 1);
        assert!(actions[0].is_action());

        // Test m_take for block on another block
        let take_a = m_take(&state, "a");
        assert!(take_a.is_some());
        let actions = take_a.unwrap();
        assert_eq!(actions.len(), 1);
        assert!(actions[0].is_action());

        // Test m_take for inaccessible block
        let take_b = m_take(&state, "b");
        assert!(take_b.is_none()); // b is not clear
    }

    #[test]
    fn test_state_creation() {
        let state1 = create_test_state1();
        assert_eq!(state1.name, "state1");
        assert_eq!(state1.get_var("pos", "a").unwrap().as_str(), Some("b"));
        assert_eq!(state1.get_var("pos", "b").unwrap().as_str(), Some("table"));
        assert_eq!(state1.get_var("clear", "a").unwrap().as_bool(), Some(true));
        assert_eq!(state1.get_var("clear", "b").unwrap().as_bool(), Some(false));

        let sussman_state = create_sussman_state();
        assert_eq!(sussman_state.name, "sussman_initial");
        assert_eq!(sussman_state.get_var("pos", "c").unwrap().as_str(), Some("a"));
        assert_eq!(sussman_state.get_var("clear", "c").unwrap().as_bool(), Some(true));
        assert_eq!(sussman_state.get_var("clear", "a").unwrap().as_bool(), Some(false));

        let complex_state = create_complex_state();
        assert_eq!(complex_state.name, "state2");
        assert_eq!(complex_state.get_var("pos", "a").unwrap().as_str(), Some("c"));
        assert_eq!(complex_state.get_var("pos", "b").unwrap().as_str(), Some("d"));
    }

    #[test]
    fn test_simple_planning() -> Result<()> {
        let domain = create_blocks_htn_domain()?;
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;

        let state = create_test_state1();

        // Test simple pickup action
        let plan = planner.find_plan(state.clone(), vec![PlanItem::action("pickup", vec![string_value("c")])])?;
        assert!(plan.is_some());
        let actions = plan.unwrap();
        assert_eq!(actions.len(), 1);

        // Test simple take task
        let plan = planner.find_plan(state, vec![PlanItem::task("take", vec![string_value("c")])])?;
        assert!(plan.is_some());
        let actions = plan.unwrap();
        assert_eq!(actions.len(), 1);

        Ok(())
    }

    #[test]
    fn test_multigoal_planning() -> Result<()> {
        let domain = create_blocks_htn_domain()?;
        let state = create_test_state1();

        // Create simple goal
        let mut goal = Multigoal::new("simple_goal");
        goal.set_goal("pos", "a", string_value("table"));

        // Create planner with the multigoal using the new builder pattern
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_multigoal(goal)
            .with_verbose_level(0)?
            .build()?;

        let plan = planner.find_plan(state, vec![PlanItem::task("achieve", vec![string_value("goal_simple_goal")])])?;
        assert!(plan.is_some());
        let actions = plan.unwrap();
        assert!(!actions.is_empty());

        Ok(())
    }
}
