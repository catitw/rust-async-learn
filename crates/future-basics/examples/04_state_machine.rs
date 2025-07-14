//! Example 04: State Machine Pattern
//!
//! This example demonstrates how async blocks are transformed into
//! state machines under the hood, and how to build them manually.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Manual state machine that mimics an async block
/// This is roughly equivalent to:
/// ```
/// async {
///     let x = some_async_function().await;
///     let y = another_async_function(x).await;
///     y + 1
/// }
/// ```
enum MyStateMachine {
    Start,
    WaitingForFirst,
    WaitingForSecond { x: i32 },
    Done,
}

impl Future for MyStateMachine {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                MyStateMachine::Start => {
                    println!("State: Start");
                    // Simulate starting the first async operation
                    *self = MyStateMachine::WaitingForFirst;
                    // Continue to next state in the same poll
                }
                MyStateMachine::WaitingForFirst => {
                    println!("State: WaitingForFirst");
                    // Simulate the first async operation completing
                    // In a real scenario, this would poll another future
                    let x = 42; // Pretend this came from polling another future
                    *self = MyStateMachine::WaitingForSecond { x };
                    // Continue to next state in the same poll
                }
                MyStateMachine::WaitingForSecond { x } => {
                    println!("State: WaitingForSecond with x = {x}");
                    // Simulate the second async operation completing
                    let y = *x * 2; // Pretend this came from polling another future
                    *self = MyStateMachine::Done;
                    return Poll::Ready(y + 1);
                }
                MyStateMachine::Done => {
                    panic!("polled after completion");
                }
            }
        }
    }
}

/// A more realistic state machine that actually polls other futures
#[allow(dead_code)]
struct AsyncChain<F1, F2, Func> {
    state: ChainState<F1, F2, Func>,
}

#[allow(dead_code)]
enum ChainState<F1, F2, Func> {
    First(F1),
    Second(F2),
    Done,
    _Phantom(std::marker::PhantomData<Func>),
}

impl<F1, F2, Func> AsyncChain<F1, F2, Func>
where
    F1: Future,
    F2: Future,
    Func: FnOnce(F1::Output) -> F2,
{
    #[allow(dead_code)]
    fn new(future1: F1, _func: Func) -> Self {
        AsyncChain {
            state: ChainState::First(future1),
        }
    }
}

impl<F1, F2, Func> Future for AsyncChain<F1, F2, Func>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
    Func: FnOnce(F1::Output) -> F2,
{
    type Output = F2::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        loop {
            match &mut this.state {
                ChainState::First(future1) => {
                    match Pin::new(future1).poll(cx) {
                        Poll::Ready(_output) => {
                            // This is a simplified example showing the concept
                            // In a real implementation, we'd transition to Second state
                            todo!("Simplified example - see join combinator for working version")
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
                ChainState::Second(future2) => {
                    return Pin::new(future2).poll(cx);
                }
                ChainState::Done => panic!("polled after completion"),
                ChainState::_Phantom(_) => unreachable!(),
            }
        }
    }
}

/// A simpler example: countdown state machine
struct CountdownFuture {
    count: usize,
}

impl CountdownFuture {
    fn new(count: usize) -> Self {
        CountdownFuture { count }
    }
}

impl Future for CountdownFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            println!("Countdown complete!");
            Poll::Ready(())
        } else {
            println!("Countdown: {}", self.count);
            self.count -= 1;
            // Wake immediately to continue countdown
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Yield point future - yields control once then completes
struct YieldNow {
    yielded: bool,
}

impl YieldNow {
    fn new() -> Self {
        YieldNow { yielded: false }
    }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            // Wake immediately but return Pending to yield control
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// Simple block_on implementation
fn block_on<F: Future>(mut future: F) -> F::Output {
    use std::task::{RawWaker, RawWakerVTable, Waker};

    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut context = Context::from_waker(&waker);

    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => {
                // In a real executor, we'd wait for a wake signal
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }
}

fn main() {
    println!("=== Manual State Machine ===");
    let state_machine = MyStateMachine::Start;
    let result = block_on(state_machine);
    println!("Result: {result}\n");

    println!("=== Countdown Future ===");
    let countdown = CountdownFuture::new(3);
    block_on(countdown);
    println!();

    println!("=== Yield Now ===");
    let yield_future = YieldNow::new();
    block_on(yield_future);
    println!("YieldNow completed");

    // Exercise 1: Implement a PingPong state machine that alternates
    // between two states N times before completing

    // Exercise 2: Create a RetryFuture that retries another future
    // up to N times with exponential backoff

    // Exercise 3: Build a StateMachine trait that makes it easier
    // to define state machines with proper transitions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countdown_future_works() {
        let future = CountdownFuture::new(0);
        let result = block_on(future);
        assert_eq!(result, ());
    }

    #[test]
    fn yield_now_yields_once() {
        let future = YieldNow::new();
        let result = block_on(future);
        assert_eq!(result, ());
    }
}
