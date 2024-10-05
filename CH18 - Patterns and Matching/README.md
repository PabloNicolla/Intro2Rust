# Rust CH18

- [Rust CH18](#rust-ch18)
  - [Match](#match)
  - [if let](#if-let)
  - [while let](#while-let)
  - [let](#let)
  - [Refutability: Whether a Pattern Might Fail to Match](#refutability-whether-a-pattern-might-fail-to-match)
  - [Matching Named Variables](#matching-named-variables)
  - [Multiple Patterns](#multiple-patterns)
  - [Matching Range](#matching-range)
  - [Destructuring to Break Apart Values](#destructuring-to-break-apart-values)
    - [Destructuring Structs](#destructuring-structs)
      - [Custom names](#custom-names)
      - [Struct defined names](#struct-defined-names)
      - [Custom literal values](#custom-literal-values)
    - [Destructuring Enums](#destructuring-enums)
    - [Destructuring Nested Structs and Enums](#destructuring-nested-structs-and-enums)
    - [Destructuring Structs and Tuples](#destructuring-structs-and-tuples)
    - [Ignoring Values in a Pattern](#ignoring-values-in-a-pattern)
      - [Ignoring an Entire Value with `_`](#ignoring-an-entire-value-with-_)
      - [Ignoring Remaining Parts of a Value with ..](#ignoring-remaining-parts-of-a-value-with-)
  - [Extra Conditionals with Match Guards](#extra-conditionals-with-match-guards)
  - [`@` Bindings](#-bindings)

## Match

```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

- **exhaustive** in the sense that all possibilities for the value in the match expression must be accounted for.
- The particular pattern `_`:
  - will match anything
  - it’s often used in the last match arm.
  - can be useful when you want to ignore any value not specified.

## if let

- `if let` expressions mainly as a shorter way to write the equivalent of a match that only matches one case.
- Optionally, if let can have a corresponding else containing code to run if the pattern in the if let doesn’t match.

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {                       // shadowed variable `age`
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

## while let

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{top}");
}
```

## let

```rust
let PATTERN = EXPRESSION;

let (x, y, z) = (1, 2, 3);

let x = 5;
```

## Refutability: Whether a Pattern Might Fail to Match

- Patterns come in two forms:
  - refutable
  - irrefutable
  
- Patterns that will match for any possible value passed are **irrefutable**.
- Patterns that can fail to match for some possible value are **refutable**.

## Matching Named Variables

```rust
let x = Some(5);
let y = 10;

match x {                                               // a new scope is created here,
    Some(50) => println!("Got 50"),                     // any name defined here will shadow the outer scope...
    Some(y) => println!("Matched, y = {y}"),            // y will match any value inside a Some value;
    _ => println!("Default case, x = {x:?}"),           // because it does not refer to the outer scope `let y = 10`
}

println!("at the end: x = {x:?}, y = {y}");
```

## Multiple Patterns

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

## Matching Range

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),              // 1 | 2 | 3 | 4 | 5 =>
    _ => println!("something else"),
}
```

```rust
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

## Destructuring to Break Apart Values

### Destructuring Structs

#### Custom names

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;               // a and b are created based on x and y
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

#### Struct defined names

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;                     // x and y are created
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

#### Custom literal values

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

### Destructuring Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }
}
```

### Destructuring Nested Structs and Enums

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
```

### Destructuring Structs and Tuples

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

### Ignoring Values in a Pattern

#### Ignoring an Entire Value with `_`

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
}
```

#### Ignoring Remaining Parts of a Value with ..

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {x}"),
}
```

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

## Extra Conditionals with Match Guards

```rust
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {x} is even"),
    Some(x) => println!("The number {x} is odd"),
    None => (),
}
```

## `@` Bindings

The at operator `@` lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,                                // variable id_variable is created
    } => println!("Found an id in range: {id_variable}"),
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {id}"),
}
```