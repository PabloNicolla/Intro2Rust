# Rust CH08

- [Rust CH08](#rust-ch08)
  - [Vector](#vector)
  - [Vector (Multiple Types)](#vector-multiple-types)
  - [Strings](#strings)
    - [String Bytes](#string-bytes)
    - [String Chars](#string-chars)
    - [String Grapheme Clusters](#string-grapheme-clusters)
  - [HashMaps](#hashmaps)

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

## Strings

### String Bytes

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

### String Chars

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

### String Grapheme Clusters

- Real/Meaningful representation

## HashMaps

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

```rust
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```