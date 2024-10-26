# Once

- [Once](#once)
  - [Introduction](#introduction)
  - [Key Features](#key-features)
  - [Summary](#summary)

## Introduction

The Once primitive is a useful tool for running an operation exactly once in a concurrent environment, even if multiple threads call it simultaneously. It’s typically used for initializing shared resources, such as global variables.

## Key Features

- One-Time Execution: Once runs the provided closure exactly once, ensuring that all threads see the initialized result, regardless of which thread called it.

- call_once and call_once_force:
  - call_once: Runs a closure exactly once. Subsequent calls to call_once return immediately.
  - call_once_force: Similar to call_once, but can forcefully retry the closure if the previous attempt panicked.

Example: Using Once for Initialization

```rust
use std::sync::{Arc, Once};
use std::thread;

static INIT: Once = Once::new();
static mut GLOBAL_RESOURCE: Option<usize> = None;

fn initialize() {
    unsafe {
        GLOBAL_RESOURCE = Some(42); // arbitrary initialization
    }
}

fn main() {
    let threads: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                INIT.call_once(|| initialize());
                unsafe {
                    println!("GLOBAL_RESOURCE: {:?}", GLOBAL_RESOURCE);
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }
}
```

In this example, Once ensures that initialize runs only once, no matter how many threads attempt to initialize GLOBAL_RESOURCE.

## Summary

Once: Ensures a one-time execution of an initialization function, useful for setting up resources like global data.
