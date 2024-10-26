# The parking_lot Crate

- [The parking\_lot Crate](#the-parking_lot-crate)
  - [Introduction](#introduction)
  - [Benefits of parking\_lot](#benefits-of-parking_lot)
  - [Key Primitives in parking\_lot](#key-primitives-in-parking_lot)
  - [MutexGuard Scope-Limited Locking](#mutexguard-scope-limited-locking)
  - [Summary](#summary)

## Introduction

The parking_lot crate is a highly optimized, lock-free synchronization library providing more efficient and flexible implementations of common primitives like Mutex, RwLock, and Condvar.

## Benefits of parking_lot

- Performance: parking_lot uses efficient thread parking mechanisms to handle contention, often outperforming std::sync.
- Simpler API: Many of parking_lot's types, such as Mutex, do not require the PoisonError handling of std::sync.
- Additional Features: Provides features like Mutex timeouts and FairMutex, which aren’t available in the standard library.

## Key Primitives in parking_lot

- Mutex: parking_lot::Mutex is a drop-in replacement for std::sync::Mutex but with improved performance and fairness. It also avoids poisoning, so a panic in one thread doesn’t prevent others from accessing the mutex.

- RwLock: parking_lot::RwLock offers reader-writer locking with better read performance and flexibility.

- Condvar: parking_lot::Condvar is an optimized condition variable similar to std::sync::Condvar, with faster wakeup times.

Example: Using parking_lot::Mutex

```rust
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut num = counter.lock();
                *num += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock());
}
```

This example is similar to the standard Mutex, but parking_lot's Mutex typically shows better performance under contention.
Using parking_lot’s Additional Features

- Timeouts: parking_lot supports timeout-based locks, e.g., try_lock_for(Duration::from_secs(1)).

- Fair Mutex: In parking_lot, you can enable fair locking with parking_lot::FairMutex, which helps prevent thread starvation by ensuring that threads acquire the lock in the order they requested it.

## MutexGuard Scope-Limited Locking

In parking_lot, you can extend or reduce the scope of a MutexGuard, which controls the lifetime of the lock. The API allows creating RwLockUpgradableReadGuard and other flexible guards for controlled access.

## Summary

parking_lot: A high-performance crate providing alternative synchronization primitives like Mutex, RwLock, and Condvar, with additional features like timeout locks and fair locks.
