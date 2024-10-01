# Rust CH04

## Ownership

### drop

- drop frees memory
- The drop function in Rust will be familiar to you if you’ve used RAII patterns.

### any group simple scalar values

```rust
let x = 1;
let y = x; // copy
```

### types requiring any allocation

```rust
let s1 = String::from("hello"); // unique_ptr
let s2 = s1; // Same as std::move(unique_ptr...)

// accessing s1 will result in a compile error

let s3 = s2.clone(); // copy: 2 independent unique_ptr // this copy may be expensive
```

### Copy vs. Clone

> [!IMPORTANT]
> COPY cannot be implemented if the type, or any of its parts, has implemented the Drop trait

### Ownership and Functions

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

### References & Borrowing

Avoid losing ownership by using references

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Mutable References

> [!IMPORTANT]
> There can only be one mutable reference to the same data at a time

```rust
let r1 = &mut s;
let r2 = &mut s;
```

> [!IMPORTANT]
> Similar rule for combining mutable and immutable references

```rust
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
```

### Lifetime Scope

- where the variable is last used... not necessarily the scope's end
