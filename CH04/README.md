# Rust CH04

## Ownership

### drop

- drop frees memory
- The drop function in Rust will be familiar to you if you’ve used RAII patterns.

### any group simple scalar values

```rust
let x = 1;
let y = x; // copy
```

### types requiring any allocation

```rust
let s1 = String::from("hello"); // unique_ptr
let s2 = s1; // Same as std::move(unique_ptr...)

// accessing s1 will result in a compile error

let s3 = s2.clone(); // copy: 2 independent unique_ptr // this copy may be expensive
```

### Copy vs. Clone

> [!IMPORTANT]
> COPY cannot be implemented if the type, or any of its parts, has implemented the Drop trait