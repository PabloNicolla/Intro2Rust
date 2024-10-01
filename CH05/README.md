# Rust CH05

## Struct

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // complete missing attributes, must be the last one
    };
}
```

## Tuple Struct

```rust
struct Color(i32, i32, i32); // Pre-typed tuple
struct Point(i32, i32, i32); // Even though both hold the same structure, they are not treated as equal types
```

### Unit-Like Struct

- unit-like struct behave similarly to `()`

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```