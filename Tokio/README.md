# Tokio Crate

- [Tokio Crate](#tokio-crate)
  - [Introduction](#introduction)
    - [Async `main` function](#async-main-function)
    - [Cargo features](#cargo-features)
  - [Links](#links)

## Introduction

### Async `main` function

The `#[tokio::main]` function is a macro. It transforms the async `fn main()` into a synchronous `fn main()` that initializes a runtime instance and executes the async main function.

### Cargo features

When depending on Tokio for this tutorial, the `full` feature flag is enabled:

```rust
tokio = { version = "1", features = ["full"] }
```

Tokio has a lot of functionality (TCP, UDP, Unix sockets, timers, sync utilities, multiple scheduler types, etc). Not all applications need all functionality. When attempting to optimize compile time or the end application footprint, the application can decide to opt into **only** the features it uses.

## Links

[Tokio](https://tokio.rs/)
