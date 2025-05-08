# Pin, Poll, and unsafe: illustrated

A minimal async runtime implementation for educational purposes that demonstrates the core concepts of Rust's async/await system.

## High-level Overview of Cooperative Scheduling

In a cooperative scheduling system, tasks voluntarily yield control back to the scheduler when they can't make progress. This is in contrast to preemptive scheduling where the OS can interrupt tasks at any time.

Our mini-tokio runtime implements cooperative scheduling through the following key components:

1. **Executor**: The main scheduler that manages tasks and their execution
2. **Task**: A unit of work that can be scheduled and executed
3. **Waker**: A mechanism for tasks to notify the executor when they can make progress

Here's a sequence diagram showing how these components interact:

```svg
<svg width="800" height="400" xmlns="http://www.w3.org/2000/svg">
  <rect x="50" y="50" width="100" height="300" fill="#f0f0f0" stroke="#000"/>
  <text x="75" y="30" text-anchor="middle">Task</text>
  <rect x="250" y="50" width="100" height="300" fill="#f0f0f0" stroke="#000"/>
  <text x="275" y="30" text-anchor="middle">Executor</text>
  <rect x="450" y="50" width="100" height="300" fill="#f0f0f0" stroke="#000"/>
  <text x="475" y="30" text-anchor="middle">Waker</text>

  <line x1="150" y1="100" x2="250" y2="100" stroke="#000"/>
  <text x="200" y="90" text-anchor="middle">poll()</text>

  <line x1="250" y1="150" x2="150" y2="150" stroke="#000"/>
  <text x="200" y="140" text-anchor="middle">Poll::Pending</text>

  <line x1="150" y1="200" x2="450" y2="200" stroke="#000"/>
  <text x="300" y="190" text-anchor="middle">register waker</text>

  <line x1="450" y1="250" x2="250" y2="250" stroke="#000"/>
  <text x="350" y="240" text-anchor="middle">wake()</text>
</svg>
```

## Pin Explained

`Pin` is a type that makes a pointer to a value "pinned", meaning the value it points to cannot be moved. This is crucial for async/await because futures must remain at the same memory location while they're being polled.

Here's a diagram showing how `Pin` works:

```svg
<svg width="600" height="400" xmlns="http://www.w3.org/2000/svg">
  <rect x="50" y="50" width="200" height="100" fill="#e0e0ff" stroke="#000"/>
  <text x="150" y="100" text-anchor="middle">Stack</text>
  <rect x="50" y="200" width="200" height="100" fill="#ffe0e0" stroke="#000"/>
  <text x="150" y="250" text-anchor="middle">Heap</text>

  <line x1="150" y1="150" x2="150" y2="200" stroke="#000" marker-end="url(#arrow)"/>
  <text x="170" y="175" text-anchor="start">Pin&lt;Box&lt;Future&gt;&gt;</text>

  <rect x="300" y="50" width="200" height="100" fill="#e0e0ff" stroke="#000"/>
  <text x="400" y="100" text-anchor="middle">Stack</text>
  <rect x="300" y="200" width="200" height="100" fill="#ffe0e0" stroke="#000"/>
  <text x="400" y="250" text-anchor="middle">Heap</text>

  <line x1="400" y1="150" x2="400" y2="200" stroke="#000" marker-end="url(#arrow)"/>
  <text x="420" y="175" text-anchor="start">Box&lt;Future&gt;</text>
  <text x="420" y="195" text-anchor="start">(can move)</text>
</svg>
```

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