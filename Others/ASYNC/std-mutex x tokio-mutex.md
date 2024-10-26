# std-mutex x tokio-mutex

- [std-mutex x tokio-mutex](#std-mutex-x-tokio-mutex)
  - [Introduction](#introduction)
    - [1. Overview of `std::sync::Mutex` and `tokio::sync::Mutex`](#1-overview-of-stdsyncmutex-and-tokiosyncmutex)
    - [2. Performance Considerations](#2-performance-considerations)
    - [3. When to Use `std::sync::Mutex`](#3-when-to-use-stdsyncmutex)
    - [4. When to Use `tokio::sync::Mutex`](#4-when-to-use-tokiosyncmutex)
    - [5. Practical Differences and Examples](#5-practical-differences-and-examples)
    - [6. Best Practices and Tips](#6-best-practices-and-tips)
    - [Summary](#summary)

## Introduction

When working with **Tokio** in Rust, choosing between `std::sync::Mutex` and `tokio::sync::Mutex` is crucial for ensuring thread safety and performance in asynchronous contexts. Here’s an in-depth look at when and why to use each:

### 1. Overview of `std::sync::Mutex` and `tokio::sync::Mutex`

- **`std::sync::Mutex`**:
  - Part of the standard library.
  - A **blocking** mutex: when a thread locks it, other threads or tasks waiting to acquire the lock are blocked until it becomes available.
  - Suitable for **synchronous code** or code that does not need to await on futures.
  - Does not work well with asynchronous runtimes like Tokio, as blocking a thread can lead to deadlocks and wasted resources.

- **`tokio::sync::Mutex`**:
  - Specifically designed for the **Tokio asynchronous runtime**.
  - An **async-aware, non-blocking** mutex: rather than blocking the entire thread, tasks are **suspended** until the mutex becomes available.
  - Efficient in an asynchronous environment, allowing other tasks to continue running on the same thread while waiting for the lock.
  - Compatible with `async/await` syntax, making it the preferred choice in asynchronous code.

### 2. Performance Considerations

In asynchronous code, particularly with Tokio, the goal is to avoid blocking the runtime’s threads. Tokio’s runtime typically uses a **small number of threads** (often a fixed number, depending on your system’s resources). Blocking these threads with `std::sync::Mutex` would prevent other asynchronous tasks from running, potentially stalling the entire program. 

`tokio::sync::Mutex`, on the other hand, avoids blocking the runtime's threads, making it a better fit for asynchronous workloads, especially those involving I/O-bound or heavily concurrent tasks.

### 3. When to Use `std::sync::Mutex`

`std::sync::Mutex` is appropriate if:
- You are working in a **synchronous, non-async context** (i.e., code that doesn’t require `await` or rely on futures).
- **The data is accessed in blocking contexts only**, or within an async context that doesn’t involve Tokio’s runtime.
- **Lock contention is low** (e.g., if the code is unlikely to have high concurrent access), so that blocking is rare.
  
Common use cases:
- A **background thread** that runs synchronous code and only occasionally needs to share state with other threads.
- **CPU-bound** operations that are not interwoven with asynchronous tasks, e.g., computational tasks that don’t involve I/O and are isolated from async tasks.

### 4. When to Use `tokio::sync::Mutex`

`tokio::sync::Mutex` is the recommended choice if:
- You are working within **Tokio’s asynchronous context** where multiple `async` tasks share data.
- The code will involve **I/O-bound** tasks (like network requests) where tasks should run concurrently without blocking.
- The shared data will be accessed **frequently and concurrently** by asynchronous tasks, as `tokio::sync::Mutex` allows tasks to yield control instead of blocking.

Common use cases:
- **Async web servers** that need to handle multiple requests and access shared data structures.
- **Stateful async applications** where shared data needs to be modified by multiple tasks without blocking others.
- **Database access layers** that may involve locking a shared connection pool in an async function.

### 5. Practical Differences and Examples

- **CPU-Bound Tasks (use `std::sync::Mutex`)**:
  ```rust
  use std::sync::{Arc, Mutex};
  use std::thread;

  fn main() {
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];

      for _ in 0..10 {
          let counter = Arc::clone(&counter);
          let handle = thread::spawn(move || {
              let mut num = counter.lock().unwrap();
              *num += 1;
          });
          handles.push(handle);
      }

      for handle in handles {
          handle.join().unwrap();
      }

      println!("Result: {}", *counter.lock().unwrap());
  }
  ```
  This example is synchronous, does not involve async I/O, and performs well with `std::sync::Mutex`.

- **Async I/O-Bound Tasks (use `tokio::sync::Mutex`)**:
  ```rust
  use std::sync::Arc;
  use tokio::sync::Mutex;
  use tokio::task;

  #[tokio::main]
  async fn main() {
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];

      for _ in 0..10 {
          let counter = Arc::clone(&counter);
          let handle = task::spawn(async move {
              let mut num = counter.lock().await;
              *num += 1;
          });
          handles.push(handle);
      }

      for handle in handles {
          handle.await.unwrap();
      }

      println!("Result: {}", *counter.lock().await);
  }
  ```
  Here, `tokio::sync::Mutex` allows each task to yield if it cannot acquire the lock, enabling other tasks to make progress.

### 6. Best Practices and Tips

1. **Avoid Long Locks in Async Code**: Try to keep the locked scope as short as possible, to reduce waiting times.
2. **Consider Other Sync Primitives**: For highly concurrent data structures, explore `tokio::sync::RwLock` (if mostly reads) or atomic operations.
3. **Be Mindful of `Send` and `Sync` Requirements**: Using `tokio::sync::Mutex` in asynchronous functions requires `Send` and `Sync`, especially if tasks are spawned across threads.
4. **Understand Task Suspension vs Blocking**: Task suspension (with `tokio::sync::Mutex`) is more resource-efficient than blocking, making it suitable for high-concurrency async applications.

### Summary

- Use `std::sync::Mutex` for **blocking, synchronous contexts** and **CPU-bound tasks**.
- Use `tokio::sync::Mutex` for **non-blocking, asynchronous tasks**, especially when using Tokio's runtime or dealing with I/O-bound operations.
  
By using `tokio::sync::Mutex` in async tasks, you avoid blocking threads unnecessarily and allow other async tasks to run concurrently, making your application more efficient in async contexts.
