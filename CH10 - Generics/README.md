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

## Traits

- Somewhat similar to `interfaces` from other languages.
- Each type implementing this trait must provide its own custom behavior for the body of the method.

```rust
pub trait Summary {
    fn summarize(&self) -> String; // just  signature
}
```

### Implementing a Trait on a Type

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### Default implementation

- Instead of having just the signature, provides a default logic
- can be overridden

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;                           // requires implementation

    fn summarize(&self) -> String {                                 // Default implementation
        format!("(Read more from {}...)", self.summarize_author())  // can call other function in the same trait
    }
}
```

### Traits Restrictions

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {/*...*/}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### Traits as Parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Specifying Multiple Trait Bounds with the + Syntax

```rust
pub fn notify(item: &(impl Summary + Display)) {/*...*/}

pub fn notify<T: Summary + Display>(item: &T) {/*...*/}
```

### Clearer Trait Bounds with where Clauses

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    /*...*/
}
```

### Conditionally Implement Methods

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {                               // Implement for all types
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {         // Implement for types that have the selected traits
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


impl<T: Display> ToString for T {               // Implement trait only for types that have the selected traits
    // --snip--
}
```