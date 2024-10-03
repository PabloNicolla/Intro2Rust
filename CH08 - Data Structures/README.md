# Rust CH08

## Vector

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

## Vector (Multiple Types)

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```