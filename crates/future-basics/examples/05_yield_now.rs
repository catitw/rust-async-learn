//! Example 05: Yield Now
//!
//! This example shows how to create a future that yields control
//! back to the executor, allowing other tasks to run.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A future that yields control once, then completes
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
            // Wake immediately to reschedule this task
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Convenience function to create a yield point
fn yield_now() -> YieldNow {
    YieldNow::new()
}

/// A cooperative task that yields periodically
struct CooperativeTask {
    id:         usize,
    work_done:  usize,
    total_work: usize,
}

impl CooperativeTask {
    fn new(id: usize, total_work: usize) -> Self {
        CooperativeTask {
            id,
            work_done: 0,
            total_work,
        }
    }
}

impl Future for CooperativeTask {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Do some work
        if self.work_done < self.total_work {
            self.work_done += 1;
            println!("Task {} did work unit {}/{}", self.id, self.work_done, self.total_work);

            // Yield every 3 work units to be cooperative
            if self.work_done % 3 == 0 {
                println!("Task {} yielding control", self.id);
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }

            // Continue working in the same poll
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            println!("Task {} completed all work", self.id);
            Poll::Ready(())
        }
    }
}

/// Simple round-robin executor
mod executor {
    use std::{
        collections::VecDeque,
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    };

    type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

    pub struct Executor {
        tasks: Arc<Mutex<VecDeque<Task>>>,
    }

    impl Executor {
        pub fn new() -> Self {
            Executor {
                tasks: Arc::new(Mutex::new(VecDeque::new())),
            }
        }

        pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
            let task = Box::pin(future);
            self.tasks.lock().unwrap().push_back(task);
        }

        pub fn run(&self) {
            loop {
                let mut task = {
                    let mut tasks = self.tasks.lock().unwrap();
                    match tasks.pop_front() {
                        Some(task) => task,
                        None => break, // No more tasks
                    }
                };

                let waker = self.create_waker();
                let mut context = Context::from_waker(&waker);

                match task.as_mut().poll(&mut context) {
                    Poll::Ready(()) => {
                        // Task completed
                    }
                    Poll::Pending => {
                        // Task will be re-queued when woken
                        self.tasks.lock().unwrap().push_back(task);
                    }
                }
            }
        }

        fn create_waker(&self) -> Waker {
            let tasks = Arc::clone(&self.tasks);

            unsafe fn clone(data: *const ()) -> RawWaker {
                let tasks = unsafe { Arc::from_raw(data as *const Mutex<VecDeque<Task>>) };
                let cloned = Arc::clone(&tasks);
                std::mem::forget(tasks);
                RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
            }

            unsafe fn wake(data: *const ()) {
                let _tasks = unsafe { Arc::from_raw(data as *const Mutex<VecDeque<Task>>) };
                // Task was already re-queued when it returned Pending
            }

            unsafe fn wake_by_ref(data: *const ()) {
                let tasks = unsafe { Arc::from_raw(data as *const Mutex<VecDeque<Task>>) };
                std::mem::forget(tasks);
                // Task was already re-queued when it returned Pending
            }

            unsafe fn drop(data: *const ()) {
                let _tasks = unsafe { Arc::from_raw(data as *const Mutex<VecDeque<Task>>) };
            }

            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

            let raw_waker = RawWaker::new(Arc::into_raw(tasks) as *const (), &VTABLE);

            unsafe { Waker::from_raw(raw_waker) }
        }
    }
}

fn main() {
    println!("=== Simple Yield Example ===");
    let executor = executor::Executor::new();

    // Spawn a task that yields
    executor.spawn(async {
        println!("Task 1: Starting");
        yield_now().await;
        println!("Task 1: After yield");
        yield_now().await;
        println!("Task 1: Completed");
    });

    executor.spawn(async {
        println!("Task 2: Starting");
        yield_now().await;
        println!("Task 2: After yield");
        println!("Task 2: Completed");
    });

    executor.run();

    println!("\n=== Cooperative Tasks ===");
    let executor2 = executor::Executor::new();

    // Spawn multiple cooperative tasks
    executor2.spawn(CooperativeTask::new(1, 10));
    executor2.spawn(CooperativeTask::new(2, 7));
    executor2.spawn(CooperativeTask::new(3, 5));

    executor2.run();

    // Exercise 1: Create a Sleep future that yields for a specified duration
    // using std::time::Instant to track elapsed time

    // Exercise 2: Implement a Producer-Consumer pattern where producers
    // yield after producing each item

    // Exercise 3: Create a RateLimiter future that yields to enforce
    // a maximum rate of operations
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_block_on<F: Future>(mut future: F) -> F::Output {
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

        let mut polls = 0;
        loop {
            polls += 1;
            if polls > 10 {
                panic!("Too many polls, likely infinite loop");
            }

            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => continue,
            }
        }
    }

    #[test]
    fn yield_now_yields_once() {
        let future = yield_now();
        let result = simple_block_on(future);
        assert_eq!(result, ());
    }

    #[test]
    fn cooperative_task_completes() {
        let task = CooperativeTask::new(1, 3);
        let result = simple_block_on(task);
        assert_eq!(result, ());
    }
}
