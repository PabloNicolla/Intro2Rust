# Rust CH10

## Generics

### Functions

```rust
fn largest<T>(list: &[T]) -> &T {}
```

### Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

#### impl 

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

##### impl specialization

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
