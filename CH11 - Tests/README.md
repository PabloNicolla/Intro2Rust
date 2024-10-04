# Rust CH11

- [Rust CH11](#rust-ch11)
  - [Testing](#testing)
    - [Cargo test Commands](#cargo-test-commands)
    - [Example 1](#example-1)
    - [assert!](#assert)
  - [assert\_eq! and assert\_ne!](#assert_eq-and-assert_ne)
  - [Custom messages](#custom-messages)
  - [Should Panic](#should-panic)
  - [Using `Result<T, E>` in Tests](#using-resultt-e-in-tests)
  - [Ignored tests](#ignored-tests)
  - [Integration Tests](#integration-tests)

## Testing

- tests are not built when running `cargo build`, saving compile time
- the `#[cfg(test)]` tells rust compiler to compile tests only when configuration has `test` option

### Cargo test Commands

```sh
cargo test

cargo test --help       # display options that can be used with test

cargo test -- --help    # display options that can be used after the separator 

cargo test -- --test-threads=1      # control the number of threads... use 1 to prevent tests running in parallel

cargo test -- --show-output         # show stdout output even for tests that passed

cargo test substr_or_name           # run all tests that contain `substr_or_name` in their name

cargo test -- --ignored             # run all ignored tests

cargo test -- --include-ignored     # run all tests
```

### Example 1

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

### assert!

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller)); // expects any expression that returns a boolean
    }
}
```

## assert_eq! and assert_ne!

- type must implement the PartialEq and Debug traits

## Custom messages

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
}
```

## Should Panic

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]             // test must panic to pass
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

```rust
#[should_panic(expected = "less than or equal to 100")] // panic message must contain expected string to pass
```

## Using `Result<T, E>` in Tests

- allows using question mark operator
- cannot have `#[should_panic]`

```rust
#[test]
fn it_works() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

## Ignored tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
```

## Integration Tests

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

```rust
// Filename: tests/integration_test.rs
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```