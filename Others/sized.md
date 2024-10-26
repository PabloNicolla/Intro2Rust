# Sized Trait

- [Sized Trait](#sized-trait)
  - [Explanation](#explanation)
  - [Key implications](#key-implications)
    - [Variables and function parameters must be Sized by default](#variables-and-function-parameters-must-be-sized-by-default)
    - [You can opt out of the Sized requirement using the ?Sized bound](#you-can-opt-out-of-the-sized-requirement-using-the-sized-bound)
    - [!Sized types must always be accessed through some kind of indirection](#sized-types-must-always-be-accessed-through-some-kind-of-indirection)

## Explanation

A type that is Sized means the compiler knows its exact size at compile time. Most types in Rust are inherently Sized, like:

```rust
let x: i32;        // Sized: exactly 4 bytes
let y: [i32; 5];   // Sized: exactly 20 bytes (4 bytes × 5 elements)
let z: (i32, f64); // Sized: exactly 16 bytes (4 + 8 bytes, with padding)
```

A type that is !Sized (pronounced "not sized") means its size is not known at compile time. The most common examples are:

```rust
str           // !Sized: a string slice of dynamic length
[T]           // !Sized: a slice of dynamic length
dyn Trait     // !Sized: trait objects with unknown concrete type
```

## Key implications

### Variables and function parameters must be Sized by default

```rust
// Won't compile - str is !Sized
fn process(s: str) { }

// This works - &str is Sized because references have a known size
fn process(s: &str) { }
```

### You can opt out of the Sized requirement using the ?Sized bound

```rust
// T must be Sized (default)
fn normal<T>(t: T) { }

// T can be either Sized or !Sized
fn flexible<T: ?Sized>(t: &T) { }
```

### !Sized types must always be accessed through some kind of indirection

- References (&str)
- Box (Box<dyn Trait>)
- Rc or Arc
