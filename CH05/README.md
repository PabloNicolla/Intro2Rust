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

## Printing structs

- Rust do not implement default `Display` for user defined structs
- However, the debug mode can be used to print structs without additional code implementation
- `:?` is used to specify `Debug` instead of `Display`
- `#[derive(Debug)]` flag required to activate `Debug`
- `{:#?}` alternative to `:?`. Useful for large data structures

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
```

## dbg!

- Prints to `stderr` instead of `stdout`
- Prints the file and line
- Prints the argument with `Debug`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // takes ownership, and returns the ownership
        height: 50,
    };

    dbg!(&rect1);
}
```