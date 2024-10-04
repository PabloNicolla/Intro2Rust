# Rust CH13

- [Rust CH13](#rust-ch13)
  - [Closures](#closures)
    - [borrowing immutably](#borrowing-immutably)
    - [borrowing mutably](#borrowing-mutably)
    - [taking ownership](#taking-ownership)
    - [Closure Traits](#closure-traits)
      - [Example: defining function as a parameter](#example-defining-function-as-a-parameter)
  - [Iterators](#iterators)
    - [Iterator Trait](#iterator-trait)
    - [consuming adaptors](#consuming-adaptors)
    - [Iterator adaptors](#iterator-adaptors)
    - [Iterator Type as parameter](#iterator-type-as-parameter)

## Closures

- The first call of a closure with inferred types defines the type until the end of its lifetime
- Closures capture their environment

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

### borrowing immutably

```rust
fn main() {
    let list = vec![1, 2, 3];
    let only_borrows = || println!("From closure: {list:?}");
    only_borrows();
}
```

### borrowing mutably

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
}
```

### taking ownership

- This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```

### Closure Traits


- **FnOnce** 
  - applies to closures that can be called once. 
  - All closures implement at least this trait, because all closures can be called.
  - A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
- **FnMut** 
  - applies to closures that don’t move captured values out of their body, but that might mutate the captured values. 
  - These closures can be called more than once.
- **Fn** 
  - applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. 
  - These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

> [!IMPORTANT]
> traits: Fn, FnMut, and FnOnce are implemented automatically to closures based on how they behave/(logic)

> [!IMPORTANT]
> normal functions in Rust implement all three closure traits: Fn, FnMut, and FnOnce.

#### Example: defining function as a parameter

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

## Iterators

- iterators are consumed
- `iter`: iterate over **immutable** references
- `into_iter`:  iterator that **takes ownership** of `v1` and returns owned values
- `iter_mut`: iterate over **mutable references**

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}
```

### Iterator Trait

- `type Item` associated type
  - must be defined
- next method: returns the next element wrapped in `Some`, or it returns `None` when finished
  - must be implemented

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

### consuming adaptors

Methods that call `next` are called **consuming adaptors**

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

### Iterator adaptors

- Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
- Instead, they produce different iterators by changing some aspect of the original iterator.

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();  // must be consumed and collected by .collect(),
                                                      // otherwise nothing happens to the created iterator

assert_eq!(v2, vec![2, 3, 4]);
```

Explanation:

- .map consumes an iterator
- .map creates a new iterator as the result/return of its operation
- since iterators are lazy and needs to be consumed, .collect() must be called to transform the iterator into a Vec collection

### Iterator Type as parameter

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --snip--
```