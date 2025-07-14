# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project focused on learning async programming concepts. Currently in initial setup phase with minimal structure.

## Architecture Notes

**Current State**: Project is in initial setup with only Cargo.toml and .gitignore present. No source files exist yet.

**Intended Structure**: This will be a cargo workspace with each crate covering one async topic. The workspace should provide a comprehensive learning path through Rust's async ecosystem.

## Planned Async Topics for Workspace Crates

### Foundation Layer

1. **future-basics** - Future trait, Poll, Waker, Context, manual Future implementation
2. **pin-unpin-safety** - Memory safety with Pin, self-referential structs, pin projections

### Runtime & Execution

5. **executor-internals** - Building a minimal executor, task scheduling, reactor pattern
6. **runtime-comparison** - Comparing tokio, async-std, smol - architectures and use cases

### Concurrency Patterns

7. **async-concurrency** - join!, select!, FuturesUnordered, concurrent execution patterns
8. **async-channels** - mpsc, oneshot, broadcast, actor patterns, backpressure
9. **async-sync-primitives** - Async Mutex/RwLock, Semaphore, Notify, Arc in async

### I/O & Streams

10. **async-io-patterns** - AsyncRead/Write, file and network I/O, zero-copy techniques
11. **async-streams** - Stream trait, combinators, async iteration, backpressure handling

### Advanced Patterns

12. **async-error-handling** - Error propagation, retry mechanisms, circuit breakers
13. **async-testing** - Testing strategies, mocking, deterministic testing
14. **async-performance** - Profiling, optimization, allocation patterns, task granularity

### Real-World Applications

15. **async-web-patterns** - Web services, middleware, connection pooling, graceful shutdown
16. **async-state-machines** - Complex flows, saga pattern, event sourcing
17. **async-distributed** - Microservices, service discovery, distributed tracing

### Special Topics

18. **async-embedded** - no_std async, Embassy framework, embedded constraints
19. **async-wasm** - Browser and WASI async, JavaScript Promise interop
20. **async-patterns-cookbook** - Common recipes: rate limiting, retries, pools, caching

## Development Approach

When implementing a new crate:

1. Start with a focused README explaining the learning objectives
2. Build examples progressively from simple to complex
3. Include exercises with solutions
4. Add benchmarks where relevant to compare approaches
5. Document common pitfalls and their solutions
6. Link to upstream documentation and further reading

## Common Commands

```bash
# Workspace commands
cargo build --workspace              # Build all crates
cargo test --workspace               # Test all crates
cargo build -p <crate-name>          # Build specific crate
cargo test -p <crate-name>           # Test specific crate

# Development
cargo check --workspace              # Fast type-check all crates
cargo clippy --workspace --all-targets
cargo fmt --all

# Documentation
cargo doc --workspace --no-deps --open

# Running examples
cargo run -p <crate-name> --example <example-name>
```
