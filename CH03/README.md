# Rust CH03

## Tuple, unit and default return value

- `()`: The tuple without any values has a special name, unit.
  - type: `()`
  - This value represents an empty value or an empty return type.
  - Expressions implicitly return the unit value if they don’t return any other value.

## Arrays

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

let a = [3; 5]; // [3] * 5

let a = [1, 2, 3, 4, 5];
```

## Tail

```rust
{
    let x = 3;
    x + 1 // tail (missing semi-colon), implicit return value.
}
```

```rust
fn five() -> i32 {
    5
}
```

## loop and break

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

## loop labels and break

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // instead of breaking innermost loop, outermost loop is stopped
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

## While and For

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    let mut index = 0;

    while index < 5 {
        // ...
    }
}
```

## Range

- `1..`    : infinite range
- `1..5`   : range from 1 to 4
- `1..=5`  : range from 1 to 5
- and others

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```