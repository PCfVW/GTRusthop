# GTRusthop Documentation

Welcome to the comprehensive documentation for GTRusthop, a Goal-Task-Network planning library written in Rust.

## Documentation Structure

This documentation is organized to serve different learning paths and use cases:

### 🚀 **Getting Started**
Start here if you're new to GTRusthop or HTN planning:

1. **[Main README](../README.md)** - Project overview, quick start, and installation
2. **[Examples and Tutorials](examples.md)** - Step-by-step learning guide from basics to advanced concepts

### 📚 **For Developers**
Technical references for building with GTRusthop:

1. **[API Reference](api.md)** - Complete technical documentation with function signatures, error handling, and integration guidelines
2. **[Working Examples](../src/examples/)** - Functional code examples in the source tree

## Learning Paths

### Path 1: Complete Beginner
**Goal**: Learn HTN planning concepts and GTRusthop basics

1. Read [Examples and Tutorials](examples.md) sections 1-3
2. Try the practice exercises in section 7
3. Explore [Working Examples](../src/examples/)
4. Reference [API Documentation](api.md) as needed

**Time Investment**: 4-6 hours

### Path 2: Experienced Developer
**Goal**: Quickly integrate GTRusthop into existing projects

1. Skim [Main README](../README.md) for overview
2. Study [API Reference](api.md) sections 1-2 and 7
3. Review [Working Examples](../src/examples/) for patterns
4. Use [Examples and Tutorials](examples.md) section 6 for troubleshooting

**Time Investment**: 1-2 hours

### Path 3: Python GTPyhop User
**Goal**: Migrate existing Python code to Rust

1. Study [API Reference](api.md) sections 1-5 for equivalent functions
2. Work through [Examples and Tutorials](examples.md) sections 2-4 for Rust-specific patterns
3. Use [Working Examples](../src/examples/) as reference implementations
4. Compare with the [Rust Migration Guide](Rust.md) for architectural differences

**Time Investment**: 3-4 hours

### Path 4: HTN Planning Researcher
**Goal**: Understand implementation details and extend the library

1. Study [API Reference](api.md) sections 6-7 for performance and integration details
2. Examine source code in conjunction with [Working Examples](../src/examples/)
3. Try advanced exercises in [Examples and Tutorials](examples.md) section 7
4. Review [Rust Migration Guide](Rust.md) for architectural insights

**Time Investment**: 6-8 hours

## Quick Reference

### Essential Functions
```rust
// Core imports
use gtrusthop::{Domain, State, PlanItem, PlannerBuilder, PlanningStrategy};
use gtrusthop::core::string_value;

// Modern Builder Pattern API (Recommended)
let mut domain = Domain::new("my_domain");
// ... declare actions and methods ...

let planner = PlannerBuilder::new()
    .with_domain(domain)
    .with_strategy(PlanningStrategy::Iterative)
    .with_verbose_level(1)?
    .build()?;

let state = State::new("initial");
// ... set up initial state ...

let goals = vec![PlanItem::task("my_task", vec![])];
let plan = planner.find_plan(state, goals)?;
```

### Cross-Platform Development Setup

**Windows Setup:**
```powershell
# Install Rust (if not already installed)
winget install Rustlang.Rustup
# Or download from https://rustup.rs/

# Create new project
cargo new my_planning_project
cd my_planning_project

# Add GTRusthop
cargo add gtrusthop

# Build and run
cargo build
cargo run

# Development tools
cargo install cargo-watch  # Auto-rebuild on changes
cargo install cargo-edit   # Enhanced cargo commands
```

**Linux/macOS Setup:**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Create new project
cargo new my_planning_project
cd my_planning_project

# Add GTRusthop
cargo add gtrusthop

# Build and run
cargo build
cargo run

# Development tools
cargo install cargo-watch  # Auto-rebuild on changes
cargo install cargo-edit   # Enhanced cargo commands
```

### Common Patterns
- **Action Declaration**: See [API Reference](api.md#domain-management)
- **Task Methods**: See [Examples and Tutorials](examples.md#building-complex-behaviors)
- **Error Handling**: See [API Reference](api.md#error-handling)
- **Performance Optimization**: See [API Reference](api.md#performance-considerations)

## Getting Help

### Documentation Issues
- **Missing Information**: Check if it's covered in another document using the cross-references above
- **Unclear Examples**: The [Examples and Tutorials](examples.md) provides multiple approaches to the same concepts
- **Technical Details**: The [API Reference](api.md) has comprehensive technical specifications

### Code Issues
- **Compilation Errors**: Check the [Rust Migration Guide](Rust.md) for language-specific differences
- **Runtime Errors**: Use the error handling patterns in [API Reference](api.md#error-handling)
- **Performance Problems**: See optimization strategies in [API Reference](api.md#performance-considerations)

### Planning Issues
- **Domain Design**: Work through [Examples and Tutorials](examples.md) sections 2-4
- **Complex Behaviors**: Study the advanced techniques in [Examples and Tutorials](examples.md) section 5
- **Debugging Plans**: Use the debugging strategies in [Examples and Tutorials](examples.md) section 6

## Documentation Design

This documentation is designed to be:
- **Comprehensive**: Covers all aspects from beginner to advanced
- **Cross-Referenced**: Each document links to related information
- **Example-Driven**: Every concept includes working code
- **Pedagogically Sound**: Builds understanding progressively

## Version Information

This documentation corresponds to GTRusthop version 1.2.1. For the latest updates and version-specific changes, see the main [README](../README.md) and [CHANGELOG](../CHANGELOG.md).

---

**Happy Planning!** 🤖🎯
