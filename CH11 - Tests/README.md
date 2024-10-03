# Rust CH11

## Testing

### Cargo Command

```sh
cargo test
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