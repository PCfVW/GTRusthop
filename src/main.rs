//! Main executable for GTRusthop examples

use gtrusthop::examples::run_simple_htn_examples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GTRusthop - Goal-Task-Network Planning in Rust");
    println!("==============================================");
    
    // Run simple HTN examples
    run_simple_htn_examples()?;
    
    Ok(())
}
