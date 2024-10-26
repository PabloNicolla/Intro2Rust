# take() and as_mut()

- [take() and as\_mut()](#take-and-as_mut)
  - [`take()`](#take)
  - [`.as_mut()`](#as_mut)
  - [Summary](#summary)

## `take()`

- Usage: `Option::take()`

- Description: This method is used with `Option` types. It replaces the value inside an `Option` with `None`, returning the original value in the `Option`.

- Example:

```rust
let mut opt = Some(5);
let val = opt.take(); // `opt` is now `None`, `val` is `Some(5)`
```

After calling `.take()`, the Option is left empty `(None)`, and you get the previous value that was inside the `Option`. It’s useful when you want to "move" the value out of an `Option` and leave `None` in its place.

## `.as_mut()`

- Usage: `Option::as_mut()`, `Result::as_mut()`, etc.

- Description: This method is used to get a mutable reference to the inner value of certain types, like `Option` or `Result`. If the value exists (`Some` or `Ok`), it returns a `Some(&mut T)` where `T` is a mutable reference. If it’s `None` or `Err`, it returns a reference to that variant.

- Example:

```rust
let mut opt = Some(10);
if let Some(x) = opt.as_mut() {
    *x += 5; // Now the value inside opt is `15`
}
```

`.as_mut()` allows you to work directly with the value inside an `Option` or `Result` in a mutable way without taking ownership of the inner value.

## Summary

- `.take()`: Moves the value out of an `Option`, leaving `None` behind.
- `.as_mut()`: Provides a mutable reference to the inner value of an `Option` or `Result`.
