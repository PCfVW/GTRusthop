//! # Logistics HGN Example for GTRusthop
//! 
//! This example demonstrates HGN (Hierarchical Goal Network) planning for a logistics domain,
//! showing how to transport packages using trucks and planes between different locations and cities.
//! 
//! This is a faithful Rust port of the Python `logistics_hgn.py` example from GTPyhop,
//! maintaining the HGN principle of using **only multigoal methods**.
//! 
//! ## Key Concepts Demonstrated
//! 
//! - **HGN Planning**: Goal-oriented planning using multigoal methods
//! - **Complex Domain**: Multi-modal transportation with trucks and planes
//! - **Hierarchical Goals**: Breaking down transportation goals into subgoals
//! - **Spatial Reasoning**: Intra-city vs inter-city transportation
//! 
//! ## Domain Description
//! 
//! The logistics domain includes:
//! - **Packages**: Items to be transported
//! - **Trucks**: For intra-city transportation
//! - **Planes**: For inter-city transportation
//! - **Locations**: Places within cities (including airports)
//! - **Cities**: Geographic regions containing locations
//! 
//! ## Transportation Modes
//! 
//! 1. **Intra-city**: Use trucks to move packages within the same city
//! 2. **Inter-city**: Use planes to move packages between different cities
//! 3. **Mixed**: Combine truck and plane transportation as needed
//! 
//! **Python equivalent**: `logistics_hgn.py`

use crate::core::{State, Domain, PlanItem, Multigoal, string_value, StateValue};
use crate::planning::PlannerBuilder;
use crate::error::Result;


/// Run logistics HGN examples demonstrating HGN planning capabilities
pub fn run_logistics_hgn_examples() -> Result<()> {
    println!("=== Running Logistics HGN Examples ===");
    
    // Create the logistics HGN domain
    let domain = create_logistics_hgn_domain()?;
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(3)? // High verbosity to see goal decomposition
        .build()?;
    
    // Create initial state
    let state1 = create_logistics_state();
    
    println!("\nInitial state:");
    state1.display(None);
    
    // Test 1: Transport within the same city
    println!("\n----------");
    println!("Goal 1: package1 is at location2; package2 is at location3 (transport within the same city)");
    println!("----------");
    
    let mut goal1 = Multigoal::new("goal1");
    goal1.set_goal("at", "package1", string_value("location2"));
    goal1.set_goal("at", "package2", string_value("location3"));
    
    let todo_list = vec![PlanItem::multigoal(goal1)];
    let result = planner.find_plan(state1.clone(), todo_list)?;
    print_result(&result, "Goal 1");
    
    // Test 2: Transport to a different city
    println!("\n----------");
    println!("Goal 2: package1 is at location10 (transport to a different city)");
    println!("----------");
    
    let mut goal2 = Multigoal::new("goal2");
    goal2.set_goal("at", "package1", string_value("location10"));
    
    let todo_list = vec![PlanItem::multigoal(goal2)];
    let result = planner.find_plan(state1.clone(), todo_list)?;
    print_result(&result, "Goal 2");
    
    // Test 3: No actions needed
    println!("\n----------");
    println!("Goal 3: package1 is at location1 (no actions needed)");
    println!("----------");
    
    let mut goal3 = Multigoal::new("goal3");
    goal3.set_goal("at", "package1", string_value("location1"));
    
    let todo_list = vec![PlanItem::multigoal(goal3)];
    let result = planner.find_plan(state1, todo_list)?;
    print_result(&result, "Goal 3");
    
    println!("\n=== Logistics HGN Examples Completed ===");
    Ok(())
}

/// Create the logistics HGN domain with actions and unigoal methods
pub fn create_logistics_hgn_domain() -> Result<Domain> {
    let mut domain = Domain::new("logistics_hgn");
    
    // Declare actions
    declare_logistics_actions(&mut domain)?;
    
    // Declare unigoal methods (HGN approach)
    declare_logistics_unigoal_methods(&mut domain)?;
    
    Ok(domain)
}

/// Declare actions for the logistics domain
fn declare_logistics_actions(domain: &mut Domain) -> Result<()> {
    // Drive truck action
    domain.declare_action("drive_truck", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(truck), Some(location)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("truck_at", truck, string_value(location));
                return Some(state.clone());
            }
        }
        None
    })?;
    
    // Load truck action
    domain.declare_action("load_truck", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(package), Some(truck)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("at", package, string_value(truck));
                return Some(state.clone());
            }
        }
        None
    })?;
    
    // Unload truck action
    domain.declare_action("unload_truck", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(package), Some(location)) = (args[0].as_str(), args[1].as_str()) {
                // Check if package is on a truck and truck is at the location
                if let Some(truck_val) = state.get_var("at", package) {
                    if let Some(truck) = truck_val.as_str() {
                        if let Some(truck_loc_val) = state.get_var("truck_at", truck) {
                            if let Some(truck_loc) = truck_loc_val.as_str() {
                                if truck_loc == location {
                                    state.set_var("at", package, string_value(location));
                                    return Some(state.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    })?;
    
    // Fly plane action
    domain.declare_action("fly_plane", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(plane), Some(airport)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("plane_at", plane, string_value(airport));
                return Some(state.clone());
            }
        }
        None
    })?;
    
    // Load plane action
    domain.declare_action("load_plane", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(package), Some(plane)) = (args[0].as_str(), args[1].as_str()) {
                state.set_var("at", package, string_value(plane));
                return Some(state.clone());
            }
        }
        None
    })?;
    
    // Unload plane action
    domain.declare_action("unload_plane", |state: &mut State, args: &[StateValue]| {
        if args.len() >= 2 {
            if let (Some(package), Some(airport)) = (args[0].as_str(), args[1].as_str()) {
                // Check if package is on a plane and plane is at the airport
                if let Some(plane_val) = state.get_var("at", package) {
                    if let Some(plane) = plane_val.as_str() {
                        if let Some(plane_loc_val) = state.get_var("plane_at", plane) {
                            if let Some(plane_loc) = plane_loc_val.as_str() {
                                if plane_loc == airport {
                                    state.set_var("at", package, string_value(airport));
                                    return Some(state.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    })?;
    
    Ok(())
}

/// Declare unigoal methods for the logistics domain (HGN approach)
fn declare_logistics_unigoal_methods(domain: &mut Domain) -> Result<()> {
    // Unigoal methods for 'at' (package location)
    
    // Method: Load truck
    domain.declare_unigoal_method("at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(truck) = value.as_str() {
            // Check if this is a truck and package is at truck location
            if is_truck(state, truck) {
                if let Some(package_loc_val) = state.get_var("at", arg) {
                    if let Some(package_loc) = package_loc_val.as_str() {
                        if let Some(truck_loc_val) = state.get_var("truck_at", truck) {
                            if let Some(truck_loc) = truck_loc_val.as_str() {
                                if package_loc == truck_loc {
                                    return Some(vec![PlanItem::action("load_truck", vec![string_value(arg), string_value(truck)])]);
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    })?;
    
    // Method: Unload truck
    domain.declare_unigoal_method("at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(location) = value.as_str() {
            // Check if package is on a truck and we want to unload at a location
            if is_location(state, location) {
                if let Some(truck_val) = state.get_var("at", arg) {
                    if let Some(truck) = truck_val.as_str() {
                        if is_truck(state, truck) {
                            return Some(vec![PlanItem::action("unload_truck", vec![string_value(arg), string_value(location)])]);
                        }
                    }
                }
            }
        }
        None
    })?;
    
    // Method: Load plane
    domain.declare_unigoal_method("at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(plane) = value.as_str() {
            // Check if this is a plane and package is at plane location
            if is_plane(state, plane) {
                if let Some(package_loc_val) = state.get_var("at", arg) {
                    if let Some(package_loc) = package_loc_val.as_str() {
                        if let Some(plane_loc_val) = state.get_var("plane_at", plane) {
                            if let Some(plane_loc) = plane_loc_val.as_str() {
                                if package_loc == plane_loc {
                                    return Some(vec![PlanItem::action("load_plane", vec![string_value(arg), string_value(plane)])]);
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    })?;
    
    // Method: Unload plane
    domain.declare_unigoal_method("at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(airport) = value.as_str() {
            // Check if package is on a plane and we want to unload at an airport
            if is_airport(state, airport) {
                if let Some(plane_val) = state.get_var("at", arg) {
                    if let Some(plane) = plane_val.as_str() {
                        if is_plane(state, plane) {
                            return Some(vec![PlanItem::action("unload_plane", vec![string_value(arg), string_value(airport)])]);
                        }
                    }
                }
            }
        }
        None
    })?;
    
    // Unigoal methods for 'truck_at'
    domain.declare_unigoal_method("truck_at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(location) = value.as_str() {
            if is_truck(state, arg) && is_location(state, location) {
                // Check if truck and location are in the same city
                if let (Some(truck_city), Some(loc_city)) = (get_city(state, arg), get_city(state, location)) {
                    if truck_city == loc_city {
                        return Some(vec![PlanItem::action("drive_truck", vec![string_value(arg), string_value(location)])]);
                    }
                }
            }
        }
        None
    })?;
    
    // Unigoal methods for 'plane_at'
    domain.declare_unigoal_method("plane_at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(airport) = value.as_str() {
            if is_plane(state, arg) && is_airport(state, airport) {
                return Some(vec![PlanItem::action("fly_plane", vec![string_value(arg), string_value(airport)])]);
            }
        }
        None
    })?;

    // Complex unigoal methods for package transportation

    // Method: Move within city using truck
    domain.declare_unigoal_method("at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(target_location) = value.as_str() {
            if is_package(state, arg) && is_location(state, target_location) {
                if let Some(package_loc_val) = state.get_var("at", arg) {
                    if let Some(package_loc) = package_loc_val.as_str() {
                        if is_location(state, package_loc) {
                            // Check if both locations are in the same city
                            if let (Some(pkg_city), Some(target_city)) = (get_city(state, package_loc), get_city(state, target_location)) {
                                if pkg_city == target_city {
                                    // Find a truck in the same city
                                    if let Some(truck) = find_truck_in_city(state, &pkg_city) {
                                        return Some(vec![
                                            PlanItem::unigoal("truck_at".to_string(), truck.clone(), string_value(package_loc)),
                                            PlanItem::unigoal("at".to_string(), arg.to_string(), string_value(&truck)),
                                            PlanItem::unigoal("truck_at".to_string(), truck, string_value(target_location)),
                                            PlanItem::unigoal("at".to_string(), arg.to_string(), string_value(target_location))
                                        ]);
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

    // Method: Move between airports using plane
    domain.declare_unigoal_method("at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(target_airport) = value.as_str() {
            if is_package(state, arg) && is_airport(state, target_airport) {
                if let Some(package_loc_val) = state.get_var("at", arg) {
                    if let Some(package_loc) = package_loc_val.as_str() {
                        if is_airport(state, package_loc) {
                            // Check if airports are in different cities
                            if let (Some(pkg_city), Some(target_city)) = (get_city(state, package_loc), get_city(state, target_airport)) {
                                if pkg_city != target_city {
                                    // Find a plane
                                    if let Some(plane) = find_plane_in_city(state, &pkg_city) {
                                        return Some(vec![
                                            PlanItem::unigoal("plane_at".to_string(), plane.clone(), string_value(package_loc)),
                                            PlanItem::unigoal("at".to_string(), arg.to_string(), string_value(&plane)),
                                            PlanItem::unigoal("plane_at".to_string(), plane, string_value(target_airport)),
                                            PlanItem::unigoal("at".to_string(), arg.to_string(), string_value(target_airport))
                                        ]);
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

    // Method: Move between cities (location to location)
    domain.declare_unigoal_method("at", |state: &State, arg: &str, value: &StateValue| {
        if let Some(target_location) = value.as_str() {
            if is_package(state, arg) && is_location(state, target_location) {
                if let Some(package_loc_val) = state.get_var("at", arg) {
                    if let Some(package_loc) = package_loc_val.as_str() {
                        if is_location(state, package_loc) {
                            // Check if locations are in different cities
                            if let (Some(pkg_city), Some(target_city)) = (get_city(state, package_loc), get_city(state, target_location)) {
                                if pkg_city != target_city {
                                    // Find airports in both cities
                                    if let (Some(source_airport), Some(target_airport)) = (find_airport_in_city(state, &pkg_city), find_airport_in_city(state, &target_city)) {
                                        return Some(vec![
                                            PlanItem::unigoal("at".to_string(), arg.to_string(), string_value(&source_airport)),
                                            PlanItem::unigoal("at".to_string(), arg.to_string(), string_value(&target_airport)),
                                            PlanItem::unigoal("at".to_string(), arg.to_string(), string_value(target_location))
                                        ]);
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

    Ok(())
}

/// Create the initial logistics state
fn create_logistics_state() -> State {
    let mut state = State::new("state1");
    
    // Set up packages
    state.set_var("at", "package1", string_value("location1"));
    state.set_var("at", "package2", string_value("location2"));
    
    // Set up trucks
    state.set_var("truck_at", "truck1", string_value("location3"));
    state.set_var("truck_at", "truck6", string_value("location10"));
    
    // Set up planes
    state.set_var("plane_at", "plane2", string_value("airport2"));
    
    // Set up city mappings
    state.set_var("in_city", "location1", string_value("city1"));
    state.set_var("in_city", "location2", string_value("city1"));
    state.set_var("in_city", "location3", string_value("city1"));
    state.set_var("in_city", "airport1", string_value("city1"));
    state.set_var("in_city", "location10", string_value("city2"));
    state.set_var("in_city", "airport2", string_value("city2"));
    
    // Set up entity types
    state.set_var("packages", "package1", true.into());
    state.set_var("packages", "package2", true.into());
    state.set_var("trucks", "truck1", true.into());
    state.set_var("trucks", "truck6", true.into());
    state.set_var("airplanes", "plane2", true.into());
    state.set_var("locations", "location1", true.into());
    state.set_var("locations", "location2", true.into());
    state.set_var("locations", "location3", true.into());
    state.set_var("locations", "airport1", true.into());
    state.set_var("locations", "location10", true.into());
    state.set_var("locations", "airport2", true.into());
    state.set_var("airports", "airport1", true.into());
    state.set_var("airports", "airport2", true.into());
    state.set_var("cities", "city1", true.into());
    state.set_var("cities", "city2", true.into());
    
    state
}

/// Helper functions for domain logic

fn is_package(state: &State, entity: &str) -> bool {
    state.get_var("packages", entity).map_or(false, |v| v.as_bool().unwrap_or(false))
}

fn is_truck(state: &State, entity: &str) -> bool {
    state.get_var("trucks", entity).map_or(false, |v| v.as_bool().unwrap_or(false))
}

fn is_plane(state: &State, entity: &str) -> bool {
    state.get_var("airplanes", entity).map_or(false, |v| v.as_bool().unwrap_or(false))
}

fn is_location(state: &State, entity: &str) -> bool {
    state.get_var("locations", entity).map_or(false, |v| v.as_bool().unwrap_or(false))
}

fn is_airport(state: &State, entity: &str) -> bool {
    state.get_var("airports", entity).map_or(false, |v| v.as_bool().unwrap_or(false))
}

fn get_city(state: &State, entity: &str) -> Option<String> {
    state.get_var("in_city", entity)?.as_str().map(|s| s.to_string())
}

/// Find a truck in the same city as the given city
fn find_truck_in_city(state: &State, city: &str) -> Option<String> {
    // Check all trucks
    for truck in ["truck1", "truck6"] {
        if is_truck(state, truck) {
            if let Some(truck_loc_val) = state.get_var("truck_at", truck) {
                if let Some(truck_loc) = truck_loc_val.as_str() {
                    if let Some(truck_city) = get_city(state, truck_loc) {
                        if truck_city == city {
                            return Some(truck.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Find a plane in the same city as the given city, or any plane if none available
fn find_plane_in_city(state: &State, city: &str) -> Option<String> {
    // First try to find a plane in the same city
    for plane in ["plane2"] {
        if is_plane(state, plane) {
            if let Some(plane_loc_val) = state.get_var("plane_at", plane) {
                if let Some(plane_loc) = plane_loc_val.as_str() {
                    if let Some(plane_city) = get_city(state, plane_loc) {
                        if plane_city == city {
                            return Some(plane.to_string());
                        }
                    }
                }
            }
        }
    }
    // If no plane in the same city, return any plane
    for plane in ["plane2"] {
        if is_plane(state, plane) {
            return Some(plane.to_string());
        }
    }
    None
}

/// Find an airport in the same city as the given city
fn find_airport_in_city(state: &State, city: &str) -> Option<String> {
    for airport in ["airport1", "airport2"] {
        if is_airport(state, airport) {
            if let Some(airport_city) = get_city(state, airport) {
                if airport_city == city {
                    return Some(airport.to_string());
                }
            }
        }
    }
    None
}

/// Print the result of a planning attempt
fn print_result(result: &Option<Vec<PlanItem>>, test_name: &str) {
    match result {
        Some(plan) => {
            println!("✓ {} succeeded with {} actions:", test_name, plan.len());
            for (i, action) in plan.iter().enumerate() {
                println!("  {}: {}", i + 1, action);
            }
        }
        None => {
            println!("✗ {} failed: No plan found", test_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_logistics_hgn_examples() -> Result<()> {
        run_logistics_hgn_examples()
    }
    
    #[test]
    fn test_create_logistics_hgn_domain() -> Result<()> {
        let domain = create_logistics_hgn_domain()?;
        assert_eq!(domain.name, "logistics_hgn");
        Ok(())
    }
    
    #[test]
    fn test_logistics_state_creation() -> Result<()> {
        let state = create_logistics_state();
        
        // Verify initial package locations
        assert_eq!(state.get_var("at", "package1").unwrap().as_str().unwrap(), "location1");
        assert_eq!(state.get_var("at", "package2").unwrap().as_str().unwrap(), "location2");
        
        // Verify truck locations
        assert_eq!(state.get_var("truck_at", "truck1").unwrap().as_str().unwrap(), "location3");
        assert_eq!(state.get_var("truck_at", "truck6").unwrap().as_str().unwrap(), "location10");
        
        Ok(())
    }
    
    #[test]
    fn test_helper_functions() -> Result<()> {
        let state = create_logistics_state();
        
        assert!(is_truck(&state, "truck1"));
        assert!(!is_truck(&state, "package1"));
        
        assert!(is_plane(&state, "plane2"));
        assert!(!is_plane(&state, "truck1"));
        
        assert!(is_location(&state, "location1"));
        assert!(!is_location(&state, "truck1"));
        
        assert!(is_airport(&state, "airport1"));
        assert!(!is_airport(&state, "location1"));
        
        assert_eq!(get_city(&state, "location1"), Some("city1".to_string()));
        assert_eq!(get_city(&state, "location10"), Some("city2".to_string()));
        
        Ok(())
    }
    
    #[test]
    fn test_simple_logistics_planning() -> Result<()> {
        let domain = create_logistics_hgn_domain()?;
        let planner = PlannerBuilder::new()
            .with_domain(domain)
            .with_verbose_level(0)?
            .build()?;
        
        let state = create_logistics_state();
        
        // Test goal that should require no actions (package1 already at location1)
        let mut goal = Multigoal::new("no_action_goal");
        goal.set_goal("at", "package1", string_value("location1"));
        
        let todo_list = vec![PlanItem::multigoal(goal)];
        let plan = planner.find_plan(state, todo_list)?;
        
        // Should succeed with empty plan since goal is already satisfied
        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.len(), 0);
        
        Ok(())
    }
}
