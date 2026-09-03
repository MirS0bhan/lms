# Contributing to Lambda MCP Server

Thank you for your interest in contributing! This document outlines guidelines and best practices.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/MirS0bhan/lms.git
cd lms

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Setup development environment
cp .env.example .env
cargo build
cargo test
```

## Code Style

- Follow Rust conventions via `rustfmt`
- Use `clippy` for linting
- Maximum line length: 100 characters (soft limit)
- Document public APIs with doc comments

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

## Testing

- Write tests for all new functionality
- Aim for >80% code coverage
- Use descriptive test names

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name -- --nocapture

# Test with logging
RUST_LOG=debug cargo test
```

## Commit Messages

- Use clear, descriptive commit messages
- Format: `<type>: <subject>`
- Types: `feat`, `fix`, `docs`, `style`, `test`, `refactor`

Example:
```
feat: Add Python runtime implementation
fix: Handle graceful shutdown in MCP handler
docs: Update architecture documentation
```

## Pull Requests

1. Create a feature branch from `phase-N/...` branch
2. Write tests for new functionality
3. Update documentation if needed
4. Ensure all tests pass locally
5. Submit PR with clear description of changes

## Project Structure

- `src/main.rs`: Entry point
- `src/mcp/`: MCP protocol and artifact handling
- `src/registry/`: Function registry
- `src/runtime/`: Runtime abstraction and implementations
- `src/sandbox/`: Security and sandboxing
- `src/ipc/`: Inter-process communication
- `src/server/`: HTTP API server
- `src/error.rs`: Error types
- `src/config.rs`: Configuration management

## Naming Conventions

- **Files**: snake_case
- **Functions/Methods**: snake_case
- **Types/Structs**: PascalCase
- **Constants**: SCREAMING_SNAKE_CASE
- **Private helpers**: prefix with `_` or use module privacy

## Documentation

- Add doc comments to all public items
- Use markdown in doc comments
- Include examples in doc comments for complex functionality

```rust
/// Registers a new function in the registry
///
/// # Arguments
///
/// * `metadata` - Function metadata including name and runtime
///
/// # Errors
///
/// Returns an error if a function with the same name already exists
///
/// # Example
///
/// ```
/// let registry = FunctionRegistry::new();
/// let metadata = FunctionMetadata::new("my_fn".to_string(), "python".to_string());
/// registry.register(metadata)?;
/// ```
pub fn register(&self, metadata: FunctionMetadata) -> Result<String> {
    // ...
}
```

## Security Considerations

- Never expose sensitive configuration
- Always validate user input
- Follow principle of least privilege
- Use secure defaults
- Document security assumptions

## Phase Boundaries

Each development phase has specific goals:
- Stay within your phase's scope
- Don't implement Phase N+1 features in Phase N
- Mark incomplete features with `// TODO: Phase X:`
- Use `#[cfg(feature = "...")] for experimental code

## Questions?

- Check existing issues and discussions
- Read through the ARCHITECTURE.md
- Ask in GitHub discussions
- Review similar PRs for context

Happy contributing! 🚀