//! Example 02: Timer Future
//!
//! This example shows how to build a timer future that uses
//! the waker mechanism to signal when it's ready.

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

/// Shared state between the future and the timer thread
struct TimerState {
    completed: bool,
    waker:     Option<Waker>,
}

/// A future that completes after a specified duration
pub struct TimerFuture {
    shared_state: Arc<Mutex<TimerState>>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(TimerState {
            completed: false,
            waker:     None,
        }));

        // Spawn a thread to complete the timer
        let thread_shared_state = Arc::clone(&shared_state);
        thread::spawn(move || {
            thread::sleep(duration);

            let mut state = thread_shared_state.lock().unwrap();
            state.completed = true;

            // Wake the task if it's waiting
            if let Some(waker) = state.waker.take() {
                println!("Timer expired, waking the task");
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.shared_state.lock().unwrap();

        if state.completed {
            println!("Timer future is ready!");
            Poll::Ready(())
        } else {
            // Store the waker so the timer thread can wake this task
            state.waker = Some(cx.waker().clone());
            println!("Timer future is pending, registering waker");
            Poll::Pending
        }
    }
}

/// A simple single-threaded executor
mod executor {
    use std::{
        collections::VecDeque,
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    };

    type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

    struct Executor {
        ready_queue: Arc<Mutex<VecDeque<Task>>>,
    }

    impl Executor {
        fn new() -> Self {
            Executor {
                ready_queue: Arc::new(Mutex::new(VecDeque::new())),
            }
        }

        fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
            let task = Box::pin(future);
            self.ready_queue.lock().unwrap().push_back(task);
        }

        fn run(&self) {
            loop {
                let mut task = {
                    let mut queue = self.ready_queue.lock().unwrap();
                    match queue.pop_front() {
                        Some(task) => task,
                        None => {
                            // No more tasks, sleep and check again
                            std::thread::sleep(std::time::Duration::from_millis(10));
                            continue;
                        }
                    }
                };

                let waker = self.create_waker();
                let mut context = Context::from_waker(&waker);

                match task.as_mut().poll(&mut context) {
                    Poll::Ready(()) => {
                        println!("Task completed");
                        // Task is done, don't re-queue
                        return; // Exit since we only have one task
                    }
                    Poll::Pending => {
                        // Don't re-queue immediately - wait for wake signal
                        // This is where a real executor would park the task
                        // For this simple example, we'll just sleep
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        self.ready_queue.lock().unwrap().push_back(task);
                    }
                }
            }
        }

        fn create_waker(&self) -> Waker {
            let _queue = Arc::clone(&self.ready_queue);

            unsafe fn clone(data: *const ()) -> RawWaker {
                let queue = unsafe { Arc::from_raw(data as *const Mutex<VecDeque<Task>>) };
                let cloned = Arc::clone(&queue);
                std::mem::forget(queue); // Don't drop the Arc
                RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
            }

            unsafe fn wake(data: *const ()) {
                let _queue = unsafe { Arc::from_raw(data as *const Mutex<VecDeque<Task>>) };
                // In a real executor, we'd wake the specific task
                // For this simple example, the task is already queued
                println!("Wake called!");
                // Arc will be dropped here
            }

            unsafe fn wake_by_ref(data: *const ()) {
                let queue = unsafe { Arc::from_raw(data as *const Mutex<VecDeque<Task>>) };
                println!("Wake by ref called!");
                std::mem::forget(queue); // Don't drop the Arc
            }

            unsafe fn drop(data: *const ()) {
                let _ = unsafe { Arc::from_raw(data as *const Mutex<VecDeque<Task>>) };
                // Arc will be dropped here
            }

            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

            let raw_waker = RawWaker::new(Arc::into_raw(Arc::clone(&self.ready_queue)) as *const (), &VTABLE);

            unsafe { Waker::from_raw(raw_waker) }
        }
    }

    pub fn block_on(future: impl Future<Output = ()> + Send + 'static) {
        let executor = Executor::new();
        executor.spawn(future);
        executor.run();
    }
}

fn main() {
    println!("Starting timer example");
    let start = Instant::now();

    // Create a 1-second timer
    let timer = TimerFuture::new(Duration::from_secs(1));

    println!("Created timer, running executor...");
    executor::block_on(async move {
        timer.await;
        println!("Timer completed after {:?}", start.elapsed());
    });
}
