# `.from()`

- [`.from()`](#from)
  - [Explanation](#explanation)
  - [Summary](#summary)

## Explanation

- Usage: Trait method `From::from()`

- Description: This method is part of the `From` trait and is used to convert one type into another. The `From` trait is complementary to `Into`, but it works the opposite way: with `From`, you specify the conversion at the type being converted to, while `Into` works from the type being converted from.

- Example:

```rust
let s = String::from("Hello, Rust!");
let num: i32 = i32::from(42u8); // converts u8 to i32
```

In the example, `String::from("Hello, Rust!")` converts a `&str` into a `String`, and `i32::from(42u8)` converts a `u8` to an `i32`. `From` is generally preferred when a type has a clear and non-ambiguous way to be constructed from another.

## Summary

- `.from()`: Converts one type to another, typically used for explicit conversions at the type being converted to.
