# Async Learn - Rust Async Programming Tutorial

A comprehensive learning path through Rust's async ecosystem, organized as a cargo workspace with each crate focusing on a specific async topic.

## Structure

This workspace is organized into logical learning layers:

### Foundation Layer

- `future-basics` - Future trait, Poll, Waker, manual implementations
- `pin-unpin-safety` - Memory safety with Pin and self-referential structs

### Runtime & Execution

- `executor-internals` - Building a minimal executor
- `runtime-comparison` - Comparing tokio, async-std, and smol

### Concurrency Patterns

- `async-concurrency` - join!, select!, concurrent execution
- `async-channels` - Channel types and actor patterns
- `async-sync-primitives` - Async Mutex, RwLock, Semaphore

### I/O & Streams

- `async-io-patterns` - AsyncRead/Write, file and network I/O
- `async-streams` - Stream trait and async iteration

### Advanced Patterns

- `async-error-handling` - Error propagation and recovery
- `async-testing` - Testing strategies for async code
- `async-performance` - Profiling and optimization

### Real-World Applications

- `async-web-patterns` - Web services and middleware
- `async-state-machines` - Complex async flows
- `async-distributed` - Microservices and distributed systems

### Special Topics

- `async-embedded` - no_std async programming
- `async-wasm` - Browser and WASI async
- `async-patterns-cookbook` - Common recipes and patterns

## Learning Path

1. Start with the Foundation Layer crates in order
2. Move to Runtime & Execution to understand how async works
3. Learn Concurrency Patterns for practical async programming
4. Explore remaining topics based on your needs

Each crate contains:

- Detailed README with learning objectives
- Progressive examples
- Exercises with solutions
- Links to further reading

## Contributing

This is a learning project. Contributions that improve explanations, add examples, or fix issues are welcome!
