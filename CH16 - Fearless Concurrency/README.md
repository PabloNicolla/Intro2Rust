# Rust CH16

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

