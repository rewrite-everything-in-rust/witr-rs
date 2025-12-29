# Contributing to witr-rs

Thank you for your interest in contributing to witr-rs! This document provides guidelines and information for contributors.

## Code of Conduct

- Be respectful and constructive
- Focus on what is best for the project and community
- Show empathy towards other contributors

## Getting Started

### Prerequisites

- Rust 1.88 or later
- Git
- Familiarity with Clean Architecture principles

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/witr-rs.git
cd witr-rs

# Build the project
cargo build

# Run tests
cargo test --all

# Run the tool
cargo run -- --help
```

## Development Guidelines

### Code Style

1. **Follow Rust conventions**

   - Use `cargo fmt` before committing
   - Run `cargo clippy` and fix all warnings
   - Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

2. **File size limit**

   - No file should exceed 300 lines
   - Split large files into logical modules
   - Use the `mod` system to organize code

3. **Comments**

   - Write self-documenting code with clear naming
   - Only add comments for non-obvious logic or public APIs
   - Use Rust doc comments (`///`) for public items
   - Avoid TODO comments in completed code

4. **Error handling**
   - Use `Result<T, E>` for operations that can fail
   - Use custom error types (defined in `ports.rs`)
   - Provide meaningful error messages

### Architecture

witr-rs follows Clean Architecture (Ports & Adapters):

```
src/
├── core/           # Business logic (no external dependencies)
│   ├── models/     # Domain entities
│   ├── ports.rs    # Interface definitions (traits)
│   ├── service.rs  # Business logic orchestration
│   └── ...
├── adapters/       # Infrastructure (OS-specific implementations)
│   ├── system.rs   # Main system adapter
│   └── ...
└── main.rs         # CLI entry point
```

**Rules:**

- Core should never depend on adapters
- Adapters implement traits defined in core/ports.rs
- Keep platform-specific code in adapters with `#[cfg(target_os = "...")]`

### Adding New Features

1. **Check the TODO list** in README.md
2. **Create an issue** describing what you want to implement
3. **Discuss the approach** before starting major work
4. **Write tests first** (TDD approach recommended)
5. **Implement the feature** following the architecture
6. **Update documentation** (README, code comments)

### Testing

All new code must include tests:

```bash
# Run all tests
cargo test --all

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

**Test requirements:**

- Unit tests for all core business logic
- Integration tests for adapter implementations
- Test coverage should not decrease
- All tests must pass on Linux, Windows, and macOS (if possible)

**Writing tests:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_output);
    }
}
```

### Platform-Specific Code

Use conditional compilation for platform-specific features:

```rust
#[cfg(target_os = "linux")]
pub fn linux_specific_function() {
    // Linux implementation
}

#[cfg(target_os = "windows")]
pub fn windows_specific_function() {
    // Windows implementation
}

#[cfg(target_os = "macos")]
pub fn macos_specific_function() {
    // macOS implementation
}
```

### Pull Request Process

1. **Fork the repository**
2. **Create a feature branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**

   - Write code
   - Add tests
   - Update documentation

4. **Verify everything works**

   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test --all
   cargo build --release
   ```

5. **Commit your changes**

   ```bash
   git add .
   git commit -m "feat: add descriptive commit message"
   ```

   **Commit message format:**

   - `feat: description` - New feature
   - `fix: description` - Bug fix
   - `docs: description` - Documentation only
   - `test: description` - Adding tests
   - `refactor: description` - Code refactoring
   - `perf: description` - Performance improvement

6. **Push to your fork**

   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**
   - Provide a clear description of your changes
   - Reference any related issues
   - Ensure CI passes

### Review Process

- All PRs require at least one approval
- Maintainers may request changes
- Be responsive to feedback
- Keep PRs focused and reasonably sized

## Project Structure

### Core Modules

- **models/** - Domain entities (Process, Source, Result, etc.)
- **ports.rs** - Interface definitions (SystemProvider trait)
- **service.rs** - Business logic (WitrService)
- **ancestry.rs** - Ancestry tree operations
- **time.rs** - Time formatting utilities
- **color.rs** - Color output utilities

### Adapter Modules

- **system.rs** - Main system adapter implementation
- **socketstate.rs** - Network socket detection
- **boot.rs** - Boot time detection
- **cmdline.rs** - Command-line parsing
- **fd.rs** - File descriptor handling
- **filecontext.rs** - File context detection
- **resource.rs** - Resource context (macOS)
- **user.rs** - User resolution

### Adding a New Adapter

1. Create `src/adapters/your_adapter.rs`
2. Implement necessary functions with platform guards
3. Add module to `src/adapters/mod.rs`
4. Use in `system.rs` where needed
5. Write unit tests
6. Update README if it adds user-facing features

## Common Tasks

### Adding a New CLI Flag

1. Add field to `Args` struct in `main.rs`
2. Update CLI help text
3. Implement handler logic in `main()`
4. Add tests
5. Update README usage section

### Adding a New Output Format

1. Create formatter function (e.g., `print_yaml`)
2. Add flag to `Args` struct
3. Add condition in main display logic
4. Write tests
5. Update documentation

### Improving Platform Support

1. Identify missing functionality for specific OS
2. Research platform-specific APIs/commands
3. Implement in appropriate adapter with `#[cfg(...)]`
4. Add unit tests (use mocking if necessary)
5. Test on actual platform if possible

## Getting Help

- **Questions**: Open a discussion on GitHub
- **Bugs**: Create an issue with reproduction steps
- **Feature ideas**: Open an issue for discussion first

## Recognition

Contributors will be acknowledged in release notes and the project README.

Thank you for contributing to witr-rs!
