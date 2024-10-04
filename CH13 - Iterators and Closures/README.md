# Rust CH13

- [Rust CH13](#rust-ch13)
  - [Closures](#closures)
    - [borrowing immutably](#borrowing-immutably)
    - [borrowing mutably](#borrowing-mutably)
    - [taking ownership](#taking-ownership)
    - [Closure Traits](#closure-traits)
      - [Example: defining function as a parameter](#example-defining-function-as-a-parameter)
  - [Iterators](#iterators)

## Closures

- The first call of a closure with inferred types defines the type until the end of its lifetime

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