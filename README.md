# Lambda MCP Server (LMS)

A production-ready lambda server providing local function-as-a-service (FaaS) for Large Language Models (LLMs) via the Model Context Protocol (MCP).

## Vision

Enable LLMs to safely and efficiently invoke, register, and revoke custom functions through a standardized MCP artifact format, with pluggable runtime support and strict security sandboxing.

## Core Goals

1. **MCP Function Interface**: LLMs can invoke, revoke, and register functions via MCP artifacts
2. **Multi-Runtime Support**: Pluggable runtimes for Python, Rust, Perl, Ruby, and more
3. **Security Sandboxing**: Zero OS access, capability-based restrictions, syscall filtering, namespace isolation
4. **Inter-Process Communication**: Functions can call other functions via IPC with controlled access
5. **Production Ready**: Robust error handling, observability, performance optimization

## Architecture

```
┌─────────────────────────────────────┐
│         MCP Protocol Layer          │
│  (Function Registration/Invocation) │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│      Function Registry & Router     │
│  (Manages function metadata & deps) │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│       Runtime Manager (Pluggable)   │
│  ┌──────────┐ ┌──────────┐          │
│  │ PyRuntime│ │RustRuntime          │
│  └──────────┘ └──────────┘ ...      │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│    Sandbox & Execution Engine       │
│  (Capabilities, Syscall Filtering)  │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│      IPC & Function Call Router     │
│  (Cross-function communication)     │
└─────────────────────────────────────┘
```

## Development Phases

### Phase 1: Core Setup ✓ (Current)
- [x] Project initialization and tooling
- [x] MCP protocol adapter
- [x] Basic function registry
- [x] Core server infrastructure

### Phase 2: Runtime Foundation
- [ ] Runtime abstraction layer
- [ ] Python runtime implementation
- [ ] Rust runtime implementation
- [ ] Runtime lifecycle management

### Phase 3: Sandboxing & Security
- [ ] Capability-based security model
- [ ] Syscall filtering (seccomp/landlock)
- [ ] Namespace isolation
- [ ] Resource limits (memory, CPU, time)

### Phase 4: IPC & Advanced Features
- [ ] IPC protocol and routing
- [ ] Function-to-function calls
- [ ] Dependency resolution
- [ ] Artifact format support

### Phase 5: Production Hardening
- [ ] Comprehensive testing
- [ ] Performance optimization
- [ ] Monitoring and observability
- [ ] Documentation

## Quick Start

```bash
# Clone and navigate
git clone https://github.com/MirS0bhan/lms.git
cd lms

# Build
cargo build --release

# Run
cargo run --release

# Test
cargo test
```

## Configuration

Configuration via `config.toml` or environment variables (see `.env.example`).

## Contributing

See `CONTRIBUTING.md` for development guidelines.

## License

MIT