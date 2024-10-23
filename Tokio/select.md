# Tokio select

- [Tokio select](#tokio-select)
  - [Introduction](#introduction)

## Introduction

1. Introduction to select! Macro

- The tokio::select! macro allows for waiting on multiple async computations concurrently.
- It returns when any single computation completes, dropping the others.

2. Example Usage of select!

```rust
tokio::select! {
    val = rx1 => {
        println!("rx1 completed first with {:?}", val);
    }
    val = rx2 => {
        println!("rx2 completed first with {:?}", val);
    }
}
```

- This example waits on two oneshot channels, executing the block corresponding to the first to complete.

3. Cancellation

- Futures in Rust are lazy and can be cancelled by dropping the future.
- tokio::select! can handle tasks with cancellation by using the closed() method of a oneshot receiver.

4. Future Implementation of select!

- A custom Future can simulate the select! behavior.
- Futures need to implement polling, ensuring tasks proceed by signaling the waker when they are ready.

5. Syntax and Branch Matching

- select! can handle up to 64 branches.
- The general structure is:

```rust
<pattern> = <async expression> => <handler>,
```

- Once a branch completes, the remaining branches are cancelled.

6. Error Handling with select!

- The ? operator propagates errors in async expressions or handlers.
- Errors in the async expression are passed to the handler for further propagation.

7. Pattern Matching in select!

- Branches can include any valid Rust patterns, not just variable bindings.
- For example, matching on Some(v) from a channel and handling None indicates a closed channel.

8. Borrowing Data in select!

- The macro allows borrowing data across multiple async expressions, enabling shared state between branches.

9. Using select! in Loops

- Often used in loops to continuously wait on multiple channels.
- Select random branches to prevent any one branch from starving others when there are multiple ready branches.

10. Resuming Async Operations Across select! Calls

- Async operations can be pinned and resumed across multiple calls to select!.
- This approach allows continuing a long-running operation while listening for other events like channel messages.

11. Modifying Branch Behavior in Loops

- You can modify branches to restart or change behavior based on new incoming data (e.g., restarting operations with new inputs).
- Example scenario: restart an async operation with a new even number from the channel.
