use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

/// A handle to a spawned task that can be awaited
pub struct JoinHandle<T> {
    inner: Arc<Mutex<TaskInner<T>>>,
}

struct TaskInner<T> {
    output: Option<T>,
    waker: Option<Waker>,
    cancelled: bool,
}

impl<T> JoinHandle<T> {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(TaskInner {
                output: None,
                waker: None,
                cancelled: false,
            })),
        }
    }

    /// Poll the task for completion
    pub fn poll(&self, cx: &mut Context<'_>) -> Poll<Result<T, super::Cancelled>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.cancelled {
            return Poll::Ready(Err(super::Cancelled));
        }

        if let Some(output) = inner.output.take() {
            return Poll::Ready(Ok(output));
        }

        inner.waker = Some(cx.waker().clone());
        Poll::Pending
    }

    /// Set the task's output
    pub(crate) fn set_output(&self, output: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.output = Some(output);
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }

    /// Mark the task as cancelled
    pub(crate) fn cancel(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.cancelled = true;
        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, super::Cancelled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll(cx)
    }
}

impl<T> Clone for JoinHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}