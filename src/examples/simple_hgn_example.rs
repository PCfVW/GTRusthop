//! Simple HGN (Hierarchical Goal Network) Example for GTRusthop
//!
//! This example demonstrates HGN planning concepts including:
//! - Goal methods for hierarchical goal decomposition
//! - Logistics domain with trucks, planes, and packages
//! - Blocks world domain with hierarchical block stacking
//! - Multigoal planning and goal satisfaction

use crate::core::{State, Domain, PlanItem, Multigoal, string_value};
use crate::planning::PlannerBuilder;
use crate::error::Result;

/// Run simple HGN examples
pub fn run_simple_hgn_examples() -> Result<()> {
    println!("=== Running Simple HGN Examples ===");

    // Run logistics HGN example
    run_logistics_hgn_example()?;

    println!("\n{}", "=".repeat(60));

    // Run blocks HGN example
    run_blocks_hgn_example()?;

    println!("=== Simple HGN Examples Completed ===");
    Ok(())
}

/// Run logistics HGN example
fn run_logistics_hgn_example() -> Result<()> {
    println!("--- Logistics HGN Example ---");

    let domain = create_logistics_hgn_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Create initial state
    let state = create_logistics_initial_state();

    println!("\nInitial state:");
    state.display(None);

    // Goal 1: Transport within the same city
    println!("\n--- Goal 1: Transport within the same city ---");
    let goals1 = vec![
        PlanItem::unigoal("at", "package1", string_value("location2")),
        PlanItem::unigoal("at", "package2", string_value("location3"))
    ];

    let plan1 = planner.find_plan(state.clone(), goals1)?;
    match plan1 {
        Some(actions) => {
            println!("Found plan with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("No plan found"),
    }

    // Goal 2: Transport to a different city
    println!("\n--- Goal 2: Transport to a different city ---");
    let goals2 = vec![
        PlanItem::unigoal("at", "package1", string_value("location10"))
    ];

    let plan2 = planner.find_plan(state.clone(), goals2)?;
    match plan2 {
        Some(actions) => {
            println!("Found plan with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("No plan found"),
    }

    // Goal 3: Already satisfied goal
    println!("\n--- Goal 3: Already satisfied goal ---");
    let goals3 = vec![
        PlanItem::unigoal("at", "package1", string_value("location1"))
    ];

    let plan3 = planner.find_plan(state, goals3)?;
    match plan3 {
        Some(actions) => {
            if actions.is_empty() {
                println!("Goal already satisfied - empty plan returned");
            } else {
                println!("Found plan with {} actions:", actions.len());
                for (i, action) in actions.iter().enumerate() {
                    println!("  {}: {:?}", i + 1, action);
                }
            }
        }
        None => println!("No plan found"),
    }

    Ok(())
}

/// Run blocks HGN example
fn run_blocks_hgn_example() -> Result<()> {
    println!("--- Blocks HGN Example ---");

    let domain = create_blocks_hgn_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(1)?
        .build()?;

    // Test simple block operations
    println!("\n--- Simple Block Operations ---");
    let state1 = create_blocks_initial_state();

    println!("Initial state:");
    state1.display(None);

    // Test pickup action
    let pickup_goals = vec![PlanItem::unigoal("pos", "c", string_value("hand"))];
    let pickup_plan = planner.find_plan(state1.clone(), pickup_goals)?;

    match pickup_plan {
        Some(actions) => {
            println!("Plan to pick up block c:");
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("No plan found for pickup"),
    }

    // Test multigoal planning - Sussman anomaly
    println!("\n--- Sussman Anomaly ---");
    let mut sussman_state = State::new("sussman_initial");
    sussman_state.set_var("pos", "a", string_value("table"));
    sussman_state.set_var("pos", "b", string_value("table"));
    sussman_state.set_var("pos", "c", string_value("a"));
    sussman_state.set_var("clear", "a", false.into());
    sussman_state.set_var("clear", "b", true.into());
    sussman_state.set_var("clear", "c", true.into());
    sussman_state.set_var("holding", "hand", false.into());

    println!("Sussman initial state:");
    sussman_state.display(None);

    // Create multigoal for Sussman anomaly: a on b, b on c
    let mut sussman_goal = Multigoal::new("sussman_goal");
    sussman_goal.set_goal("pos", "a", string_value("b"));
    sussman_goal.set_goal("pos", "b", string_value("c"));

    println!("Sussman goal:");
    sussman_goal.display(None);

    let sussman_plan = planner.find_plan(sussman_state, vec![PlanItem::Multigoal(sussman_goal)])?;

    match sussman_plan {
        Some(actions) => {
            println!("Sussman anomaly solution with {} actions:", actions.len());
            for (i, action) in actions.iter().enumerate() {
                println!("  {}: {:?}", i + 1, action);
            }
        }
        None => println!("No plan found for Sussman anomaly"),
    }

    Ok(())
}

/// Create logistics HGN domain
fn create_logistics_hgn_domain() -> Result<Domain> {
    let mut domain = Domain::new("logistics_hgn");

    // Actions for logistics domain
    domain.declare_action("drive_truck", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(truck), Some(location)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("truck_at", truck, string_value(location));
                return Some(state.clone());
            }
        }
        None
    })?;

    domain.declare_action("load_truck", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(obj), Some(truck)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("at", obj, string_value(truck));
                return Some(state.clone());
            }
        }
        None
    })?;

    domain.declare_action("unload_truck", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(obj), Some(location)) = (args[0].as_str(), args[1].as_str()) {
                // Check if truck is at the location
                if let Some(truck_name) = state.get_var("at", obj) {
                    if let Some(truck_str) = truck_name.as_str() {
                        if let Some(truck_location) = state.get_var("truck_at", truck_str) {
                            if truck_location.as_str() == Some(location) {
                                state.set_var("at", obj, string_value(location));
                                return Some(state.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    })?;

    domain.declare_action("fly_plane", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(plane), Some(airport)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("plane_at", plane, string_value(airport));
                return Some(state.clone());
            }
        }
        None
    })?;

    domain.declare_action("load_plane", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(obj), Some(plane)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("at", obj, string_value(plane));
                return Some(state.clone());
            }
        }
        None
    })?;

    domain.declare_action("unload_plane", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(obj), Some(airport)) = (args[0].as_str(), args[1].as_str()) {
                // Check if plane is at the airport
                if let Some(plane_name) = state.get_var("at", obj) {
                    if let Some(plane_str) = plane_name.as_str() {
                        if let Some(plane_location) = state.get_var("plane_at", plane_str) {
                            if plane_location.as_str() == Some(airport) {
                                state.set_var("at", obj, string_value(airport));
                                return Some(state.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    })?;

    // Goal methods for logistics domain
    domain.declare_unigoal_method("at", |state: &State, obj: &str, target_value: &crate::core::StateValue| {
        if let Some(target_location) = target_value.as_str() {
            // Check if already at target
            if let Some(current_location) = state.get_var("at", obj) {
                if current_location.as_str() == Some(target_location) {
                    return Some(vec![]); // Already satisfied
                }

                // Try to find a truck in the same city
                if let Some(current_loc_str) = current_location.as_str() {
                    if let Some(current_city) = state.get_var("in_city", current_loc_str) {
                        if let Some(target_city) = state.get_var("in_city", target_location) {

                            // Same city - use truck
                            if current_city == target_city {
                                if let Some(truck) = find_truck_in_city(state, current_city.as_str().unwrap_or("")) {
                                    return Some(vec![
                                        PlanItem::unigoal("truck_at", &truck, current_location.clone()),
                                        PlanItem::action("load_truck", vec![string_value(obj), string_value(&truck)]),
                                        PlanItem::unigoal("truck_at", &truck, string_value(target_location)),
                                        PlanItem::action("unload_truck", vec![string_value(obj), string_value(target_location)])
                                    ]);
                                }
                            }
                            // Different cities - use plane
                            else {
                                if let Some(airport1) = find_airport_in_city(state, current_city.as_str().unwrap_or("")) {
                                    if let Some(airport2) = find_airport_in_city(state, target_city.as_str().unwrap_or("")) {
                                        if let Some(plane) = find_plane(state) {
                                            return Some(vec![
                                                PlanItem::unigoal("at", obj, string_value(&airport1)),
                                                PlanItem::unigoal("plane_at", &plane, string_value(&airport1)),
                                                PlanItem::action("load_plane", vec![string_value(obj), string_value(&plane)]),
                                                PlanItem::unigoal("plane_at", &plane, string_value(&airport2)),
                                                PlanItem::action("unload_plane", vec![string_value(obj), string_value(&airport2)]),
                                                PlanItem::unigoal("at", obj, string_value(target_location))
                                            ]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    })?;

    domain.declare_unigoal_method("truck_at", |state: &State, truck: &str, target_value: &crate::core::StateValue| {
        if let Some(target_location) = target_value.as_str() {
            // Check if already at target
            if let Some(current_location) = state.get_var("truck_at", truck) {
                if current_location.as_str() == Some(target_location) {
                    return Some(vec![]); // Already satisfied
                }

                // Check if in same city
                if let Some(current_loc_str) = current_location.as_str() {
                    if let Some(current_city) = state.get_var("in_city", current_loc_str) {
                        if let Some(target_city) = state.get_var("in_city", target_location) {
                            if current_city == target_city {
                                return Some(vec![
                                    PlanItem::action("drive_truck", vec![string_value(truck), string_value(target_location)])
                                ]);
                            }
                        }
                    }
                }
            }
        }
        None
    })?;

    domain.declare_unigoal_method("plane_at", |state: &State, plane: &str, target_value: &crate::core::StateValue| {
        if let Some(target_airport) = target_value.as_str() {
            // Check if already at target
            if let Some(current_airport) = state.get_var("plane_at", plane) {
                if current_airport.as_str() == Some(target_airport) {
                    return Some(vec![]); // Already satisfied
                }

                return Some(vec![
                    PlanItem::action("fly_plane", vec![string_value(plane), string_value(target_airport)])
                ]);
            }
        }
        None
    })?;

    Ok(domain)
}

/// Helper function to find a truck in a given city
fn find_truck_in_city(state: &State, city: &str) -> Option<String> {
    // Look for trucks in the state
    if let Some(trucks_data) = state.get_var_map("truck_at") {
        for (truck, location_value) in trucks_data {
            if let Some(location) = location_value.as_str() {
                if let Some(truck_city) = state.get_var("in_city", location) {
                    if truck_city.as_str() == Some(city) {
                        return Some(truck.clone());
                    }
                }
            }
        }
    }
    None
}

/// Helper function to find an airport in a given city
fn find_airport_in_city(state: &State, city: &str) -> Option<String> {
    // Look for airports in the state
    if let Some(city_data) = state.get_var_map("in_city") {
        for (location, city_value) in city_data {
            if city_value.as_str() == Some(city) {
                // Check if this location is an airport
                if location.starts_with("airport") {
                    return Some(location.clone());
                }
            }
        }
    }
    None
}

/// Helper function to find any available plane
fn find_plane(state: &State) -> Option<String> {
    // Look for planes in the state
    if let Some(planes_data) = state.get_var_map("plane_at") {
        for (plane, _) in planes_data {
            return Some(plane.clone());
        }
    }
    None
}

/// Create initial state for logistics domain
fn create_logistics_initial_state() -> State {
    let mut state = State::new("logistics_initial");

    // Packages
    state.set_var("at", "package1", string_value("location1"));
    state.set_var("at", "package2", string_value("location2"));

    // Trucks
    state.set_var("truck_at", "truck1", string_value("location3"));
    state.set_var("truck_at", "truck6", string_value("location10"));

    // Planes
    state.set_var("plane_at", "plane2", string_value("airport2"));

    // City mappings
    state.set_var("in_city", "location1", string_value("city1"));
    state.set_var("in_city", "location2", string_value("city1"));
    state.set_var("in_city", "location3", string_value("city1"));
    state.set_var("in_city", "airport1", string_value("city1"));
    state.set_var("in_city", "location10", string_value("city2"));
    state.set_var("in_city", "airport2", string_value("city2"));

    state
}

/// Create blocks HGN domain
fn create_blocks_hgn_domain() -> Result<Domain> {
    let mut domain = Domain::new("blocks_hgn");

    // Actions for blocks domain
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
                       clear.as_bool() == Some(true) &&
                       holding.as_bool() == Some(false) &&
                       block2 != "table" {
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

    domain.declare_action("putdown", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 1 {
            if let Some(block) = args[0].as_str() {
                // Check preconditions: holding block
                if let Some(pos) = state.get_var("pos", block) {
                    if pos.as_str() == Some("hand") {
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

    domain.declare_action("stack", |state: &mut State, args: &[crate::core::StateValue]| {
        if args.len() >= 2 {
            if let (Some(block1), Some(block2)) = (args[0].as_str(), args[1].as_str()) {
                // Check preconditions: holding block1, block2 clear
                if let (Some(pos1), Some(clear2)) = (
                    state.get_var("pos", block1),
                    state.get_var("clear", block2)
                ) {
                    if pos1.as_str() == Some("hand") && clear2.as_bool() == Some(true) {
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

    // Goal methods for blocks domain
    domain.declare_unigoal_method("pos", |state: &State, block: &str, target_value: &crate::core::StateValue| {
        if let Some(target) = target_value.as_str() {
            // Check if already satisfied
            if let Some(current_pos) = state.get_var("pos", block) {
                if current_pos.as_str() == Some(target) {
                    return Some(vec![]); // Already satisfied
                }

                // If target is "hand", pick up the block
                if target == "hand" {
                    if let Some(clear) = state.get_var("clear", block) {
                        if clear.as_bool() == Some(true) {
                            if let Some(holding) = state.get_var("holding", "hand") {
                                if holding.as_bool() == Some(false) {
                                    // Check if on table or on another block
                                    if current_pos.as_str() == Some("table") {
                                        return Some(vec![
                                            PlanItem::action("pickup", vec![string_value(block)])
                                        ]);
                                    } else if let Some(under_block) = current_pos.as_str() {
                                        if under_block != "table" && under_block != "hand" {
                                            return Some(vec![
                                                PlanItem::action("unstack", vec![string_value(block), string_value(under_block)])
                                            ]);
                                        }
                                    }
                                }
                            }
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
                        // Stack on another block
                        if let Some(target_clear) = state.get_var("clear", target) {
                            if target_clear.as_bool() == Some(true) {
                                return Some(vec![
                                    PlanItem::action("stack", vec![string_value(block), string_value(target)])
                                ]);
                            }
                        }
                    }
                }
            }
        }
        None
    })?;

    // Multigoal method for blocks domain
    domain.declare_multigoal_method(|state: &State, mgoal: &Multigoal| {
        // Find a clear block that can be moved to its final location
        let clear_blocks = get_clear_blocks(state);

        for block in &clear_blocks {
            let block_status = get_block_status(state, mgoal, block);

            match block_status.as_str() {
                "move-to-block" => {
                    if let Some(target) = mgoal.get_goal("pos", block) {
                        return Some(vec![
                            PlanItem::unigoal("pos", block, string_value("hand")),
                            PlanItem::unigoal("pos", block, target.clone()),
                            PlanItem::Multigoal(mgoal.clone())
                        ]);
                    }
                }
                "move-to-table" => {
                    return Some(vec![
                        PlanItem::unigoal("pos", block, string_value("hand")),
                        PlanItem::unigoal("pos", block, string_value("table")),
                        PlanItem::Multigoal(mgoal.clone())
                    ]);
                }
                _ => continue,
            }
        }

        // Look for blocks that need to be moved out of the way
        for block in &clear_blocks {
            let block_status = get_block_status(state, mgoal, block);
            if block_status == "waiting" {
                if let Some(current_pos) = state.get_var("pos", block) {
                    if current_pos.as_str() != Some("table") {
                        return Some(vec![
                            PlanItem::unigoal("pos", block, string_value("hand")),
                            PlanItem::unigoal("pos", block, string_value("table")),
                            PlanItem::Multigoal(mgoal.clone())
                        ]);
                    }
                }
            }
        }

        // No blocks need moving
        Some(vec![])
    })?;

    Ok(domain)
}

/// Get all clear blocks in the state
fn get_clear_blocks(state: &State) -> Vec<String> {
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

/// Determine the status of a block for planning purposes
fn get_block_status(state: &State, mgoal: &Multigoal, block: &str) -> String {
    // Check if block is done (doesn't need to be moved)
    if is_block_done(state, mgoal, block) {
        return "done".to_string();
    }

    // Check if block is clear
    if let Some(clear) = state.get_var("clear", block) {
        if clear.as_bool() != Some(true) {
            return "inaccessible".to_string();
        }
    }

    // Check goal position
    if let Some(goal_pos) = mgoal.get_goal("pos", block) {
        if let Some(goal_str) = goal_pos.as_str() {
            if goal_str == "table" {
                return "move-to-table".to_string();
            } else {
                // Check if target block is done and clear
                if is_block_done(state, mgoal, goal_str) {
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

/// Check if a block is done (doesn't need to be moved)
fn is_block_done(state: &State, mgoal: &Multigoal, block: &str) -> bool {
    if block == "table" {
        return true;
    }

    // Check if block has a goal position and is not there
    if let Some(goal_pos) = mgoal.get_goal("pos", block) {
        if let Some(current_pos) = state.get_var("pos", block) {
            if goal_pos != current_pos {
                return false;
            }
        }
    }

    // Check if block is on table
    if let Some(current_pos) = state.get_var("pos", block) {
        if current_pos.as_str() == Some("table") {
            return true;
        }

        // Recursively check the block below
        if let Some(below_block) = current_pos.as_str() {
            if below_block != "table" && below_block != "hand" {
                return is_block_done(state, mgoal, below_block);
            }
        }
    }

    true
}

/// Create initial state for blocks domain
fn create_blocks_initial_state() -> State {
    let mut state = State::new("blocks_initial");

    // Block positions: a on b, b on table, c on table
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple_hgn_examples() -> Result<()> {
        run_simple_hgn_examples()
    }

    #[test]
    fn test_logistics_hgn_domain() -> Result<()> {
        let domain = create_logistics_hgn_domain()?;
        assert_eq!(domain.name, "logistics_hgn");
        Ok(())
    }

    #[test]
    fn test_blocks_hgn_domain() -> Result<()> {
        let domain = create_blocks_hgn_domain()?;
        assert_eq!(domain.name, "blocks_hgn");
        Ok(())
    }

    #[test]
    fn test_logistics_initial_state() {
        let state = create_logistics_initial_state();
        assert_eq!(state.name, "logistics_initial");

        // Check some initial conditions
        assert_eq!(state.get_var("at", "package1").unwrap().as_str(), Some("location1"));
        assert_eq!(state.get_var("truck_at", "truck1").unwrap().as_str(), Some("location3"));
    }

    #[test]
    fn test_blocks_initial_state() {
        let state = create_blocks_initial_state();
        assert_eq!(state.name, "blocks_initial");

        // Check initial block configuration
        assert_eq!(state.get_var("pos", "a").unwrap().as_str(), Some("b"));
        assert_eq!(state.get_var("pos", "b").unwrap().as_str(), Some("table"));
        assert_eq!(state.get_var("clear", "c").unwrap().as_bool(), Some(true));
    }
}
