//! Planning algorithms for GTRusthop

pub mod planner;
pub mod planner_instance;
pub mod strategy;
pub mod verification;


pub use strategy::{PlanningStrategy, set_planning_strategy, get_planning_strategy};

use crate::core::{State, Domain, PlanItem, TodoList, Plan, StateValue, Multigoal};
use crate::error::{GTRustHopError, Result};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Global verbose level for debugging output
static VERBOSE_LEVEL: Mutex<i32> = Mutex::new(1);

/// Set the verbosity level for planning output
/// - level = 0: print nothing
/// - level = 1: print the initial parameters and the answer
/// - level = 2: also print a message on each recursive call
/// - level = 3: also print some info about intermediate computations
pub fn set_verbose_level(level: i32) -> Result<()> {
    if !(0..=3).contains(&level) {
        return Err(GTRustHopError::InvalidVerboseLevel { level });
    }
    
    let mut verbose = VERBOSE_LEVEL.lock().unwrap();
    *verbose = level;
    println!("Verbose level set to {level}.");
    Ok(())
}

/// Get the current verbosity level
pub fn get_verbose_level() -> i32 {
    *VERBOSE_LEVEL.lock().unwrap()
}

/// Check if verbose output is enabled at the given level
pub fn is_verbose(level: i32) -> bool {
    get_verbose_level() >= level
}

/// Print a message if verbose level is sufficient
pub fn verbose_print(level: i32, message: &str) {
    if is_verbose(level) {
        println!("{message}");
    }
}

/// Pyhop compatibility function for backward compatibility with original Pyhop planner
///
/// This function exists to provide backward compatibility with the original Pyhop planner.
/// It creates a default planner and calls `find_plan()` with a deprecation message.
///
/// **Note**: This function is provided for compatibility but is not recommended for new code.
/// For better control and thread safety, create a `Planner` instance using `PlannerBuilder`
/// and call `find_plan()` directly.
///
/// # Arguments
///
/// * `domain` - The planning domain containing actions and methods
/// * `state` - The initial state
/// * `todo_list` - List of tasks, goals, and actions to achieve
///
/// # Returns
///
/// The same result as `find_plan()`: `Ok(Some(plan))` if successful,
/// `Ok(None)` if no plan found, or `Err` if an error occurred.
///
/// # Example
///
/// ```rust,no_run
/// use gtrusthop::planning::pyhop;
/// use gtrusthop::core::{State, PlanItem, string_value, Domain};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let domain = Domain::new("test");
/// let mut state = State::new("initial");
/// state.set_var("loc", "alice", string_value("home"));
///
/// let todo_list = vec![
///     PlanItem::task("travel", vec![
///         string_value("alice"),
///         string_value("home"),
///         string_value("park")
///     ])
/// ];
///
/// // Old Pyhop-style call (deprecated)
/// let plan = pyhop(domain, state, todo_list)?;
/// # Ok(())
/// # }
/// ```
pub fn pyhop(domain: Domain, state: State, todo_list: Vec<PlanItem>) -> Result<Option<Plan>> {
    use crate::planning::PlannerBuilder;

    let verbose_level = get_verbose_level();
    if verbose_level > 0 {
        println!();
        println!("        >> The function 'pyhop' exists to provide backward compatibility");
        println!("        >> with Pyhop. In the future, please use PlannerBuilder and find_plan instead.");
    }

    // Create a default planner with current verbose level
    let planner = PlannerBuilder::new()
        .with_domain(domain)
        .with_verbose_level(verbose_level)?
        .build()?;

    planner.find_plan(state, todo_list)
}

/// Print a formatted message if verbose level is sufficient
pub fn verbose_printf(level: i32, _format: &str, args: std::fmt::Arguments) {
    if is_verbose(level) {
        println!("{args}");
    }
}

/// Convert a plan item to a string representation for debugging
pub fn item_to_string(item: &PlanItem) -> String {
    match item {
        PlanItem::Task(name, args) => {
            let args_str: Vec<String> = args.iter().map(value_to_string).collect();
            format!("({})", [name.clone()].into_iter().chain(args_str).collect::<Vec<_>>().join(" "))
        }
        PlanItem::Action(name, args) => {
            let args_str: Vec<String> = args.iter().map(value_to_string).collect();
            format!("({})", [name.clone()].into_iter().chain(args_str).collect::<Vec<_>>().join(" "))
        }
        PlanItem::Unigoal(var_name, arg, value) => {
            format!("({} {} {})", var_name, arg, value_to_string(value))
        }
        PlanItem::Multigoal(mg) => format!("{mg}"),
    }
}

/// Convert a StateValue to a string without quotes for strings
fn value_to_string(value: &StateValue) -> String {
    match value {
        StateValue::String(s) => s.clone(),
        StateValue::Number(n) => n.to_string(),
        StateValue::Bool(b) => b.to_string(),
        StateValue::Null => "null".to_string(),
        StateValue::Array(arr) => format!("[{}]", arr.iter().map(value_to_string).collect::<Vec<_>>().join(", ")),
        StateValue::Object(obj) => format!("{{{}}}", obj.iter().map(|(k, v)| format!("{}: {}", k, value_to_string(v))).collect::<Vec<_>>().join(", ")),
    }
}

/// Convert a todo list to a string representation for debugging
pub fn todo_list_to_string(todo_list: &TodoList) -> String {
    let items: Vec<String> = todo_list.iter().map(item_to_string).collect();
    format!("[{}]", items.join(", "))
}

/// Planning context that holds the current domain and other global state
#[derive(Debug, Clone)]
pub struct PlanningContext {
    /// Current domain being used for planning
    pub domain: Arc<Domain>,
    /// Whether to verify goals after method application
    pub verify_goals: bool,
    /// Current planning strategy
    pub strategy: PlanningStrategy,
}

impl PlanningContext {
    /// Create a new planning context
    pub fn new(domain: Arc<Domain>) -> Self {
        Self {
            domain,
            verify_goals: true,
            strategy: PlanningStrategy::Iterative,
        }
    }

    /// Set whether to verify goals
    pub fn set_verify_goals(&mut self, verify: bool) {
        self.verify_goals = verify;
    }

    /// Set the planning strategy
    pub fn set_strategy(&mut self, strategy: PlanningStrategy) {
        self.strategy = strategy;
    }
}

/// Global planning context
static PLANNING_CONTEXT: Mutex<Option<PlanningContext>> = Mutex::new(None);

/// Set the current planning context
pub fn set_planning_context(context: PlanningContext) {
    let mut ctx = PLANNING_CONTEXT.lock().unwrap();
    *ctx = Some(context);
}

/// Get the current planning context
pub fn get_planning_context() -> Result<PlanningContext> {
    let ctx = PLANNING_CONTEXT.lock().unwrap();
    ctx.clone().ok_or_else(|| GTRustHopError::generic("No planning context has been set"))
}



/// Create a planner from a domain using the new builder pattern
///
/// This is a convenience function that creates a planner with default settings.
/// For more control over configuration, use `PlannerBuilder` directly.
pub fn create_planner(domain: Domain) -> Planner {
    Planner::new(domain)
}

/// Create a planner builder for fluent configuration
///
/// This is equivalent to `PlannerBuilder::new()` but may be more discoverable.
pub fn planner() -> PlannerBuilder {
    PlannerBuilder::new()
}

/// Result of a planning step
#[derive(Debug, Clone)]
pub enum PlanningResult {
    /// Planning succeeded with the given plan
    Success(Plan),
    /// Planning failed
    Failure,
    /// Planning needs to continue with new state
    Continue {
        state: State,
        todo_list: TodoList,
        plan: Plan,
        depth: usize,
    },
}

/// Trait for planning strategies
pub trait PlanningStrategyTrait {
    /// Seek a plan using this strategy
    fn seek_plan(
        &self,
        context: &PlanningContext,
        state: State,
        todo_list: TodoList,
        plan: Plan,
        depth: usize,
    ) -> Result<PlanningResult>;
}

/// Builder for creating isolated planner instances
///
/// This builder provides a fluent interface for configuring planning parameters
/// and creates immutable planner instances that are thread-safe and isolated.
#[derive(Debug, Clone)]
pub struct PlannerBuilder {
    domain: Option<Domain>,
    verbose_level: i32,
    strategy: PlanningStrategy,
    verify_goals: bool,
    multigoals: HashMap<String, Multigoal>,
}

impl Default for PlannerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PlannerBuilder {
    /// Create a new planner builder with default settings
    pub fn new() -> Self {
        Self {
            domain: None,
            verbose_level: 1,
            strategy: PlanningStrategy::Iterative,
            verify_goals: true,
            multigoals: HashMap::new(),
        }
    }

    /// Set the domain for planning
    pub fn with_domain(mut self, domain: Domain) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Set the verbosity level for planning output
    /// - level = 0: print nothing
    /// - level = 1: print the initial parameters and the answer
    /// - level = 2: also print a message on each recursive call
    /// - level = 3: also print the intermediate values
    pub fn with_verbose_level(mut self, level: i32) -> Result<Self> {
        if !(0..=3).contains(&level) {
            return Err(GTRustHopError::InvalidVerboseLevel { level });
        }
        self.verbose_level = level;
        Ok(self)
    }

    /// Set the planning strategy
    pub fn with_strategy(mut self, strategy: PlanningStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set whether to verify goals after method application
    pub fn with_goal_verification(mut self, verify: bool) -> Self {
        self.verify_goals = verify;
        self
    }

    /// Register a multigoal with the planner
    ///
    /// This replaces the global `register_multigoal()` function by storing
    /// multigoals as instance data in the planner.
    ///
    /// # Arguments
    ///
    /// * `multigoal` - The multigoal to register
    ///
    /// # Returns
    ///
    /// The builder with the multigoal registered
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use gtrusthop::core::{Multigoal, string_value, Domain};
    /// use gtrusthop::planning::PlannerBuilder;
    ///
    /// let mut goal = Multigoal::new("my_goal");
    /// goal.set_goal("pos", "a", string_value("table"));
    ///
    /// let domain = Domain::new("test");
    /// let planner = PlannerBuilder::new()
    ///     .with_domain(domain)
    ///     .with_multigoal(goal)
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_multigoal(mut self, multigoal: Multigoal) -> Self {
        let goal_id = format!("goal_{}", multigoal.name);
        self.multigoals.insert(goal_id, multigoal);
        self
    }

    /// Register multiple multigoals at once
    ///
    /// # Arguments
    ///
    /// * `multigoals` - Vector of multigoals to register
    ///
    /// # Returns
    ///
    /// The builder with all multigoals registered
    pub fn with_multigoals(mut self, multigoals: Vec<Multigoal>) -> Self {
        for multigoal in multigoals {
            let goal_id = format!("goal_{}", multigoal.name);
            self.multigoals.insert(goal_id, multigoal);
        }
        self
    }

    /// Build the planner instance
    pub fn build(self) -> Result<Planner> {
        let mut domain = self.domain.ok_or_else(||
            GTRustHopError::generic("Domain is required for planner")
        )?;

        // If this is a blocks domain and we have multigoals, create a new domain with multigoals baked in
        if domain.name == "blocks_htn" && !self.multigoals.is_empty() {
            domain = crate::examples::blocks_htn_example::create_blocks_htn_domain_with_multigoals(self.multigoals.clone())?;
        }

        Ok(Planner {
            domain: Arc::new(domain),
            verbose_level: self.verbose_level,
            strategy: self.strategy,
            verify_goals: self.verify_goals,
            multigoals: Arc::new(self.multigoals),
        })
    }
}

/// Immutable planner instance with isolated state
///
/// This planner is thread-safe and contains no global state.
/// Each instance is completely isolated from others.
#[derive(Debug, Clone)]
pub struct Planner {
    domain: Arc<Domain>,
    verbose_level: i32,
    strategy: PlanningStrategy,
    verify_goals: bool,
    multigoals: Arc<HashMap<String, Multigoal>>,
}

impl Planner {
    /// Create a new planner with the given domain and default settings
    pub fn new(domain: Domain) -> Self {
        Self {
            domain: Arc::new(domain),
            verbose_level: 1,
            strategy: PlanningStrategy::Iterative,
            verify_goals: true,
            multigoals: Arc::new(HashMap::new()),
        }
    }

    /// Get the domain used by this planner
    pub fn domain(&self) -> &Arc<Domain> {
        &self.domain
    }

    /// Get the current verbose level
    pub fn verbose_level(&self) -> i32 {
        self.verbose_level
    }

    /// Get the current planning strategy
    pub fn strategy(&self) -> PlanningStrategy {
        self.strategy
    }

    /// Check if goal verification is enabled
    pub fn verify_goals(&self) -> bool {
        self.verify_goals
    }

    /// Get a multigoal by its ID
    ///
    /// This replaces the global `get_multigoal()` function by accessing
    /// multigoals stored as instance data in the planner.
    ///
    /// # Arguments
    ///
    /// * `goal_id` - The unique ID of the multigoal (e.g., "goal_my_goal")
    ///
    /// # Returns
    ///
    /// The multigoal if found, or `None` if the ID is not registered
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use gtrusthop::planning::PlannerBuilder;
    /// # use gtrusthop::core::{Domain, Multigoal};
    /// # let domain = Domain::new("test");
    /// # let goal = Multigoal::new("my_goal");
    /// # let planner = PlannerBuilder::new().with_domain(domain).with_multigoal(goal).build().unwrap();
    /// if let Some(multigoal) = planner.get_multigoal("goal_my_goal") {
    ///     // Use the multigoal
    /// }
    /// ```
    pub fn get_multigoal(&self, goal_id: &str) -> Option<&Multigoal> {
        self.multigoals.get(goal_id)
    }

    /// Register a new multigoal with this planner instance
    ///
    /// Returns a new planner instance with the additional multigoal.
    /// This follows the immutable builder pattern.
    ///
    /// # Arguments
    ///
    /// * `multigoal` - The multigoal to register
    ///
    /// # Returns
    ///
    /// A new planner instance with the multigoal registered
    pub fn with_multigoal(self, multigoal: Multigoal) -> Self {
        let goal_id = format!("goal_{}", multigoal.name);
        let mut new_multigoals = (*self.multigoals).clone();
        new_multigoals.insert(goal_id, multigoal);

        Self {
            domain: self.domain,
            verbose_level: self.verbose_level,
            strategy: self.strategy,
            verify_goals: self.verify_goals,
            multigoals: Arc::new(new_multigoals),
        }
    }

    /// Create a new planner with different verbose level
    pub fn with_verbose_level(&self, level: i32) -> Result<Self> {
        if !(0..=3).contains(&level) {
            return Err(GTRustHopError::InvalidVerboseLevel { level });
        }
        Ok(Self {
            domain: Arc::clone(&self.domain),
            verbose_level: level,
            strategy: self.strategy,
            verify_goals: self.verify_goals,
            multigoals: Arc::clone(&self.multigoals),
        })
    }

    /// Create a new planner with different strategy
    pub fn with_strategy(&self, strategy: PlanningStrategy) -> Self {
        Self {
            domain: Arc::clone(&self.domain),
            verbose_level: self.verbose_level,
            strategy,
            verify_goals: self.verify_goals,
            multigoals: Arc::clone(&self.multigoals),
        }
    }

    /// Create a new planner with different goal verification setting
    pub fn with_goal_verification(&self, verify: bool) -> Self {
        Self {
            domain: Arc::clone(&self.domain),
            verbose_level: self.verbose_level,
            strategy: self.strategy,
            verify_goals: verify,
            multigoals: Arc::clone(&self.multigoals),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verbose_level() {
        assert!(set_verbose_level(2).is_ok());
        assert_eq!(get_verbose_level(), 2);
        assert!(is_verbose(1));
        assert!(is_verbose(2));
        assert!(!is_verbose(3));

        assert!(set_verbose_level(4).is_err());
        assert!(set_verbose_level(-1).is_err());
    }

    #[test]
    fn test_item_to_string() {
        let task = PlanItem::task("travel", vec!["alice".into(), "home".into(), "park".into()]);
        assert_eq!(item_to_string(&task), "(travel alice home park)");

        let action = PlanItem::action("move", vec!["obj1".into(), "loc2".into()]);
        assert_eq!(item_to_string(&action), "(move obj1 loc2)");

        let unigoal = PlanItem::unigoal("loc", "alice", crate::core::string_value("park"));
        assert_eq!(item_to_string(&unigoal), "(loc alice park)");
    }

    #[test]
    fn test_planning_context() {
        let domain = Domain::new("test_domain");
        let context = PlanningContext::new(Arc::new(domain));
        
        assert_eq!(context.domain.name, "test_domain");
        assert!(context.verify_goals);
        assert_eq!(context.strategy, PlanningStrategy::Iterative);
    }
}
