# Rust CH10.5

## Lifetime Annotation Syntax

- Lifetime annotations don’t change how long any of the references live.
- Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

## Lifetime Annotations in Function Signatures

- the smallest lifetime between `x` and `y` will be the return value lifetime

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Lifetime Annotations in Struct Definitions

- allow structs to hold references
- struct cannot outlive the data member reference lifetime

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

## lifetime elision rules

- patterns programmed into Rust’s analysis of references.
- allows the compiler to infer the return reference lifetime.
- works when there is no ambiguity

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### lifetime elision rules

#### Example 1

```rust
fn first_word(s: &str) -> &str {/*...*/}
```

- rule 1 and 2 applied

```rust
fn first_word<'a>(s: &'a str) -> &'a str {/*...*/}
```

---
#### Example 2

```rust
fn longest(x: &str, y: &str) -> &str {/*...*/}
```

- rule 1 applied

```rust
fn longest(x: &str, y: &str) -> &str {/*...*/}
```

---
#### Example 3

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

- no rules applied since return is not a reference

---
#### Example 4

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

- rule 1 and 3 applied
  - return types has &self lifetime

## The Static Lifetime

- denotes that the affected reference can live for the entire duration of the program.

```rust
let s: &'static str = "I have a static lifetime.";
```

> [!IMPORTANT]
> You might see suggestions to use the 'static lifetime in error messages. But only apply it when it makes sense to do so.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```