//! Regression tests for GTRusthop
//!
//! This module runs comprehensive regression tests to ensure all functionality
//! works correctly across different planning paradigms and domains.

use crate::error::Result;
use super::{
    run_simple_htn_examples,
    run_simple_hgn_examples,
    run_blocks_htn_examples,
    run_lazy_lookahead_examples,
    backtracking_htn_example::run_backtracking_htn_examples,
    logistics_hgn_example::run_logistics_hgn_examples
};

/// Run all regression tests
pub fn run_regression_tests() -> Result<()> {
    println!("=== Running Comprehensive Regression Tests ===");

    // HTN Examples
    println!("\n--- HTN Planning Examples ---");
    run_simple_htn_examples()?;
    run_blocks_htn_examples()?;
    run_backtracking_htn_examples()?;

    // HGN Examples
    println!("\n--- HGN Planning Examples ---");
    run_simple_hgn_examples()?;
    run_logistics_hgn_examples()?;

    // Mixed Examples
    println!("\n--- Mixed Planning Examples ---");
    run_lazy_lookahead_examples()?;

    println!("\n=== All Regression Tests Completed Successfully! ===");
    Ok(())
}

/// Run specific domain regression tests
pub fn run_domain_regression_tests() -> Result<()> {
    println!("=== Running Domain-Specific Regression Tests ===");

    // Test HTN domains
    test_htn_domains()?;

    // Test HGN domains
    test_hgn_domains()?;

    println!("=== Domain Regression Tests Completed Successfully! ===");
    Ok(())
}

/// Test HTN-specific functionality
fn test_htn_domains() -> Result<()> {
    println!("\nTesting HTN domains...");

    // Test simple HTN domain
    crate::domains::create_simple_htn_domain()?;

    // Test blocks HTN domain
    super::blocks_htn_example::create_blocks_htn_domain()?;

    // Test backtracking HTN domain
    super::backtracking_htn_example::create_backtracking_htn_domain()?;

    println!("✓ All HTN domains created successfully");
    Ok(())
}

/// Test HGN-specific functionality
fn test_hgn_domains() -> Result<()> {
    println!("\nTesting HGN domains...");

    // Test simple HGN domain
    crate::domains::create_simple_hgn_domain()?;

    // Test logistics HGN domain
    super::logistics_hgn_example::create_logistics_hgn_domain()?;

    println!("✓ All HGN domains created successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_regression_tests() {
        assert!(run_regression_tests().is_ok());
    }

    #[test]
    fn test_run_domain_regression_tests() {
        assert!(run_domain_regression_tests().is_ok());
    }

    #[test]
    fn test_htn_domains_wrapper() -> Result<()> {
        test_htn_domains()
    }

    #[test]
    fn test_hgn_domains_wrapper() -> Result<()> {
        test_hgn_domains()
    }

    #[test]
    fn test_individual_examples() -> Result<()> {
        // Test each example individually to ensure isolation
        use crate::examples::*;
        run_simple_htn_examples()?;
        run_blocks_htn_examples()?;
        run_backtracking_htn_examples()?;
        run_simple_hgn_examples()?;
        run_logistics_hgn_examples()?;
        run_lazy_lookahead_examples()?;
        Ok(())
    }
}
