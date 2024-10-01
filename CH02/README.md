# RUST CH02

## Standard Library

[Standard Library](https://doc.rust-lang.org/std/prelude/index.html)

```rust
use std::io;

io::stdin()
```

OR

```rust
std::io::stdin()
```

## Intro2Mutability

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

## References

Same as C++ so far...

## Result Enumerations

Can have multiple states called variants

in case of `std::io::stdin()` is `Ok` or `Err`

these states may also hold values related to their origin operation