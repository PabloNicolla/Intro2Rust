# Rust CH05

- [Rust CH05](#rust-ch05)
  - [Struct](#struct)
  - [Tuple Struct](#tuple-struct)
    - [Unit-Like Struct](#unit-like-struct)
  - [Printing structs](#printing-structs)
  - [dbg!](#dbg)
  - [Struct Method Implementation](#struct-method-implementation)
  - [Associated Functions (Class Functions)](#associated-functions-class-functions)

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

## Struct Method Implementation

- Methods must have a parameter named self of type Self for their first parameter
- self: &Self
- &self
- self: &mut Self
- &mut self

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // Any function defined here will be a method of Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

## Associated Functions (Class Functions)

- does not take `&self` as parameter
- belongs to the type
- accessed with `::`

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

let ret = Rectangle::square(20);
```