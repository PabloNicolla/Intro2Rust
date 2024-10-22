# Channels

- [Channels](#channels)
  - [Introduction](#introduction)
    - [mpsc example](#mpsc-example)
    - [oneshot](#oneshot)
  - [Backpressure and bounded channels](#backpressure-and-bounded-channels)

## Introduction

Tokio provides a number of channels, each serving a different purpose.

- `mpsc`: multi-producer, single-consumer channel. Many values can be sent.
- `oneshot`: single-producer, single consumer channel. A single value can be sent.
- `broadcast`: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
- `watch`: multi-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.

If you need a multi-producer multi-consumer channel where only one consumer sees each message, you can use the `async-channel` crate. There are also channels for use outside of asynchronous Rust, such as `std::sync::mpsc` and `crossbeam::channel`. These channels wait for messages by blocking the thread, which is not allowed in asynchronous code.

### mpsc example

The channel is created with a capacity of 32. If messages are sent faster than they are received, the channel will store them. Once the 32 messages are stored in the channel, calling send(...).await will go to sleep until a message has been removed by the receiver.

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
```

Both messages are sent to the single Receiver handle. It is not possible to clone the receiver of an mpsc channel.

When every Sender has gone out of scope or has otherwise been dropped, it is no longer possible to send more messages into the channel. At this point, the recv call on the Receiver will return None, which means that all senders are gone and the channel is closed.

### oneshot

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();
```

Unlike mpsc, no capacity is specified as the capacity is always one. Additionally, neither handle can be cloned.

## Backpressure and bounded channels

Whenever concurrency or queuing is introduced, it is important to ensure that the queing is bounded and the system will gracefully handle the load. Unbounded queues will eventually fill up all available memory and cause the system to fail in unpredictable ways.

Tokio takes care to avoid implicit queuing. A big part of this is the fact that async operations are lazy. Consider the following:

```rust
loop {
    async_op();
}
```

If the asynchronous operation runs eagerly, the loop will repeatedly queue a new `async_op` to run without ensuring the previous operation completed. This results in implicit unbounded queuing. Callback based systems and eager future based systems are particularly susceptible to this.

However, with Tokio and asynchronous Rust, the above snippet will not result in `async_op` running at all. This is because `.await` is never called. If the snippet is updated to use `.await`, then the loop waits for the operation to complete before starting over.

```rust
loop {
    // Will not repeat until `async_op` completes
    async_op().await;
}
```

Concurrency and queuing must be explicitly introduced. Ways to do this include:

- `tokio::spawn`
- `select!`
- `join!`
- `mpsc::channel`

When doing so, take care to ensure the total amount of concurrency is bounded. For example, when writing a TCP accept loop, ensure that the total number of open sockets is bounded. When using `mpsc::channel`, pick a manageable channel capacity. Specific bound values will be application specific.

Taking care and picking good bounds is a big part of writing reliable Tokio applications.