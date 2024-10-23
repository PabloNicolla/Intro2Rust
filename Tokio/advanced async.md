# Async in Depth

- [Async in Depth](#async-in-depth)
  - [Introduction](#introduction)
  - [Conclusion](#conclusion)

## Introduction

1. Futures in Rust

- Futures in Rust differ from other languages as they represent computations that must be explicitly advanced through polling. Rust's Future trait defines this, where the future is the computation itself.
- Example: async functions return futures, and calling .await on these futures waits for them to complete.

2. Implementing the Future Trait

- Demonstrates implementing a custom future that outputs text after a specified delay.
- Explains how poll() is used to advance the future's state and handle when the computation is ready or pending.

3. Async Functions as State Machines

- Rust futures are state machines. For example, an async fn is compiled into an enum that represents various states of the future, transitioning from State0 (initial state) to Terminated.
- Polling moves the future between states until completion.

4. Executors in Async Rust

- Executors (like Tokio) are responsible for polling futures and driving them to completion. Futures submitted to an executor are run asynchronously.

5. Building a Mini Tokio Executor

- A step-by-step guide to building a simplified version of the Tokio executor, demonstrating how futures can be scheduled, polled, and executed.
- The concept of wakers is introduced to notify when futures are ready to make progress, preventing continuous polling.

6. Wakers and Task Scheduling

- Wakers are used to notify the executor when a task can proceed. The document explains how to implement wakers and handle task scheduling in an executor.
- Provides examples using the futures crate for task management and polling.

7. Handling Polling and Spurious Wake-ups

- Explains how to manage scenarios where a future might be polled multiple times (spurious wake-ups), ensuring proper task scheduling and resource management.

8. Waker Synchronization

- Emphasizes the importance of ensuring the correct Waker is used for a future, especially when tasks migrate between executors.

9. Notify Utility

- Introduces the Notify utility from Tokio, which simplifies implementing task notifications without manually dealing with wakers.

10. Practical Example: Delay Future

- Provides a complete example of implementing a custom Delay future that waits for a specified duration using both manual waker handling and Notify utility from Tokio.

## Conclusion

- Asynchronous Rust is built on top of traits like Future and involves an explicit polling mechanism.
- Executors manage task execution by polling futures, and wakers notify when tasks can make progress.
- Rust's async model ensures efficient handling of resources and supports advanced customization, as demonstrated with examples of futures, executors, and wakers.
