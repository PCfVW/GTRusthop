# Cross-Platform Command Reference

This document provides a quick reference for all cross-platform commands used in GTRusthop development, ensuring developers on any platform can follow the documentation without translation.

## Installation Commands

### Rust Installation

**Windows:**
```powershell
# Method 1: Download from rustup.rs
# Visit https://rustup.rs/ and download rustup-init.exe

# Method 2: Using winget
winget install Rustlang.Rustup

# Method 3: Using PowerShell
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
.\rustup-init.exe
```

**Linux/macOS:**
```bash
# Standard installation
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add to shell profile
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Verification
```bash
# Same on all platforms
rustc --version
cargo --version
rustup update
```

## Project Setup Commands

### Creating New Projects

**Windows (PowerShell/Command Prompt):**
```powershell
# Create new project
cargo new my_planning_project
cd my_planning_project

# Add GTRusthop dependency
cargo add gtrusthop
cargo add serde --features derive
cargo add serde_json

# Check project structure
dir src\
```

**Linux/macOS (bash/zsh):**
```bash
# Create new project
cargo new my_planning_project
cd my_planning_project

# Add GTRusthop dependency
cargo add gtrusthop
cargo add serde --features derive
cargo add serde_json

# Check project structure
ls -la src/
```

## Build and Test Commands

### Basic Build Commands
```bash
# Same on all platforms
cargo build              # Debug build
cargo build --release    # Release build
cargo clean              # Clean build artifacts
```

### Testing Commands
```bash
# Same on all platforms
cargo test                    # Run all tests
cargo test --release          # Run tests in release mode
cargo test -- --nocapture     # Show test output
cargo test specific_test      # Run specific test
cargo test --doc              # Run documentation tests
```

### Running Examples
```bash
# Same on all platforms
cargo run                # Run main example
cargo run --release      # Run in release mode
```

### Running Individual Example Modules

GTRusthop includes several example modules that can be executed individually using `cargo test` commands. This is the recommended approach for exploring specific planning scenarios.

**Available Example Modules:**
```bash
# Same on all platforms
cargo test simple_htn_example    # Basic HTN planning (travel scenarios)
cargo test blocks_htn_example    # Blocks world planning problems
cargo test simple_hgn_example    # Hierarchical Goal Network examples
cargo test regression_tests      # Comprehensive regression tests

# Run with detailed output
cargo test simple_htn_example -- --nocapture
cargo test blocks_htn_example -- --nocapture
```

**Platform-Specific Execution:**

**Windows (PowerShell/Command Prompt):**
```powershell
# Navigate to project directory
GTRusthop> cd path\to\GTRusthop

# Run individual examples
GTRusthop> cargo test simple_htn_example
GTRusthop> cargo test blocks_htn_example
GTRusthop> cargo test simple_hgn_example
GTRusthop> cargo test regression_tests

# With detailed output
GTRusthop> cargo test simple_htn_example -- --nocapture
```

**Linux/macOS (bash/zsh):**
```bash
# Navigate to project directory
GTRusthop$ cd path/to/GTRusthop

# Run individual examples
GTRusthop$ cargo test simple_htn_example
GTRusthop$ cargo test blocks_htn_example
GTRusthop$ cargo test simple_hgn_example
GTRusthop$ cargo test regression_tests

# With detailed output
GTRusthop$ cargo test simple_htn_example -- --nocapture
```

## Environment Variables

### Setting Environment Variables

**Windows (PowerShell):**
```powershell
# Set environment variable
$env:RUST_LOG = "debug"
$env:GTRUSTHOP_CONFIG = "C:\MyProject\config.toml"

# Run with environment variable
cargo run
```

**Windows (Command Prompt):**
```cmd
# Set and run in one command
set RUST_LOG=debug && cargo run

# Or using cmd from PowerShell
cmd /c "set RUST_LOG=debug && cargo run"
```

**Linux/macOS (bash/zsh):**
```bash
# Set environment variable
export RUST_LOG="debug"
export GTRUSTHOP_CONFIG="/home/user/myproject/config.toml"

# Or set temporarily for one command
RUST_LOG=debug cargo run
```

## Development Workflow Commands

### Code Quality Tools

**Windows (PowerShell/Command Prompt):**
```powershell
# Linting and formatting
cargo clippy              # Run linter
cargo clippy --fix        # Auto-fix issues
cargo fmt                 # Format code

# Documentation
cargo doc --open          # Generate and open docs

# Performance measurement
Measure-Command { cargo build --release }
Measure-Command { cargo test --release }
```

**Linux/macOS (bash/zsh):**
```bash
# Linting and formatting
cargo clippy              # Run linter
cargo clippy --fix        # Auto-fix issues
cargo fmt                 # Format code

# Documentation
cargo doc --open          # Generate and open docs

# Performance measurement
time cargo build --release
time cargo test --release
```

### Development Tools Installation
```bash
# Same on all platforms
cargo install cargo-watch    # Auto-rebuild on changes
cargo install cargo-edit     # Enhanced cargo commands
cargo install cargo-audit    # Security audit
cargo install cargo-tarpaulin # Code coverage (Linux/macOS)
```

## File System Operations

### Navigation and File Operations

**Windows (PowerShell/Command Prompt):**
```powershell
# Directory operations
cd GTRusthop
mkdir docs
dir                       # List files
dir src\                  # List subdirectory

# File operations
copy src\main.rs src\backup.rs
type src\main.rs          # Display file content
```

**Linux/macOS (bash/zsh):**
```bash
# Directory operations
cd GTRusthop
mkdir docs
ls -la                    # List files
ls -la src/               # List subdirectory

# File operations
cp src/main.rs src/backup.rs
cat src/main.rs           # Display file content
```

## Path Handling

### File Paths in Code

**Cross-Platform Rust Code:**
```rust
use std::path::PathBuf;
use std::env;

// Cross-platform path construction
let mut config_path = PathBuf::new();

if cfg!(windows) {
    config_path.push(env::var("APPDATA").unwrap_or_else(|_| "C:\\".to_string()));
} else {
    config_path.push(env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()));
    config_path.push(".config");
}

config_path.push("gtrusthop");
config_path.push("config.toml");
```

**Platform-Specific Paths:**
```rust
// Windows paths
let windows_path = r"C:\MyProject\config.toml";
let windows_dir = r"C:\MyProject\domains\";

// Unix paths
let unix_path = "/home/user/myproject/config.toml";
let unix_dir = "/home/user/myproject/domains/";
```

## Configuration Files

### TOML Configuration Examples

**Windows (config.toml):**
```toml
strategy = "Iterative"
verbose_level = 1

[paths]
domain_dir = "C:\\MyProject\\domains"
output_dir = "C:\\MyProject\\output"
log_file = "C:\\MyProject\\logs\\planning.log"
```

**Linux/macOS (config.toml):**
```toml
strategy = "Iterative"
verbose_level = 1

[paths]
domain_dir = "/home/user/myproject/domains"
output_dir = "/home/user/myproject/output"
log_file = "/home/user/myproject/logs/planning.log"
```

## Debugging and Logging

### Verbose Output

**Windows (PowerShell/Command Prompt):**
```powershell
# PowerShell
$env:RUST_LOG = "gtrusthop=debug"
cargo run

# Command Prompt
cmd /c "set RUST_LOG=gtrusthop=debug && cargo run"

# Capture output
cargo run > output.txt 2>&1
```

**Linux/macOS (bash/zsh):**
```bash
# Set logging level
RUST_LOG=gtrusthop=debug cargo run

# Capture output
cargo run > output.txt 2>&1

# View and save output
cargo run 2>&1 | tee output.txt
```

## Performance Optimization

### Build Optimization

**Release Profile (Cargo.toml):**
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

**Platform-Specific Optimizations:**
```toml
# Windows-specific
[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }

# Unix-specific
[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

## Common Issues and Solutions

### Windows-Specific Issues

1. **Long Path Names**: Enable long path support in Windows 10/11
2. **Windows Defender**: Add Rust directories to exclusions for faster builds
3. **PowerShell Execution Policy**: May need to run `Set-ExecutionPolicy RemoteSigned`

### Linux/macOS-Specific Issues

1. **Missing Build Tools**: Install `build-essential` (Ubuntu) or Xcode Command Line Tools (macOS)
2. **Permission Issues**: Ensure proper permissions for `~/.cargo` directory
3. **Library Dependencies**: Install system libraries as needed (`pkg-config`, etc.)

This reference ensures that developers on any platform can successfully work with GTRusthop without needing to translate commands or guess at platform-specific syntax.
