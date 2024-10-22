# Shared State

- [Shared State](#shared-state)
  - [Introduction](#introduction)
  - [On using `std::sync::Mutex` and `tokio::sync::Mutex`](#on-using-stdsyncmutex-and-tokiosyncmutex)
  - [Holding a `MutexGuard` across an `.await`](#holding-a-mutexguard-across-an-await)
    - [Restructure your code to not hold the lock across an `.await`](#restructure-your-code-to-not-hold-the-lock-across-an-await)
  - [Use Tokio's asynchronous `mutex`](#use-tokios-asynchronous-mutex)
  - [Tasks, threads, and contention](#tasks-threads-and-contention)
    - [Mutex sharding Example](#mutex-sharding-example)

## Introduction

Strategies

There are a couple of different ways to share state in Tokio.

- Guard the shared state with a `Mutex`.
- Spawn a task to manage the state and use message passing to operate on it.

Generally you want to use the first approach for simple data, and the second approach for things that require asynchronous work such as I/O primitives.

## On using `std::sync::Mutex` and `tokio::sync::Mutex`

Note that `std::sync::Mutex` and not `tokio::sync::Mutex` is used to guard the `HashMap`. A common error is to unconditionally use `tokio::sync::Mutex` from within async code. An async mutex is a mutex that is locked across calls to `.await`.

A synchronous mutex will block the current thread when waiting to acquire the lock. This, in turn, will block other tasks from processing. However, switching to `tokio::sync::Mutex` usually does not help as the asynchronous mutex uses a synchronous mutex internally.

As a rule of thumb, using a synchronous mutex from within asynchronous code is fine as long as contention remains low and the lock is not held across calls to `.await`.

## Holding a `MutexGuard` across an `.await`

> [!WARNING]
> The following two examples do not compile

```rust
use std::sync::{Mutex, MutexGuard};

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;

    do_something_async().await;
} // lock goes out of scope here
```

```rust
use std::sync::{Mutex, MutexGuard};

// This fails too.
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
    *lock += 1;
    drop(lock);

    do_something_async().await;
}
```

---

- This happens because the `std::sync::MutexGuard` type is not `Send`.
- **Tokio runtime can move a task between threads** at every `.await`.

```rust
// This works!
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
}
```

> [!WARNING]
> Circumventing this issue can cause **deadlocks**

### Restructure your code to not hold the lock across an `.await`

The safest way to handle a mutex is to wrap it in a struct, and lock the mutex only inside non-async methods on that struct.

```rust
use std::sync::Mutex;

struct CanIncrement {
    mutex: Mutex<i32>,
}
impl CanIncrement {
    // This function is not marked async.
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}
```

This pattern guarantees that you won't run into the `Send` error, because the mutex guard does not appear anywhere in an async function. It also protects you from **deadlocks**, when using crates whose `MutexGuard` implements `Send`.

You can find a more detailed example in this [blog post](https://draft.ryhl.io/blog/shared-mutable-state/).

## Use Tokio's asynchronous `mutex`

This is the second approach mentioned in the start of this chapter, and is often used when the shared resource is an I/O resource.

The `tokio::sync::Mutex` type provided by Tokio can also be used. The primary feature of the Tokio mutex is that it can be held across an `.await` without any issues. That said, an asynchronous mutex is more expensive than an ordinary mutex, and it is typically better to use one of the two other approaches.

```rust
use tokio::sync::Mutex; // note! This uses the Tokio mutex

// This compiles!
// (but restructuring the code would be better in this case)
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;

    do_something_async().await;
} // lock goes out of scope here
```

## Tasks, threads, and contention

Using a blocking mutex to guard short critical sections is an acceptable strategy when contention is minimal. When a lock is contended, the thread executing the task must block and wait on the mutex. This will not only block the current task but it will also block all other tasks scheduled on the current thread.

If contention on a synchronous mutex becomes a problem, the best fix is rarely to switch to the Tokio mutex. Instead, options to consider are to:

- Let a dedicated task manage state and use message passing.
- Shard the mutex.
- Restructure the code to avoid the mutex.

### Mutex sharding Example

```rust
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
```

Then, finding the cell for any given key becomes a two step process. First, the key is used to identify which shard it is part of. Then, the key is looked up in the HashMap.

```rust
let shard = db[hash(key) % db.len()].lock().unwrap();
shard.insert(key, value);
```