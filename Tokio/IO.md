# Summary of I/O in Tokio

- [Summary of I/O in Tokio](#summary-of-io-in-tokio)
  - [Introduction](#introduction)
  - [AsyncRead and AsyncWrite](#asyncread-and-asyncwrite)
    - [Reading from I/O:](#reading-from-io)
    - [Writing to I/O:](#writing-to-io)
    - [Helper Functions](#helper-functions)
    - [Echo Server Example](#echo-server-example)
    - [Using io::copy()](#using-iocopy)
    - [Manual Copying](#manual-copying)
    - [Task Efficiency](#task-efficiency)
  - [Conclusion](#conclusion)

## Introduction

In Tokio, I/O operates asynchronously, similar to how it works in `std`, but with traits like `AsyncRead` and `AsyncWrite`. These traits are implemented by types such as `TcpStream`, `File`, and `Stdout`, as well as data structures like `Vec<u8>` and `&[u8]`. This allows using byte arrays in place of readers or writers.

## AsyncRead and AsyncWrite

- `AsyncReadExt` and `AsyncWriteExt` provide utility methods for working with I/O streams without needing to manually call trait methods.

### Reading from I/O:

- `read()`: Asynchronously reads data into a buffer.

```rust
let mut f = File::open("foo.txt").await?;
let mut buffer = [0; 10];
let n = f.read(&mut buffer[..]).await?;
```

- `read_to_end()`: Reads all bytes from the stream until EOF.

```rust
let mut f = File::open("foo.txt").await?;
let mut buffer = Vec::new();
f.read_to_end(&mut buffer).await?;
```

### Writing to I/O:

- `write()`: Asynchronously writes a buffer to the writer, returning how many bytes were written.

```rust
let mut file = File::create("foo.txt").await?;
let n = file.write(b"some bytes").await?;
```

- `write_all()`: Writes the entire buffer to the writer.

```rust
file.write_all(b"some bytes").await?;
```

### Helper Functions

- Utility functions like `tokio::io::copy` allow copying data between readers and writers asynchronously.

```rust
let mut reader: &[u8] = b"hello";
let mut file = File::create("foo.txt").await?;
io::copy(&mut reader, &mut file).await?;
```

### Echo Server Example

- An echo server can read data from a `TcpListener` and immediately write the data back to the client.

### Using io::copy()

- Bind a `TcpListener` and accept inbound connections.
- For each connection, split the `TcpStream` into a reader and writer using `TcpStream::split`, then use `io::copy()` to echo the data.

```rust
let (mut rd, mut wr) = socket.split();
io::copy(&mut rd, &mut wr).await?;
```

### Manual Copying

- Read from the socket using `AsyncReadExt::read` and write back with `AsyncWriteExt::write_all`.

```rust
let mut buf = vec![0; 1024];
match socket.read(&mut buf).await {
    Ok(0) => return,
    Ok(n) => {
        socket.write_all(&buf[..n]).await?;
    }
}
```

### Task Efficiency

- To avoid performance issues, buffers used across `.await` calls are allocated dynamically (e.g., `Vec<u8>`) rather than stack arrays.
- It's important to handle `EOF (Ok(0))` to exit read loops and prevent infinite loops with high CPU usage.

## Conclusion

Tokio offers various utilities for asynchronous I/O, and combining traits like `AsyncRead`, `AsyncWrite`, and helper functions allows efficient and powerful stream processing.
