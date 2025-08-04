//! # GTRusthop Planning Strategy Performance Benchmark
//!
//! This benchmark compares the performance of iterative vs recursive planning strategies
//! across different problem sizes using the blocks world domain.
//!
//! ## Test Scenarios
//! - **Tiny**: 3 blocks (simple stacking)
//! - **Small**: 5 blocks (moderate complexity)  
//! - **Medium**: 8 blocks (challenging scenarios)
//! - **Large**: 12 blocks (complex multi-tower problems)
//! - **Very Large**: 16 blocks (stress test scenarios)
//!
//! ## Usage
//! ```bash
//! cargo bench
//! ```

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use gtrusthop::{
    core::{State, Domain, PlanItem, Multigoal},
    planning::{PlannerBuilder, PlanningStrategy},
    examples::blocks_htn_example::create_blocks_htn_domain,
};
use std::time::Duration;

/// Problem size configuration for benchmarks
#[derive(Debug, Clone)]
struct ProblemSize {
    name: &'static str,
    num_blocks: usize,
    scenarios: Vec<BlocksScenario>,
}

/// Individual test scenario within a problem size
#[derive(Debug, Clone)]
struct BlocksScenario {
    name: &'static str,
    initial_state: fn(usize) -> State,
    goal_state: fn(usize) -> Multigoal,
}



/// Create all problem sizes with their test scenarios
fn create_problem_sizes() -> Vec<ProblemSize> {
    vec![
        ProblemSize {
            name: "Tiny",
            num_blocks: 3,
            scenarios: vec![
                BlocksScenario {
                    name: "simple_stack",
                    initial_state: create_simple_initial_state,
                    goal_state: create_simple_stack_goal,
                },
                BlocksScenario {
                    name: "reverse_stack",
                    initial_state: create_reverse_initial_state,
                    goal_state: create_reverse_stack_goal,
                },
                BlocksScenario {
                    name: "sussman_anomaly",
                    initial_state: create_sussman_initial_state,
                    goal_state: create_sussman_goal,
                },
            ],
        },
        ProblemSize {
            name: "Small",
            num_blocks: 5,
            scenarios: vec![
                BlocksScenario {
                    name: "tower_build",
                    initial_state: create_scattered_initial_state,
                    goal_state: create_tower_goal,
                },
                BlocksScenario {
                    name: "multi_tower",
                    initial_state: create_single_tower_initial_state,
                    goal_state: create_multi_tower_goal,
                },
                BlocksScenario {
                    name: "complex_rearrange",
                    initial_state: create_complex_initial_state,
                    goal_state: create_complex_goal,
                },
            ],
        },
        ProblemSize {
            name: "Medium",
            num_blocks: 8,
            scenarios: vec![
                BlocksScenario {
                    name: "pyramid_build",
                    initial_state: create_scattered_initial_state,
                    goal_state: create_pyramid_goal,
                },
                BlocksScenario {
                    name: "tower_split",
                    initial_state: create_single_tower_initial_state,
                    goal_state: create_split_towers_goal,
                },
                BlocksScenario {
                    name: "interleaved_stacks",
                    initial_state: create_interleaved_initial_state,
                    goal_state: create_interleaved_goal,
                },
            ],
        },
        ProblemSize {
            name: "Large",
            num_blocks: 12,
            scenarios: vec![
                BlocksScenario {
                    name: "mega_tower",
                    initial_state: create_scattered_initial_state,
                    goal_state: create_mega_tower_goal,
                },
                BlocksScenario {
                    name: "four_towers",
                    initial_state: create_single_tower_initial_state,
                    goal_state: create_four_towers_goal,
                },
                BlocksScenario {
                    name: "complex_pyramid",
                    initial_state: create_complex_initial_state,
                    goal_state: create_complex_pyramid_goal,
                },
            ],
        },
        ProblemSize {
            name: "VeryLarge",
            num_blocks: 16,
            scenarios: vec![
                BlocksScenario {
                    name: "ultimate_tower",
                    initial_state: create_scattered_initial_state,
                    goal_state: create_ultimate_tower_goal,
                },
                BlocksScenario {
                    name: "eight_towers",
                    initial_state: create_single_tower_initial_state,
                    goal_state: create_eight_towers_goal,
                },
                BlocksScenario {
                    name: "stress_test",
                    initial_state: create_stress_initial_state,
                    goal_state: create_stress_goal,
                },
            ],
        },
    ]
}

/// Generate block names for a given number of blocks
fn generate_block_names(num_blocks: usize) -> Vec<String> {
    (0..num_blocks)
        .map(|i| {
            if i < 26 {
                ((b'a' + i as u8) as char).to_string()
            } else {
                format!("block{}", i)
            }
        })
        .collect()
}

/// Create simple initial state: all blocks on table
fn create_simple_initial_state(num_blocks: usize) -> State {
    let mut state = State::new("initial");
    let blocks = generate_block_names(num_blocks);
    
    for block in &blocks {
        state.set_var("pos", block, "table".into());
        state.set_var("clear", block, true.into());
    }
    state.set_var("holding", "hand", false.into());
    state
}

/// Create scattered initial state: blocks in random positions
fn create_scattered_initial_state(num_blocks: usize) -> State {
    let mut state = State::new("scattered");
    let blocks = generate_block_names(num_blocks);
    
    // Create some initial stacks
    for (i, block) in blocks.iter().enumerate() {
        if i == 0 {
            state.set_var("pos", block, "table".into());
        } else if i % 3 == 0 {
            state.set_var("pos", block, "table".into());
        } else {
            let base_block = &blocks[i - 1];
            state.set_var("pos", block, base_block.as_str().into());
        }
    }
    
    // Set clear flags
    for (_i, block) in blocks.iter().enumerate() {
        let is_clear = !blocks.iter().any(|b| {
            if let Some(pos) = state.get_var("pos", b) {
                pos.as_str() == Some(block)
            } else {
                false
            }
        });
        state.set_var("clear", block, is_clear.into());
    }
    
    state.set_var("holding", "hand", false.into());
    state
}

/// Create single tower initial state: all blocks in one tower
fn create_single_tower_initial_state(num_blocks: usize) -> State {
    let mut state = State::new("single_tower");
    let blocks = generate_block_names(num_blocks);
    
    for (i, block) in blocks.iter().enumerate() {
        if i == 0 {
            state.set_var("pos", block, "table".into());
            state.set_var("clear", block, false.into());
        } else if i == blocks.len() - 1 {
            state.set_var("pos", block, blocks[i - 1].as_str().into());
            state.set_var("clear", block, true.into());
        } else {
            state.set_var("pos", block, blocks[i - 1].as_str().into());
            state.set_var("clear", block, false.into());
        }
    }
    
    state.set_var("holding", "hand", false.into());
    state
}

/// Create complex initial state: mixed configuration
fn create_complex_initial_state(num_blocks: usize) -> State {
    let mut state = State::new("complex");
    let blocks = generate_block_names(num_blocks);
    
    // Create alternating pattern
    for (i, block) in blocks.iter().enumerate() {
        match i % 4 {
            0 => state.set_var("pos", block, "table".into()),
            1 => state.set_var("pos", block, blocks[i - 1].as_str().into()),
            2 => state.set_var("pos", block, "table".into()),
            3 => state.set_var("pos", block, blocks[i - 1].as_str().into()),
            _ => unreachable!(),
        }
    }
    
    // Set clear flags
    for (_i, block) in blocks.iter().enumerate() {
        let is_clear = !blocks.iter().any(|b| {
            if let Some(pos) = state.get_var("pos", b) {
                pos.as_str() == Some(block)
            } else {
                false
            }
        });
        state.set_var("clear", block, is_clear.into());
    }
    
    state.set_var("holding", "hand", false.into());
    state
}

/// Create reverse initial state for tiny problems
fn create_reverse_initial_state(num_blocks: usize) -> State {
    let mut state = State::new("reverse");
    let blocks = generate_block_names(num_blocks);
    
    // Create reverse tower (c on b on a)
    for (i, block) in blocks.iter().rev().enumerate() {
        if i == 0 {
            state.set_var("pos", block, "table".into());
            state.set_var("clear", block, false.into());
        } else if i == blocks.len() - 1 {
            let base = &blocks[blocks.len() - i];
            state.set_var("pos", block, base.as_str().into());
            state.set_var("clear", block, true.into());
        } else {
            let base = &blocks[blocks.len() - i];
            state.set_var("pos", block, base.as_str().into());
            state.set_var("clear", block, false.into());
        }
    }
    
    state.set_var("holding", "hand", false.into());
    state
}

// Goal state creation functions

/// Create simple stack goal: all blocks in a single tower (a-b-c-...)
fn create_simple_stack_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("simple_stack");
    let blocks = generate_block_names(num_blocks);

    for (i, block) in blocks.iter().enumerate() {
        if i == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            goal.set_goal("pos", block, blocks[i - 1].as_str().into());
        }
    }
    goal
}

/// Create reverse stack goal: blocks in reverse order
fn create_reverse_stack_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("reverse_stack");
    let blocks = generate_block_names(num_blocks);

    for (i, block) in blocks.iter().rev().enumerate() {
        if i == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            let base_idx = blocks.len() - i;
            goal.set_goal("pos", block, blocks[base_idx - 1].as_str().into());
        }
    }
    goal
}

/// Create Sussman anomaly goal: A on B, B on C
fn create_sussman_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("sussman_goal");
    let blocks = generate_block_names(num_blocks.max(3));

    goal.set_goal("pos", &blocks[1], blocks[2].as_str().into()); // B on C
    goal.set_goal("pos", &blocks[0], blocks[1].as_str().into()); // A on B
    goal
}

/// Create tower goal: single tower from scattered blocks
fn create_tower_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("tower_goal");
    let blocks = generate_block_names(num_blocks);

    for (i, block) in blocks.iter().enumerate() {
        if i == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            goal.set_goal("pos", block, blocks[i - 1].as_str().into());
        }
    }
    goal
}

/// Create multi-tower goal: split blocks into multiple towers
fn create_multi_tower_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("multi_tower_goal");
    let blocks = generate_block_names(num_blocks);

    let towers = 2;
    let blocks_per_tower = num_blocks / towers;

    for (i, block) in blocks.iter().enumerate() {
        let tower_id = i / blocks_per_tower;
        let pos_in_tower = i % blocks_per_tower;

        if pos_in_tower == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            let base_idx = tower_id * blocks_per_tower + pos_in_tower - 1;
            if base_idx < blocks.len() {
                goal.set_goal("pos", block, blocks[base_idx].as_str().into());
            }
        }
    }
    goal
}

/// Create complex goal: mixed arrangement
fn create_complex_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("complex_goal");
    let blocks = generate_block_names(num_blocks);

    // Create alternating towers
    for (i, block) in blocks.iter().enumerate() {
        match i % 3 {
            0 => goal.set_goal("pos", block, "table".into()),
            1 => goal.set_goal("pos", block, blocks[i - 1].as_str().into()),
            2 => goal.set_goal("pos", block, "table".into()),
            _ => unreachable!(),
        }
    }
    goal
}

/// Create pyramid goal: pyramid structure
fn create_pyramid_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("pyramid_goal");
    let blocks = generate_block_names(num_blocks);

    // Simple pyramid: base blocks on table, others stacked
    let base_size = (num_blocks as f64).sqrt().ceil() as usize;

    for (i, block) in blocks.iter().enumerate() {
        if i < base_size {
            goal.set_goal("pos", block, "table".into());
        } else {
            let base_idx = (i - base_size) % base_size;
            goal.set_goal("pos", block, blocks[base_idx].as_str().into());
        }
    }
    goal
}

/// Create split towers goal: multiple equal towers
fn create_split_towers_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("split_towers_goal");
    let blocks = generate_block_names(num_blocks);

    let num_towers = 4;
    let blocks_per_tower = num_blocks / num_towers;

    for (i, block) in blocks.iter().enumerate() {
        let tower_id = i / blocks_per_tower;
        let pos_in_tower = i % blocks_per_tower;

        if pos_in_tower == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            let base_idx = tower_id * blocks_per_tower + pos_in_tower - 1;
            if base_idx < blocks.len() {
                goal.set_goal("pos", block, blocks[base_idx].as_str().into());
            }
        }
    }
    goal
}

/// Create interleaved goal: specific interleaved pattern
fn create_interleaved_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("interleaved_goal");
    let blocks = generate_block_names(num_blocks);

    // Create specific interleaved pattern
    for (i, block) in blocks.iter().enumerate() {
        if i < 2 {
            goal.set_goal("pos", block, "table".into());
        } else if i % 2 == 0 {
            goal.set_goal("pos", block, blocks[i - 2].as_str().into());
        } else {
            goal.set_goal("pos", block, blocks[i - 2].as_str().into());
        }
    }
    goal
}

/// Create mega tower goal: single very tall tower
fn create_mega_tower_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("mega_tower_goal");
    let blocks = generate_block_names(num_blocks);

    for (i, block) in blocks.iter().enumerate() {
        if i == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            goal.set_goal("pos", block, blocks[i - 1].as_str().into());
        }
    }
    goal
}

/// Create four towers goal: exactly four towers
fn create_four_towers_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("four_towers_goal");
    let blocks = generate_block_names(num_blocks);

    let blocks_per_tower = num_blocks / 4;

    for (i, block) in blocks.iter().enumerate() {
        let tower_id = i / blocks_per_tower;
        let pos_in_tower = i % blocks_per_tower;

        if pos_in_tower == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            let base_idx = tower_id * blocks_per_tower + pos_in_tower - 1;
            if base_idx < blocks.len() {
                goal.set_goal("pos", block, blocks[base_idx].as_str().into());
            }
        }
    }
    goal
}

/// Create complex pyramid goal: multi-level pyramid
fn create_complex_pyramid_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("complex_pyramid_goal");
    let blocks = generate_block_names(num_blocks);

    // Create a more complex pyramid structure
    let base_size = 4;

    for (i, block) in blocks.iter().enumerate() {
        if i < base_size {
            goal.set_goal("pos", block, "table".into());
        } else {
            let _level = (i - base_size) / (base_size - 1);
            let pos_in_level = (i - base_size) % (base_size - 1);
            let base_idx = pos_in_level;

            if base_idx < blocks.len() {
                goal.set_goal("pos", block, blocks[base_idx].as_str().into());
            }
        }
    }
    goal
}

/// Create ultimate tower goal: maximum height tower
fn create_ultimate_tower_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("ultimate_tower_goal");
    let blocks = generate_block_names(num_blocks);

    for (i, block) in blocks.iter().enumerate() {
        if i == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            goal.set_goal("pos", block, blocks[i - 1].as_str().into());
        }
    }
    goal
}

/// Create eight towers goal: eight equal towers
fn create_eight_towers_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("eight_towers_goal");
    let blocks = generate_block_names(num_blocks);

    let num_towers = 8;
    let blocks_per_tower = num_blocks / num_towers;

    for (i, block) in blocks.iter().enumerate() {
        let tower_id = i / blocks_per_tower;
        let pos_in_tower = i % blocks_per_tower;

        if pos_in_tower == 0 {
            goal.set_goal("pos", block, "table".into());
        } else {
            let base_idx = tower_id * blocks_per_tower + pos_in_tower - 1;
            if base_idx < blocks.len() {
                goal.set_goal("pos", block, blocks[base_idx].as_str().into());
            }
        }
    }
    goal
}

/// Create stress test goal: maximum complexity arrangement
fn create_stress_goal(num_blocks: usize) -> Multigoal {
    let mut goal = Multigoal::new("stress_goal");
    let blocks = generate_block_names(num_blocks);

    // Create maximum complexity goal with interdependencies
    for (i, block) in blocks.iter().enumerate() {
        match i % 6 {
            0 => goal.set_goal("pos", block, "table".into()),
            1 => goal.set_goal("pos", block, blocks[i - 1].as_str().into()),
            2 => goal.set_goal("pos", block, blocks[i - 1].as_str().into()),
            3 => goal.set_goal("pos", block, "table".into()),
            4 => goal.set_goal("pos", block, blocks[i - 1].as_str().into()),
            5 => goal.set_goal("pos", block, blocks[i - 3].as_str().into()),
            _ => unreachable!(),
        }
    }
    goal
}

// Benchmark execution functions

/// Execute a single planning benchmark
fn benchmark_planning(
    domain: &Domain,
    strategy: PlanningStrategy,
    initial_state: State,
    goal: Multigoal,
) -> Result<Option<Vec<PlanItem>>, Box<dyn std::error::Error>> {
    // Create planner with the multigoal using the new builder pattern
    let goal_id = format!("goal_{}", goal.name);
    let planner = PlannerBuilder::new()
        .with_domain(domain.clone())
        .with_strategy(strategy)
        .with_multigoal(goal)
        .with_verbose_level(0)?
        .build()?;

    // Create the planning task
    let todo_list = vec![PlanItem::task("achieve", vec![goal_id.into()])];

    // Execute planning
    let plan = planner.find_plan(initial_state, todo_list)?;

    Ok(plan)
}

/// Benchmark a specific scenario with both strategies
fn benchmark_scenario(
    c: &mut Criterion,
    domain: &Domain,
    problem_size: &ProblemSize,
    scenario: &BlocksScenario,
) {
    let group_name = format!("{}_{}", problem_size.name, scenario.name);
    let mut group = c.benchmark_group(&group_name);

    // Set measurement time for statistical significance
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    // Set throughput based on problem size (number of blocks)
    group.throughput(Throughput::Elements(problem_size.num_blocks as u64));

    let initial_state = (scenario.initial_state)(problem_size.num_blocks);
    let goal = (scenario.goal_state)(problem_size.num_blocks);

    // Benchmark iterative strategy
    group.bench_with_input(
        BenchmarkId::new("Iterative", problem_size.num_blocks),
        &problem_size.num_blocks,
        |b, _| {
            b.iter(|| {
                let result = benchmark_planning(
                    black_box(domain),
                    black_box(PlanningStrategy::Iterative),
                    black_box(initial_state.clone()),
                    black_box(goal.clone()),
                );
                black_box(result)
            })
        },
    );

    // Benchmark recursive strategy
    group.bench_with_input(
        BenchmarkId::new("Recursive", problem_size.num_blocks),
        &problem_size.num_blocks,
        |b, _| {
            b.iter(|| {
                let result = benchmark_planning(
                    black_box(domain),
                    black_box(PlanningStrategy::Recursive),
                    black_box(initial_state.clone()),
                    black_box(goal.clone()),
                );
                black_box(result)
            })
        },
    );

    group.finish();
}

/// Main benchmark function
fn planning_strategy_benchmarks(c: &mut Criterion) {
    // Create the blocks world domain
    let domain = create_blocks_htn_domain().expect("Failed to create blocks domain");

    // Get all problem sizes and scenarios
    let problem_sizes = create_problem_sizes();

    // Run benchmarks for each problem size and scenario
    for problem_size in &problem_sizes {
        for scenario in &problem_size.scenarios {
            benchmark_scenario(c, &domain, problem_size, scenario);
        }
    }
}

/// Benchmark memory usage (simplified version)
fn memory_usage_benchmarks(c: &mut Criterion) {
    let domain = create_blocks_htn_domain().expect("Failed to create blocks domain");
    let mut group = c.benchmark_group("memory_usage");

    // Test different problem sizes for memory characteristics
    let sizes = vec![3, 5, 8, 12, 16];

    for &size in &sizes {
        let initial_state = create_simple_initial_state(size);
        let goal = create_simple_stack_goal(size);

        group.bench_with_input(
            BenchmarkId::new("Iterative_Memory", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let result = benchmark_planning(
                        black_box(&domain),
                        black_box(PlanningStrategy::Iterative),
                        black_box(initial_state.clone()),
                        black_box(goal.clone()),
                    );
                    black_box(result)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Recursive_Memory", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let result = benchmark_planning(
                        black_box(&domain),
                        black_box(PlanningStrategy::Recursive),
                        black_box(initial_state.clone()),
                        black_box(goal.clone()),
                    );
                    black_box(result)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark backtracking scenarios
fn backtracking_benchmarks(c: &mut Criterion) {
    let domain = create_blocks_htn_domain().expect("Failed to create blocks domain");
    let mut group = c.benchmark_group("backtracking");

    // Test scenarios that require significant backtracking
    let sizes = vec![5, 8, 12];

    for &size in &sizes {
        // Use complex initial state and stress goal for maximum backtracking
        let initial_state = create_complex_initial_state(size);
        let goal = create_stress_goal(size);

        group.bench_with_input(
            BenchmarkId::new("Iterative_Backtrack", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let result = benchmark_planning(
                        black_box(&domain),
                        black_box(PlanningStrategy::Iterative),
                        black_box(initial_state.clone()),
                        black_box(goal.clone()),
                    );
                    black_box(result)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Recursive_Backtrack", size),
            &size,
            |b, _| {
                b.iter(|| {
                    let result = benchmark_planning(
                        black_box(&domain),
                        black_box(PlanningStrategy::Recursive),
                        black_box(initial_state.clone()),
                        black_box(goal.clone()),
                    );
                    black_box(result)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    planning_strategy_benchmarks,
    memory_usage_benchmarks,
    backtracking_benchmarks
);
criterion_main!(benches);

/// Create Sussman anomaly initial state
fn create_sussman_initial_state(num_blocks: usize) -> State {
    let mut state = State::new("sussman");
    let blocks = generate_block_names(num_blocks.max(3));
    
    // Classic Sussman anomaly: C on A, B on table, A on table
    state.set_var("pos", &blocks[0], "table".into()); // A on table
    state.set_var("pos", &blocks[1], "table".into()); // B on table  
    state.set_var("pos", &blocks[2], blocks[0].as_str().into()); // C on A
    
    state.set_var("clear", &blocks[0], false.into()); // A not clear (C on it)
    state.set_var("clear", &blocks[1], true.into());  // B clear
    state.set_var("clear", &blocks[2], true.into());  // C clear
    
    // Add remaining blocks on table if any
    for i in 3..num_blocks {
        state.set_var("pos", &blocks[i], "table".into());
        state.set_var("clear", &blocks[i], true.into());
    }
    
    state.set_var("holding", "hand", false.into());
    state
}

/// Create interleaved initial state
fn create_interleaved_initial_state(num_blocks: usize) -> State {
    let mut state = State::new("interleaved");
    let blocks = generate_block_names(num_blocks);
    
    // Create interleaved towers: a-c-e-g on table, b-d-f-h on table
    for (i, block) in blocks.iter().enumerate() {
        if i % 2 == 0 {
            if i == 0 {
                state.set_var("pos", block, "table".into());
            } else {
                state.set_var("pos", block, blocks[i - 2].as_str().into());
            }
        } else {
            if i == 1 {
                state.set_var("pos", block, "table".into());
            } else {
                state.set_var("pos", block, blocks[i - 2].as_str().into());
            }
        }
    }
    
    // Set clear flags
    for (_i, block) in blocks.iter().enumerate() {
        let is_clear = !blocks.iter().any(|b| {
            if let Some(pos) = state.get_var("pos", b) {
                pos.as_str() == Some(block)
            } else {
                false
            }
        });
        state.set_var("clear", block, is_clear.into());
    }
    
    state.set_var("holding", "hand", false.into());
    state
}

/// Create stress test initial state
fn create_stress_initial_state(num_blocks: usize) -> State {
    let mut state = State::new("stress");
    let blocks = generate_block_names(num_blocks);
    
    // Create maximum complexity: nested dependencies
    for (i, block) in blocks.iter().enumerate() {
        match i % 5 {
            0 => state.set_var("pos", block, "table".into()),
            1 => state.set_var("pos", block, blocks[i - 1].as_str().into()),
            2 => state.set_var("pos", block, blocks[i - 1].as_str().into()),
            3 => state.set_var("pos", block, "table".into()),
            4 => state.set_var("pos", block, blocks[i - 1].as_str().into()),
            _ => unreachable!(),
        }
    }
    
    // Set clear flags
    for (_i, block) in blocks.iter().enumerate() {
        let is_clear = !blocks.iter().any(|b| {
            if let Some(pos) = state.get_var("pos", b) {
                pos.as_str() == Some(block)
            } else {
                false
            }
        });
        state.set_var("clear", block, is_clear.into());
    }
    
    state.set_var("holding", "hand", false.into());
    state
}
