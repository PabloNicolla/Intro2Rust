# Async

- [Async](#async)
  - [Introduction](#introduction)
  - [Async in Rust](#async-in-rust)

## Introduction

Rust offers a robust set of tools for handling concurrency and asynchronous operations. Here’s a deep overview covering asynchronous programming and threading, focusing on low-level aspects like atomic operations and memory ordering:

## Async in Rust

Rust async programming allows concurrent tasks without traditional threads, using async/.await syntax and async runtimes like Tokio or async-std. This approach is suitable for I/O-bound tasks, where tasks yield execution to prevent blocking threads.

### Key Concepts

- Future: The core of Rust async programming. A Future represents a value that will be available at some point. The executor polls it, checking if it's ready.

- .await: Syntax for awaiting a Future. When used, it instructs the runtime to check if the Future is ready. If not, it releases the thread for other tasks.

- Executor: Async runtimes (e.g., Tokio, async-std) provide executors that handle the scheduling of asynchronous tasks.

### Task Execution

When using async functions, the executor switches between tasks by suspending tasks that can’t proceed and resuming them when ready. This yields high performance for many I/O-bound tasks without creating a new thread for each one, conserving memory and resources.

## Threads in Rust

Threads in Rust are part of the std::thread module. Each thread has its own stack and runs concurrently with others, ideal for CPU-bound tasks. Rust threads are similar to OS threads but also support message-passing via channels and shared state management through atomic operations.

### Key Concepts

- Spawning Threads: std::thread::spawn creates a new thread that runs a provided function.

- Join Handles: When you spawn a thread, Rust gives a JoinHandle, which can be used to wait for the thread to finish.

- Thread Safety: Rust enforces strict ownership rules to avoid data races, typically managed through Arc (atomic reference counting) and Mutex for sharing data across threads safely.

## Atomics and Memory Ordering

Atomics allow operations on shared data without locks, ideal for low-level concurrency. The std::sync::atomic module provides atomic types like AtomicBool, AtomicUsize, and AtomicPtr, which are safe to share across threads.

### Memory Ordering in Rust

Memory ordering specifies constraints on the visibility of operations. In Rust, memory ordering for atomic operations includes:

- Relaxed: No ordering guarantees, just atomicity. This is the fastest but offers no ordering with other operations. It’s often useful for counters where the exact order isn’t crucial.

- Acquire and Release:
  - Acquire: Ensures prior writes are visible when a load is acquired.
  - Release: Ensures all prior operations are visible to any subsequent acquire loads.
  - These are used together to create a "happens-before" relationship, often used in producer-consumer scenarios.

- AcqRel: Combines both acquire and release guarantees. It’s useful in scenarios where both reading and writing are involved and need visibility constraints in both directions.

- SeqCst (Sequential Consistency): The strictest ordering; it enforces a global order for all operations. This can simplify reasoning about order but may impact performance.

### Common Atomic Operations

- Load and Store: Basic operations to read or set the atomic value with specified memory ordering.
- Compare-and-Swap (CAS): Attempts to update the value only if it matches an expected value, useful for lock-free data structures.
- Fetch Operations (fetch_add, fetch_sub, etc.): Allows modifying the value atomically with arithmetic operations. Often used with Relaxed ordering for counters and statistics.

## Using Atomics with Arc and Mutex

In Rust, Arc (Atomic Reference Counted) and Mutex are frequently combined with atomics for sharing data safely between threads.

- Arc: Provides thread-safe reference counting, allowing shared ownership of data across threads.

- Mutex: Ensures that only one thread can access the data at a time. Rust’s std::sync::Mutex blocks other threads if one holds the lock.

- Arc + Atomic: You can use Arc<AtomicUsize> to create shared counters or flags with no need for locking.

- RwLock: Allows multiple readers or a single writer at one time, useful for data structures that have more reads than writes.

## Choosing Between Async and Threads

Rust's async model is efficient for I/O-bound tasks, while threads are often better suited for CPU-bound tasks.

- I/O-bound tasks: Prefer async with runtimes like Tokio for handling network requests or file operations.
- CPU-bound tasks: Prefer threads for parallel processing and use atomics to minimize locking when accessing shared data.

## Advanced Use Cases with Atomics

For advanced systems like lock-free data structures, Rust atomics paired with custom memory orderings can achieve high concurrency without locks. Examples include:

- Ring Buffers: Using AtomicUsize for pointers with acquire-release semantics to manage concurrent reads and writes.
- State Machines: Leveraging compare_and_swap or compare_exchange for updating states safely and avoiding contention.

## Example: Simple Atomic Counter with Memory Ordering

Here’s a basic example of a shared atomic counter with relaxed ordering.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.load(Ordering::Relaxed));
}
```

This example demonstrates the use of AtomicUsize to increment a shared counter across multiple threads with relaxed ordering for performance.
Summary

Rust provides a unique and robust system for async and concurrent programming, enforcing memory safety and performance. Async is ideal for I/O-bound tasks, while threads paired with atomic operations or locking mechanisms like Arc and Mutex are best for CPU-bound tasks. Through careful memory ordering, Rust enables efficient, low-level concurrency primitives that can scale up to complex systems safely.
