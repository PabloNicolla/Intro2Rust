# `.into()`

- [`.into()`](#into)
  - [Explanation](#explanation)
  - [Summary](#summary)

## Explanation

- Usage: Trait method `Into::into()`

- Description: This method is part of the `Into` trait and is used to convert a value into another type. When you call `.into()`, Rust automatically infers or determines what type the value should be converted to based on the context.

- Example:

```rust
let num: i32 = 5;
let num_as_f64: f64 = num.into(); // num_as_f64 is now `5.0` of type `f64`
```

Here, `.into()` is used to convert an `i32` into an `f64`. Rust provides many default implementations of the `Into` trait, but you can also implement it for your own types to enable custom conversions.

## Summary

- `.into()`: Converts a value into another type, inferred from the context.
