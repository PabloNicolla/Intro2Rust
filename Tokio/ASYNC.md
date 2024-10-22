# ASYNC

- [ASYNC](#async)
  - [Introduction](#introduction)
  - [Using `async/await`](#using-asyncawait)

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
