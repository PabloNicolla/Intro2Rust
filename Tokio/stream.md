# Tokio Stream

- [Tokio Stream](#tokio-stream)
  - [Introduction](#introduction)
  - [Key Concepts](#key-concepts)
  - [Pinning](#pinning)
  - [Mini-Redis Example](#mini-redis-example)
  - [Stream Adapters](#stream-adapters)
  - [Stream Implementations](#stream-implementations)
  - [Async-stream Crate](#async-stream-crate)
  - [Conclusion](#conclusion)

## Introduction

The tokio-stream crate offers a variety of utilities to work with asynchronous streams in Rust, similar to how the Iterator trait operates for synchronous values. The Stream trait represents this asynchronous sequence of values and can be transformed using adapters like filter, map, take, etc., provided by StreamExt.

## Key Concepts

- Streams:
  - A stream is an asynchronous sequence of values, similar to an iterator but with support for async operations.
  - StreamExt::next() retrieves the next value from the stream, returning Some(T) or None when the stream terminates.

- Stream Iteration:
  - Since Rust does not have async for loops, stream iteration is done using while let with StreamExt::next().
  - Example:

```rust
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}
```

## Pinning

- Pinning a value means it cannot be moved in memory, which is important when working with streams and async functions.
- Streams must be pinned before using them in .await loops, like in this example:

```rust
tokio::pin!(stream);
while let Some(v) = stream.next().await {
    // ...
}
```

## Mini-Redis Example

- A Redis client can use streams for real-time message handling.
- Example: A task is spawned to publish messages, and the main task subscribes to a Redis channel using streams, iterating over incoming messages asynchronously.

## Stream Adapters

- Functions like filter, take, and map allow transforming streams. These are similar to iterator adapters but work asynchronously.
- Example of limiting and filtering messages from Redis:

```rust
let messages = subscriber
    .into_stream()
    .filter(|msg| msg.is_ok() && msg.unwrap().content.len() == 1)
    .take(3);
```

## Stream Implementations

- Streams can be manually implemented by implementing the Stream trait and handling polling.
- Example:

```rust
impl Stream for MyStream {
    type Item = ...;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Stream logic here
    }
}
```

## Async-stream Crate

- The async-stream crate provides a macro to simplify creating streams using async/await syntax.
- Example:

```rust
use async_stream::stream;

stream! {
    for i in 0..3 {
        yield i;
    }
}
```

## Conclusion

Streams are powerful for managing asynchronous data flows, especially in scenarios like networking, where data arrives in bursts and needs to be processed incrementally over time.
