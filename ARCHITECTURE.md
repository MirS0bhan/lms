# Lambda MCP Server Architecture

## Overview

The Lambda MCP Server is designed as a modular, production-ready system for executing sandboxed functions requested by LLMs through the Model Context Protocol (MCP).

## Core Modules

### 1. MCP Protocol Layer (`src/mcp/`)
- **protocol.rs**: Message types and structures (RegisterFunction, InvokeFunction, etc.)
- **artifacts.rs**: Function artifact format supporting multiple runtimes
- **handlers.rs**: Request/response handling for MCP messages

### 2. Function Registry (`src/registry/`)
- Central repository of registered functions
- Metadata management (name, runtime, version, dependencies)
- Thread-safe access via DashMap

### 3. Runtime Abstraction (`src/runtime/`)
- Trait-based runtime interface
- RuntimeManager for pluggable runtime implementations
- Support for: Python, Rust, Perl, Ruby, (extensible)

### 4. Sandboxing & Security (`src/sandbox/`)
- SandboxPolicy: Configuration for resource limits and restrictions
- Capability-based access control (CAPs)
- Syscall filtering (seccomp)
- Namespace isolation (PID, Network, Mount)

### 5. IPC Router (`src/ipc/`)
- IpcMessage: Function-to-function call protocol
- IpcResponse: Result and error handling
- IpcRouter trait for call routing and access control

### 6. HTTP Server (`src/server/`)
- Axum-based async HTTP API
- RESTful endpoints for function management
- Health checks and monitoring

### 7. Error Handling (`src/error.rs`)
- Unified error types using `thiserror`
- Comprehensive error variants for all failure modes

### 8. Configuration (`src/config.rs`)
- Extensible configuration management
- Environment-based overrides
- Defaults for all parameters

## Request Flow

```
Client (LLM)
    │
    ├─→ MCP Message (RegisterFunction/InvokeFunction/etc.)
    │
    ▼
HTTP Server (Axum)
    │
    ├─→ Parse & Validate
    │
    ▼
MCP Handler
    │
    ├─→ Dispatch to appropriate handler
    │
    ├─→ Function Registry lookup
    │
    ▼
Runtime Manager
    │
    ├─→ Select runtime (Python/Rust/etc.)
    │
    ▼
Sandbox Executor
    │
    ├─→ Apply security policies
    ├─→ Set resource limits
    ├─→ Execute in isolated environment
    │
    ▼
Function Execution
    │
    ├─→ If calling other functions: IPC Router
    │
    ▼
Response
    │
    ├─→ MCP Response Message
    │
    ▼
Client (LLM)
```

## Security Model

### Capability-Based Access Control
- Functions run with minimal capabilities
- No CAP_SYS_ADMIN or other dangerous capabilities
- CAPs whitelist-based

### Syscall Filtering
- seccomp-bpf for syscall restrictions
- Whitelist essential syscalls (read, write, mmap, etc.)
- Block OS-level access (open, socket, etc.)

### Namespace Isolation
- PID namespace: Processes see only their own processes
- Network namespace: Isolated from host network (if enabled)
- Mount namespace: Private filesystem view

### Resource Limits
- Memory: Configurable hard limit
- CPU Time: Execution timeout
- File Descriptors: Limited per process
- Process Count: Max 1 per function by default

### Whitelist Mode
- Functions run with zero-trust approach
- Access must be explicitly granted
- Includes function-to-function call permissions

## Development Phases

### ✓ Phase 1: Core Setup
- Project structure
- MCP protocol definitions
- Basic registry and server
- Error handling framework

### ⏳ Phase 2: Runtime Foundation
- Runtime abstraction implementation
- Python runtime
- Rust runtime
- Runtime lifecycle management

### ⏳ Phase 3: Sandboxing & Security
- seccomp filter implementation
- Namespace setup and management
- Resource limit enforcement
- Capability handling

### ⏳ Phase 4: IPC & Advanced Features
- IPC routing and access control
- Function-to-function calls
- Dependency resolution
- Artifact loading

### ⏳ Phase 5: Production Hardening
- Comprehensive testing
- Performance optimization
- Monitoring and observability
- Documentation

## Key Design Decisions

1. **Trait-Based Runtimes**: Easy to extend with new runtimes
2. **DashMap Registry**: Lock-free concurrent access to function metadata
3. **Async-First**: Full async/await for scalability
4. **Strong Type Safety**: Rust's type system prevents entire classes of bugs
5. **Minimal Permissions**: Default-deny security model
6. **IPC Over Network**: Allows future distributed deployment

## Future Considerations

- Distributed deployment with remote runtimes
- Function versioning and rollback
- Persistent storage for registered functions
- Function monitoring and metrics
- Advanced scheduling policies
- Hot-reloading of function code