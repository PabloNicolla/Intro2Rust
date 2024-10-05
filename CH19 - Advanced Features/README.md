# Rust CH19

- [Rust CH19](#rust-ch19)
  - [Unsafe Rust](#unsafe-rust)
    - [Dereferencing Raw Pointer](#dereferencing-raw-pointer)
    - [Calling an Unsafe Function or Method](#calling-an-unsafe-function-or-method)
    - [Using `extern` Functions to Call External Code](#using-extern-functions-to-call-external-code)
    - [Accessing or Modifying a Mutable Static Variable](#accessing-or-modifying-a-mutable-static-variable)
    - [Implementing an Unsafe Trait](#implementing-an-unsafe-trait)
    - [Accessing Fields of a Union](#accessing-fields-of-a-union)
  - [Advanced Traits](#advanced-traits)
  - [Advanced Types](#advanced-types)
  - [Advanced Functions and Closures](#advanced-functions-and-closures)
  - [Macros](#macros)

## Unsafe Rust

- To switch to **unsafe Rust**, use the `unsafe` keyword and then start a new block that holds the unsafe code.
- You can take five actions in unsafe Rust that you can’t in safe Rust, which we call unsafe superpowers. Those superpowers include the ability to:
  - Dereference a raw pointer
  - Call an unsafe function or method
  - Access or modify a mutable static variable
  - Implement an unsafe trait
  - Access fields of a union

### Dereferencing Raw Pointer

- raw pointers can be immutable or mutable
  - `*const T`
  - `*mut T`

Different from references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

### Calling an Unsafe Function or Method

- tells compiler that you are accepting and understands the risks of calling the unsafe function

```rust
unsafe fn dangerous(/*...*/) {/*...*/}
```

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {                                                        // Unsafe block
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

### Using `extern` Functions to Call External Code

- Rust code might need to interact with code written in another language.
- keyword `extern` that facilitates the creation and use of a **Foreign Function Interface (FFI)**.
- An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

### Accessing or Modifying a Mutable Static Variable

- In Rust, global variables are called static variables.

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {HELLO_WORLD}");
}
```

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;                 // unsafe
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
```

### Implementing an Unsafe Trait

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

### Accessing Fields of a Union

Unions are primarily used to interface with unions in C code.

## Advanced Traits

## Advanced Types

## Advanced Functions and Closures

## Macros
