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
    - [Default Generic Type Parameters and Operator Overloading](#default-generic-type-parameters-and-operator-overloading)
    - [Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name](#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name)
      - [Associated Function with the same name](#associated-function-with-the-same-name)
    - [Using Supertraits to Require One Trait’s Functionality Within Another Trait](#using-supertraits-to-require-one-traits-functionality-within-another-trait)
  - [Advanced Types](#advanced-types)
    - [Creating Type Synonyms with Type Aliases](#creating-type-synonyms-with-type-aliases)
    - [The Never Type that Never Returns](#the-never-type-that-never-returns)
  - [Advanced Functions and Closures](#advanced-functions-and-closures)
    - [Function Pointers](#function-pointers)
    - [Returning Closures](#returning-closures)
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

```rust
pub trait Iterator {
    type Item;                  // Somewhat similar to generic placeholder type (<T>)
                                // Allows you to use Item without knowing/defining the type it can/will be
    fn next(&mut self) -> Option<Self::Item>;
}
```

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

### Default Generic Type Parameters and Operator Overloading

```rust
trait Add<Rhs=Self> {           // default
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```
### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

#### Associated Function with the same name

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

- enforces any type implementing `OutlinePrint` trait to have `Display` trait also implemented

```rust
trait OutlinePrint: fmt::Display {
    /**/
}
```

## Advanced Types

### Creating Type Synonyms with Type Aliases

custom names

```rust
type Kilometers = i32;
```

reduce repetition of large types

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

### The Never Type that Never Returns

- `!` is the **never type**
- represents an empty type because it has no values
- used in functions that never return **diverging functions**
- somewhat similar to C++ `noreturn`

```rust
fn bar() -> ! {
    // --snip--
}
```

## Advanced Functions and Closures

### Function Pointers

- The `fn` type is called a function pointer
- Function pointers implement all three of the closure `traits` `(Fn, FnMut, and FnOnce)`

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {           // fn receives one i32 and returns i32
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}
```

### Returning Closures

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## Macros
