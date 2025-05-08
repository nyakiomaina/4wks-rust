use std::{
    future::Future,
    pin::Pin,
    sync::mpsc,
    task::{Context, Poll},
    thread,
    time::Duration,
};

/// A future that completes after the specified duration
pub struct DelayFuture {
    duration: Duration,
    rx: mpsc::Receiver<()>,
}

impl DelayFuture {
    pub fn new(duration: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            thread::sleep(duration);
            let _ = tx.send(());
        });

        Self { duration, rx }
    }
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.rx.try_recv() {
            Ok(()) => Poll::Ready(()),
            Err(mpsc::TryRecvError::Empty) => {
                // Register waker for when the duration elapses
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(mpsc::TryRecvError::Disconnected) => Poll::Ready(()),
        }
    }
}

/// Creates a future that completes after the specified duration
pub fn delay(ms: u64) -> DelayFuture {
    DelayFuture::new(Duration::from_millis(ms))
}