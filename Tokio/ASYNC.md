# ASYNC

- [ASYNC](#async)
  - [Introduction](#introduction)
  - [Using `async/await`](#using-asyncawait)
  - [Tasks and Spawn](#tasks-and-spawn)
    - [`'static` bound](#static-bound)
    - [`Send` bound](#send-bound)

## Introduction

Rust implements asynchronous programming using a feature called `async/await`.
Functions that perform asynchronous operations are labeled with the `async` keyword

```rust
use mini_redis::Result;
use mini_redis::client::Client;
use tokio::net::ToSocketAddrs;

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client> {
    // ...
}
```

Rust transforms the async fn at compile time into a routine that operates asynchronously.

## Using `async/await`

Async functions are called like any other Rust function. However, calling these functions does not result in the function body executing. Instead, calling an `async fn` returns a value representing the operation.

**To actually run the operation**, you should use the `.await` operator on the return value.

```rust
async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}
```

The return value of an async fn is an anonymous type that implements the `Future` trait.

## Tasks and Spawn

A Tokio task is an asynchronous green thread. They are created by passing an `async` block to `tokio::spawn`. The `tokio::spawn` function returns a `JoinHandle`, which the caller may use to interact with the spawned task. The `async` block may have a return value. The caller may obtain the return value using `.await` on the `JoinHandle`.

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}
```

Awaiting on `JoinHandle` returns a `Result`. When a task encounters an error during execution, the `JoinHandle` will return an `Err`. This happens when the task either panics, or if the task is forcefully cancelled by the runtime shutting down.

### `'static` bound

When you spawn a task on the Tokio runtime, its type's lifetime must be `'static`. This means that the spawned task must not contain any references to data owned outside the task.

### `Send` bound

Tasks spawned by `tokio::spawn` must implement `Send`. This allows the Tokio runtime to move the tasks between threads while they are suspended at an `.await`.

this works:

```rust
use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}
```

This does not:

```rust
use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let rc = Rc::new("hello");

        // `rc` is used after `.await`. It must be persisted to
        // the task's state.
        yield_now().await;

        println!("{}", rc);
    });
}
```
