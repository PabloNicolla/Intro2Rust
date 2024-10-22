# Summary of Framing in Mini-Redis

- [Summary of Framing in Mini-Redis](#summary-of-framing-in-mini-redis)
  - [What is Framing?](#what-is-framing)
  - [HTTP frames follow a similar structure:](#http-frames-follow-a-similar-structure)
  - [Implementing the Framing Layer](#implementing-the-framing-layer)
  - [Buffered Writes](#buffered-writes)
  - [Buf and BufMut Traits](#buf-and-bufmut-traits)
  - [Parsing Frames](#parsing-frames)
  - [Optimizations](#optimizations)

## What is Framing?

Framing is the process of converting a byte stream into a stream of frames, which are units of data transmitted between two peers. In Mini-Redis, a frame could be:

```rust
enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}
```

## HTTP frames follow a similar structure:

```rust
enum HttpFrame {
    RequestHead { ... },
    ResponseHead { ... },
    BodyChunk { ... },
}
```

## Implementing the Framing Layer

In Mini-Redis, the Connection struct is introduced to handle reading and writing Frame objects over a TcpStream:

```rust
struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}
```

The `read_frame()` method buffers incoming data, parsing it into frames when possible. Here's an example of buffered reads in `read_frame`:

```rust
pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
    // Loop until a frame is successfully parsed or an error occurs
}
```

## Buffered Writes

To optimize writing frames, a write buffer is maintained using `BufWriter`. Frames are written to this buffer and flushed to the socket to minimize syscalls. The `write_frame()` function encodes and sends a frame over the socket:

```rust
async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
    // Write the frame type and data, followed by flushing the buffer
}
```

## Buf and BufMut Traits

- Buf abstracts reading from byte arrays, while BufMut handles writing. These are used to streamline reading/writing operations.
- BytesMut is a mutable byte buffer that avoids unnecessary memory initialization.

## Parsing Frames

The `parse_frame()` function parses a frame in two steps:

- Ensure a full frame is buffered.
- Parse the frame:

```rust
fn parse_frame(&mut self) -> Result<Option<Frame>> {
    // Parse a frame and update the buffer
}
```

## Optimizations

- `BufWriter` ensures that data is buffered before writing to the socket, reducing write syscalls.
- `flush()` is called after each write to ensure the buffer is written to the socket.
