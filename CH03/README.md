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
