use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use crate::task::JoinHandle;

/// A single-threaded executor for running async tasks
pub struct Executor {
    tasks: Arc<Mutex<VecDeque<Task>>>,
}

type Task = Pin<Box<dyn Future<Output = ()> + 'static>>;

impl Executor {
    /// Creates a new executor
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Spawns a new task onto the executor
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let handle = JoinHandle::new();
        let handle_clone = handle.clone();

        let future = async move {
            let output = future.await;
            handle_clone.set_output(output);
        };

        let mut tasks = self.tasks.lock().unwrap();
        tasks.push_back(Box::pin(future));
        handle
    }

    /// Runs the executor until the given future completes
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        let waker = waker_fn();
        let mut cx = Context::from_waker(&waker);

        let mut future = Box::pin(future);
        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => {
                    let mut tasks = self.tasks.lock().unwrap();
                    if tasks.is_empty() {
                        continue;
                    }

                    let mut i = 0;
                    while i < tasks.len() {
                        let task = &mut tasks[i];
                        match task.as_mut().poll(&mut cx) {
                            Poll::Ready(()) => {
                                tasks.remove(i);
                            }
                            Poll::Pending => {
                                i += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn waker_fn() -> Waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};

    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}