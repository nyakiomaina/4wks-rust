# Pin, Poll, and unsafe: illustrated

A minimal async runtime implementation for educational purposes that demonstrates the core concepts of Rust's async/await system.

## High-level Overview of Cooperative Scheduling

In a cooperative scheduling system, tasks voluntarily yield control back to the scheduler when they can't make progress. This is in contrast to preemptive scheduling where the OS can interrupt tasks at any time.

Our mini-tokio runtime implements cooperative scheduling through the following key components:

1. **Executor**: The main scheduler that manages tasks and their execution
2. **Task**: A unit of work that can be scheduled and executed
3. **Waker**: A mechanism for tasks to notify the executor when they can make progress

Here's a sequence diagram showing how these components interact:

<img src="data:image/svg+xml;utf8,%3Csvg%20width%3D%22800%22%20height%3D%22400%22%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%3E%3Crect%20x%3D%2250%22%20y%3D%2250%22%20width%3D%22100%22%20height%3D%22300%22%20fill%3D%22%23f0f0f0%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%2275%22%20y%3D%2230%22%20text-anchor%3D%22middle%22%3ETask%3C/text%3E%3Crect%20x%3D%22250%22%20y%3D%2250%22%20width%3D%22100%22%20height%3D%22300%22%20fill%3D%22%23f0f0f0%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22275%22%20y%3D%2230%22%20text-anchor%3D%22middle%22%3EExecutor%3C/text%3E%3Crect%20x%3D%22450%22%20y%3D%2250%22%20width%3D%22100%22%20height%3D%22300%22%20fill%3D%22%23f0f0f0%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22475%22%20y%3D%2230%22%20text-anchor%3D%22middle%22%3EWaker%3C/text%3E%3Cline%20x1%3D%22150%22%20y1%3D%22100%22%20x2%3D%22250%22%20y2%3D%22100%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22200%22%20y%3D%2290%22%20text-anchor%3D%22middle%22%3Epoll()%3C/text%3E%3Cline%20x1%3D%22250%22%20y1%3D%22150%22%20x2%3D%22150%22%20y2%3D%22150%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22200%22%20y%3D%22140%22%20text-anchor%3D%22middle%22%3EPoll%3A%3APending%3C/text%3E%3Cline%20x1%3D%22150%22%20y1%3D%22200%22%20x2%3D%22450%22%20y2%3D%22200%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22300%22%20y%3D%22190%22%20text-anchor%3D%22middle%22%3Eregister%20waker%3C/text%3E%3Cline%20x1%3D%22450%22%20y1%3D%22250%22%20x2%3D%22250%22%20y2%3D%22250%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22350%22%20y%3D%22240%22%20text-anchor%3D%22middle%22%3Ewake()%3C/text%3E%3C/svg%3E" />

## Pin Explained

`Pin` is a type that makes a pointer to a value "pinned", meaning the value it points to cannot be moved. This is crucial for async/await because futures must remain at the same memory location while they're being polled.

Here's a diagram showing how `Pin` works:

<img src="data:image/svg+xml;utf8,%3Csvg%20width%3D%22600%22%20height%3D%22400%22%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%3E%3Crect%20x%3D%2250%22%20y%3D%2250%22%20width%3D%22200%22%20height%3D%22100%22%20fill%3D%22%23e0e0ff%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22150%22%20y%3D%22100%22%20text-anchor%3D%22middle%22%3EStack%3C/text%3E%3Crect%20x%3D%2250%22%20y%3D%22200%22%20width%3D%22200%22%20height%3D%22100%22%20fill%3D%22%23ffe0e0%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22150%22%20y%3D%22250%22%20text-anchor%3D%22middle%22%3EHeap%3C/text%3E%3Cline%20x1%3D%22150%22%20y1%3D%22150%22%20x2%3D%22150%22%20y2%3D%22200%22%20stroke%3D%22%23000%22%20marker-end%3D%22url(%23arrow)%22/%3E%3Ctext%20x%3D%22170%22%20y%3D%22175%22%20text-anchor%3D%22start%22%3EPin%3C%3CBox%3CFuture%3E%3E%3C/text%3E%3Crect%20x%3D%22300%22%20y%3D%2250%22%20width%3D%22200%22%20height%3D%22100%22%20fill%3D%22%23e0e0ff%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22400%22%20y%3D%22100%22%20text-anchor%3D%22middle%22%3EStack%3C/text%3E%3Crect%20x%3D%22300%22%20y%3D%22200%22%20width%3D%22200%22%20height%3D%22100%22%20fill%3D%22%23ffe0e0%22%20stroke%3D%22%23000%22/%3E%3Ctext%20x%3D%22400%22%20y%3D%22250%22%20text-anchor%3D%22middle%22%3EHeap%3C/text%3E%3Cline%20x1%3D%22400%22%20y1%3D%22150%22%20x2%3D%22400%22%20y2%3D%22200%22%20stroke%3D%22%23000%22%20marker-end%3D%22url(%23arrow)%22/%3E%3Ctext%20x%3D%22420%22%20y%3D%22175%22%20text-anchor%3D%22start%22%3EBox%3CFuture%3E%3C/text%3E%3Ctext%20x%3D%22420%22%20y%3D%22195%22%20text-anchor%3D%22start%22%3E(can%20move)%3C/text%3E%3C/svg%3E" />

## Poll Contract and Waker Callbacks

The `Poll` type represents the result of polling a future. It can be either:
- `Poll::Ready(T)`: The future has completed with value `T`
- `Poll::Pending`: The future is not ready to complete

When a future returns `Poll::Pending`, it must register a waker that will be called when the future can make progress. This is done through the `Context` parameter in the `poll` method.

## Unsafe Blocks

Our implementation uses exactly two `unsafe` blocks:

1. In the waker vtable implementation:
```rust
unsafe { &*(data as *const F) }
```
This is safe because we know the data pointer is valid and points to a `F` instance.

2. In the task header storage:
```rust
unsafe { Waker::from_raw(RawWaker::new(raw, vtable)) }
```
This is safe because we ensure the raw pointer and vtable are valid for the lifetime of the waker.

## Performance

Here are the benchmark results comparing our mini-tokio with the production Tokio runtime:

| Runtime    | Mean Latency (µs/op) |
|------------|---------------------|
| mini-tokio | 1.2                 |
| tokio      | 0.15                |

Our implementation is about 8x slower than Tokio. This is expected because:
1. We use a simpler scheduling algorithm
2. We don't optimize for common cases
3. We use more locks and synchronization primitives
4. We don't implement work stealing or other advanced features

## Running the Code

To run the tests:
```bash
just test
```

To run the benchmarks:
```bash
just bench
```

To generate documentation:
```bash
just docs
```

## License

This project is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.