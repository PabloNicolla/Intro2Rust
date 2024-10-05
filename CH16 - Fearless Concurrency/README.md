# Rust CH16

- [Rust CH16](#rust-ch16)
  - [Fearless Concurrency](#fearless-concurrency)
    - [`thread::spawn`](#threadspawn)
      - [`join` Handles](#join-handles)
      - [Threads and Closures](#threads-and-closures)
  - [Message Passing to Transfer Data Between Threads](#message-passing-to-transfer-data-between-threads)
    - [Sending Multiple Values and Seeing the Receiver Waiting](#sending-multiple-values-and-seeing-the-receiver-waiting)
    - [Creating Multiple Producers by Cloning the Transmitter](#creating-multiple-producers-by-cloning-the-transmitter)
    - [mpsc::sync\_channel vs. mpsc::channel](#mpscsync_channel-vs-mpscchannel)
  - [Shared-State Concurrency](#shared-state-concurrency)
    - [Mutex](#mutex)
  - [Extensible Concurrency](#extensible-concurrency)
    - [`Send` Trait](#send-trait)
    - [`Sync` Trait](#sync-trait)
    - [Implementing Send and Sync Manually Is Unsafe](#implementing-send-and-sync-manually-is-unsafe)

## Fearless Concurrency

- **concurrent programming**, where different parts of a program execute independently
- **parallel programming**, where different parts of a program execute at the same time

### `thread::spawn`

> [!WARNING]
> When the **main thread** of a Rust program completes, **all spawned threads are shut down**, whether or not they have finished running

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

#### `join` Handles 

- Waiting for Threads to Finish

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

#### Threads and Closures

- The following code would not work without passing ownership of `v` to the spawned thread
- since rust has no way to know when the thread will access `v`

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```

- The move keyword overrides Rust’s conservative default of borrowing

## Message Passing to Transfer Data Between Threads

- channel in programming is a directional tunnel of communication that transfers a message from A -> B

- A channel has two halves: a transmitter and a receiver
  - **Tasimeter** is the one that sends the messages
  - **Receiver** is the one that receives the messages

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();          // returns a Result<T, E> type, so if the receiver has already been dropped and
    });                                 // there’s nowhere to send a value, the send operation will return an error.

    let received = rx.recv().unwrap();  // .recv() will block the main thread’s execution and wait until a value is sent down
    println!("Got: {received}");        // .try_recv() doesn’t block, but will instead return a Result<T, E> immediately
}
```

- We create a new channel using the `mpsc::channel` function
- **mpsc** stands for **multiple producer, single consumer**.
  - Can have multiple sending ends
  - but only one receiving end

- `mpsc::channel` function returns a tuple which is deconstructed to
  - `tx` : transmitter
  - `rx` : receiver

- the spawned thread needs to own the transmitter, then the use of `move`

### Sending Multiple Values and Seeing the Receiver Waiting

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
```

outcome

```text
Got: hi
Got: from
Got: the
Got: thread
```

### Creating Multiple Producers by Cloning the Transmitter

```rust
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // --snip--
```

### mpsc::sync_channel vs. mpsc::channel

- `mpsc::channel`
  - **Unbuffered channels:** The sender (tx) will block on .send() until the receiver (rx) reads the message.

- `mpsc::sync_channel`
  - **Buffered channels:** The sender will block only if the buffer is full. If there's space in the buffer, the sender can send multiple messages without waiting for the receiver to read each one immediately.

## Shared-State Concurrency

### Mutex

- is an abbreviation for **mutual exclusion**

- two rules:
  - You must attempt to **acquire the lock** before using the data.
  - **When you’re done with the data** that the mutex guards, you must **unlock the data** so other threads can acquire the lock.

- the smart pointer returned by `mutex.lock()` implements `Deref`
  - meaning, the lock is released automatically when the smart points goes out of scope

- `Mutex<T>` provides interior mutability similar to `RefCell<T>`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));    // Arc is similar to Rc but thread safe
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

## Extensible Concurrency

- `std::marker` traits `Sync` and `Send`

### `Send` Trait

- The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be **transferred between threads**.
- Almost every Rust type is `Send`, but there are some exceptions, including `Rc<T>`
  - Where there is the risk of two threads trying to update the reference count at the same time

### `Sync` Trait

- The `Sync` marker trait **indicates that it is safe** for the type implementing `Sync` to be **referenced from multiple threads**.

- In other words, any type `T` is `Sync` if `&T` (an immutable reference to T) is `Send`, meaning the reference can be sent safely to another thread.
- Similar to `Send`, primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`.

### Implementing Send and Sync Manually Is Unsafe

- Because types that are made up of `Send` and `Sync` traits are automatically also `Send` and `Sync`, we don’t have to implement those traits manually.
