//! Example 01: Simple Future
//!
//! This example demonstrates the most basic Future implementation
//! that is immediately ready with a value.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A future that is immediately ready with the given value
struct Ready<T> {
    value: Option<T>,
}

impl<T> Ready<T> {
    fn new(value: T) -> Self {
        Ready { value: Some(value) }
    }
}

impl<T: Unpin> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Take the value out of the Option
        // This ensures we only return Ready once
        match self.value.take() {
            Some(value) => Poll::Ready(value),
            None => panic!("Ready polled after completion"),
        }
    }
}

/// Manual executor that polls a future until completion
fn block_on<F: Future>(mut future: F) -> F::Output {
    use std::task::{RawWaker, RawWakerVTable, Waker};

    // Create a no-op waker
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

    // Pin the future to the stack
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => {
            // In a real executor, we would park the thread here
            // For this example, we just panic as Ready should never return Pending
            panic!("Ready future returned Pending!");
        }
    }
}

fn main() {
    println!("Creating a Ready future with value 42");
    let future = Ready::new(42);

    println!("Polling the future...");
    let result = block_on(future);

    println!("Future completed with value: {result}");

    // Exercise 1: Try to poll a Ready future twice
    // Uncomment the following to see what happens:
    // let mut future2 = Ready::new("hello");
    // let _ = block_on(&mut future2);
    // let _ = block_on(&mut future2); // This will panic!

    // Exercise 2: Create a NotReady future that always returns Pending
    // Hint: It should never store a value, just return Poll::Pending

    // Exercise 3: Modify Ready to count how many times it was polled
    // before returning the value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_returns_value() {
        let future = Ready::new(100);
        let result = block_on(future);
        assert_eq!(result, 100);
    }

    #[test]
    fn ready_works_with_different_types() {
        let future = Ready::new("hello world");
        let result = block_on(future);
        assert_eq!(result, "hello world");
    }
}
