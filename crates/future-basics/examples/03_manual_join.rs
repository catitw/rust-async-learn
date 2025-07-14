//! Example 03: Manual Join Implementation
//!
//! This example shows how to implement a join combinator that runs
//! two futures concurrently and completes when both are done.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A future that joins two other futures
pub struct Join<F1, F2>
where
    F1: Future,
    F2: Future,
{
    future1: Option<F1>,
    future2: Option<F2>,
    output1: Option<F1::Output>,
    output2: Option<F2::Output>,
}

impl<F1, F2> Join<F1, F2>
where
    F1: Future,
    F2: Future,
{
    pub fn new(future1: F1, future2: F2) -> Self {
        Join {
            future1: Some(future1),
            future2: Some(future2),
            output1: None,
            output2: None,
        }
    }
}

impl<F1, F2> Future for Join<F1, F2>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
{
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        // Poll the first future if not complete
        if this.output1.is_none() {
            if let Some(mut future1) = this.future1.take() {
                match Pin::new(&mut future1).poll(cx) {
                    Poll::Ready(output) => {
                        println!("Future 1 completed");
                        this.output1 = Some(output);
                    }
                    Poll::Pending => {
                        this.future1 = Some(future1);
                    }
                }
            }
        }

        // Poll the second future if not complete
        if this.output2.is_none() {
            if let Some(mut future2) = this.future2.take() {
                match Pin::new(&mut future2).poll(cx) {
                    Poll::Ready(output) => {
                        println!("Future 2 completed");
                        this.output2 = Some(output);
                    }
                    Poll::Pending => {
                        this.future2 = Some(future2);
                    }
                }
            }
        }

        // Check if both are complete
        match (this.output1.take(), this.output2.take()) {
            (Some(output1), Some(output2)) => {
                println!("Both futures completed!");
                Poll::Ready((output1, output2))
            }
            (output1, output2) => {
                // Put back any outputs we took
                this.output1 = output1;
                this.output2 = output2;
                Poll::Pending
            }
        }
    }
}

/// Helper function to create a join future
pub fn join<F1, F2>(future1: F1, future2: F2) -> Join<F1, F2>
where
    F1: Future,
    F2: Future,
{
    Join::new(future1, future2)
}

// Import our timer from the previous example
use std::time::Duration;

mod timer {
    use std::{
        sync::{Arc, Mutex},
        task::Waker,
        thread,
    };

    use super::*;

    pub struct TimerFuture {
        shared_state: Arc<Mutex<TimerState>>,
    }

    struct TimerState {
        completed: bool,
        waker:     Option<Waker>,
    }

    impl TimerFuture {
        pub fn new(duration: Duration) -> Self {
            let shared_state = Arc::new(Mutex::new(TimerState {
                completed: false,
                waker:     None,
            }));

            let thread_shared_state = Arc::clone(&shared_state);
            thread::spawn(move || {
                thread::sleep(duration);
                let mut state = thread_shared_state.lock().unwrap();
                state.completed = true;
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            });

            TimerFuture { shared_state }
        }
    }

    impl Future for TimerFuture {
        type Output = Duration;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut state = self.shared_state.lock().unwrap();
            if state.completed {
                Poll::Ready(Duration::from_secs(1)) // Dummy duration
            } else {
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

use timer::TimerFuture;

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
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

fn main() {
    println!("Testing manual join implementation");

    // Create two timers with different durations
    let timer1 = TimerFuture::new(Duration::from_millis(500));
    let timer2 = TimerFuture::new(Duration::from_secs(1));

    println!("Starting two timers: 500ms and 1s");
    let start = std::time::Instant::now();

    let joined = join(timer1, timer2);
    let (result1, result2) = block_on(joined);

    println!("Both timers completed after {:?}", start.elapsed());
    println!("Results: {result1:?} and {result2:?}");

    // Exercise 1: Implement a Select combinator that completes when
    // the first of two futures completes

    // Exercise 2: Create a JoinAll that takes a Vec of futures
    // and completes when all are done

    // Exercise 3: Add proper Pin projection to handle !Unpin futures
    // Hint: Look into pin_project or manual projection
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ready<T>(Option<T>);

    impl<T: Unpin> Future for Ready<T> {
        type Output = T;

        fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(self.0.take().expect("polled after completion"))
        }
    }

    #[test]
    fn join_two_ready_futures() {
        let future1 = Ready(Some(42));
        let future2 = Ready(Some("hello"));

        let joined = join(future1, future2);
        let (num, text) = block_on(joined);

        assert_eq!(num, 42);
        assert_eq!(text, "hello");
    }
}
