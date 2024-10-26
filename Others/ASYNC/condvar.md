# Condvar (Condition Variable)

- [Condvar (Condition Variable)](#condvar-condition-variable)
  - [Introduction](#introduction)
  - [How it Works](#how-it-works)
  - [Summary](#summary)

## Introduction

A Condvar (Condition Variable) is a low-level synchronization primitive that enables threads to wait for specific conditions to be met, often in conjunction with a Mutex.

## How it Works

- Condition Variables provide the ability to wait on a signal while releasing a lock. This allows other threads to modify the data protected by the mutex, after which the waiting thread can be awakened.

- Notifying and Waiting:
  - wait: A thread holding a Mutex lock can call wait on a Condvar. This releases the lock and puts the thread to sleep atomically, ensuring that no data races occur while waiting.
  - notify_one and notify_all: Other threads can call notify_one (to wake one waiting thread) or notify_all (to wake all waiting threads) on the Condvar. Once notified, the waiting thread(s) reacquire the lock and proceed.

Example: Using Condvar for Producer-Consumer Pattern

```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let producer = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        thread::sleep(Duration::from_millis(1000)); // simulate work
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one(); // notify the waiting thread
    });

    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap(); // wait until notified
        }
        println!("Condition met, proceeding!");
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
```

In this example, Condvar is used to make a consumer wait until the producer has finished work, with wait releasing the lock while sleeping. The producer then calls notify_one to wake the consumer.

## Summary

Condvar: A primitive for conditional waiting, often paired with Mutex. Useful for signaling events or waiting for conditions.
