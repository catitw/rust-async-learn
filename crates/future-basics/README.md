# Future Basics

Understanding the core building blocks of Rust's async ecosystem: Future trait, Poll, Waker, and Context.

## Learning Objectives

1. **Understand the Future trait** - Learn how futures represent asynchronous computations
2. **Poll and Waker mechanism** - Explore how futures are polled and woken up
3. **Manual Future implementation** - Build custom futures from scratch
4. **Context and task spawning** - Understand the executor context
5. **State machines** - See how async/await desugars to state machines

## Prerequisites

- Basic Rust knowledge (ownership, traits, generics)
- Understanding of Option and Result types
- Familiarity with closures and function pointers

## Topics Covered

### 1. The Future Trait

- Definition and associated types
- Poll enum (Ready/Pending)
- Relationship to async/await

### 2. Polling Mechanism

- How executors drive futures
- The role of Waker
- Spurious wakeups

### 3. Building Custom Futures

- Timer future from scratch
- State machine patterns
- Composing futures

### 4. Context and Waker

- Task context
- Waker cloning and wake-by-ref
- Wake implementations

### 5. Common Patterns

- Leaf futures vs combinators
- Fusing futures
- Pin and self-referential futures (preview)

## Examples

- `01_simple_future` - A minimal future that's immediately ready
- `02_timer_future` - Building a timer future with waker support
- `03_manual_join` - Implementing join! combinator manually
- `04_state_machine` - How async blocks become state machines
- `05_yield_now` - Creating a future that yields control

## Exercises

Each example includes exercises to reinforce learning:

- Implement a CountdownFuture
- Build a simple channel using futures
- Create a timeout combinator
- Debug wake patterns

## Running Examples

```bash
# Run a specific example
cargo run --example 01_simple_future

# Run with logging to see wake patterns
RUST_LOG=debug cargo run --example 02_timer_future
```

## Further Reading

- [Async Book - Under the Hood](https://rust-lang.github.io/async-book/02_execution/01_chapter.html)
- [Future trait documentation](https://doc.rust-lang.org/std/future/trait.Future.html)

