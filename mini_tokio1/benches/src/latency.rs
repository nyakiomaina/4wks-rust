use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam::channel;
use mini_tokio::Executor;
use std::sync::mpsc;

fn ping_pong_mini_tokio() {
    let executor = Executor::new();
    let (tx, rx) = mpsc::channel::<u32>();

    executor.block_on(async {
        let (tx1, rx1) = channel::bounded(1);
        let (tx2, rx2) = channel::bounded(1);

        let handle1 = executor.spawn(async move {
            for i in 0..1000 {
                tx1.send(i).unwrap();
                let _ = rx2.recv().unwrap();
            }
        });

        let handle2 = executor.spawn(async move {
            for _ in 0..1000 {
                let _ = rx1.recv().unwrap();
                tx2.send(()).unwrap();
            }
        });

        handle1.await.unwrap();
        handle2.await.unwrap();
    });
}

fn ping_pong_tokio() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let (tx1, mut rx1) = mpsc::channel();
        let (tx2, mut rx2) = mpsc::channel();

        let handle1 = tokio::spawn(async move {
            for i in 0..1000 {
                tx1.send(i).unwrap();
                let _ = rx2.recv().unwrap();
            }
        });

        let handle2 = tokio::spawn(async move {
            for _ in 0..1000 {
                let _ = rx1.recv().unwrap();
                tx2.send(()).unwrap();
            }
        });

        handle1.await.unwrap();
        handle2.await.unwrap();
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ping-pong");

    group.bench_function("mini_tokio", |b| b.iter(|| ping_pong_mini_tokio()));
    group.bench_function("tokio", |b| b.iter(|| ping_pong_tokio()));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);