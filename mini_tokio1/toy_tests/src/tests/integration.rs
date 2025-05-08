use mini_tokio::{delay, Executor};
use std::time::{Duration, Instant};

#[test]
fn completes_simple_future() {
    let executor = Executor::new();
    let handle = executor.spawn(async { 42 });
    let result = executor.block_on(handle);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn join_two_tasks() {
    let executor = Executor::new();
    let handle1 = executor.spawn(async { 21 });
    let handle2 = executor.spawn(async { 21 });

    let result = executor.block_on(async {
        let a = handle1.await.unwrap();
        let b = handle2.await.unwrap();
        a + b
    });

    assert_eq!(result, 42);
}

#[test]
fn delay_future_sleeps() {
    let executor = Executor::new();
    let start = Instant::now();

    executor.block_on(async {
        delay(50).await;
    });

    let elapsed = start.elapsed();
    assert!(elapsed >= Duration::from_millis(50));
}

#[test]
fn nested_spawn() {
    let executor = Executor::new();

    let result = executor.block_on(async {
        let outer = executor.spawn(async {
            let inner = executor.spawn(async { 21 });
            inner.await.unwrap() * 2
        });

        outer.await.unwrap()
    });

    assert_eq!(result, 42);
}

#[test]
fn cancel_handle() {
    let executor = Executor::new();
    let handle = executor.spawn(async {
        delay(100).await;
        42
    });

    // Drop the handle before the task completes
    drop(handle);

    // Give the task time to complete
    std::thread::sleep(Duration::from_millis(150));

    // The executor should still be running
    let result = executor.block_on(async { 42 });
    assert_eq!(result, 42);
}