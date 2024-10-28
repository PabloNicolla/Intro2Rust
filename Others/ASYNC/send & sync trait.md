# Send and Sync

- [Send and Sync](#send-and-sync)
  - [Introduction](#introduction)
    - [1. `Send` Trait](#1-send-trait)
      - [Characteristics of `Send`](#characteristics-of-send)
      - [Example](#example)
      - [Custom Types](#custom-types)
    - [2. `Sync` Trait](#2-sync-trait)
      - [Characteristics of `Sync`](#characteristics-of-sync)
      - [Example](#example-1)
      - [Custom Types](#custom-types-1)
    - [`Send` and `Sync` Interaction](#send-and-sync-interaction)
    - [Practical Usage](#practical-usage)
    - [Send and Sync Summary](#send-and-sync-summary)
  - [Send and Sync possible combinations](#send-and-sync-possible-combinations)
    - [How `T` is Determined to be `Send`](#how-t-is-determined-to-be-send)
    - [How `&T` is Determined to be `Send`](#how-t-is-determined-to-be-send-1)
    - [Can `T` Be `Send` While `&T` Is Not?](#can-t-be-send-while-t-is-not)
      - [Example: `Cell<T>`](#example-cellt)
    - [Can `&T` Be `Send` While `T` Is Not?](#can-t-be-send-while-t-is-not-1)
      - [Example: `Arc<T>`](#example-arct)
    - [Summary Table](#summary-table)
    - [Key Takeaways](#key-takeaways)

## Introduction

In Rust, the `Send` and `Sync` traits play crucial roles in ensuring safe concurrency and multithreading. They provide guidelines for how data can be safely shared or transferred across threads. Let’s dive into what each trait does, how they work, and how they’re implemented in Rust.

### 1. `Send` Trait

The `Send` trait indicates that a type is safe to transfer ownership between threads. If a type implements `Send`, you can move it to another thread, making it possible for data to be passed safely between threads without risking data races.

#### Characteristics of `Send`

- **Automatic Implementation**: Rust automatically implements `Send` for types composed of other `Send` types. Most types in Rust are `Send` by default.
- **Types without `Send`**: Some types, like `Rc<T>`, are not `Send` because they maintain shared state that doesn’t have thread-safe mechanisms for mutation. Using `Rc<T>` across threads could cause data races since it’s a single-threaded reference-counting pointer without atomic operations.
- **Common `Send` Types**:
  - Primitive types (e.g., `i32`, `f64`, `bool`) are `Send`.
  - `String` and `Vec<T>` are `Send` if the data they contain is `Send`.
  - Types with `Arc<T>` and `Mutex<T>` are `Send`, as they use atomic operations and locks to safely share across threads.

#### Example

Consider this code where `Send` is necessary:

```rust
use std::thread;

fn main() {
    let message = String::from("Hello, world!");

    // Move `message` to a new thread.
    let handle = thread::spawn(move || {
        println!("{}", message);
    });

    handle.join().unwrap();
}
```

In this example, the `message` string is moved into the new thread because `String` implements `Send`.

#### Custom Types

For custom types, `Send` is automatically implemented if all its fields are `Send`. Otherwise, you may need to implement it manually, though manual implementations of `Send` are unsafe and discouraged.

### 2. `Sync` Trait

The `Sync` trait indicates that a type can safely be referenced from multiple threads. In other words, a type is `Sync` if it’s safe to share a reference (`&T`) to it across threads.

#### Characteristics of `Sync`

- **Automatic Implementation**: Like `Send`, Rust automatically implements `Sync` for types composed of other `Sync` types.
- **Types without `Sync`**: Types like `Cell<T>` and `RefCell<T>` are not `Sync` because they allow for mutable access to data without any locking mechanism, making them unsafe to share across threads.
- **Common `Sync` Types**:
  - Primitive types (e.g., `i32`, `f64`, `bool`) are `Sync`.
  - `Arc<T>` is `Sync` if `T` is `Sync`, meaning shared ownership of `T` across threads is safe.
  - `Mutex<T>` and `RwLock<T>` make it possible to have `Sync` behavior on types that would otherwise be non-`Sync` by providing a locking mechanism.

#### Example

Using a reference to a shared data structure across threads requires `Sync`:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let threads: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("Thread {}: {:?}", i, data);
        })
    }).collect();

    for handle in threads {
        handle.join().unwrap();
    }
}
```

In this example, `Arc<Vec<i32>>` is `Sync`, so multiple threads can share references to `data` without data races.

#### Custom Types

For custom types, `Sync` is automatically implemented if all of their fields are `Sync`. Manual implementations are rare and should be done carefully, as incorrect implementations can cause undefined behavior.

### `Send` and `Sync` Interaction

- **`Send` Without `Sync`**: A type can be `Send` without being `Sync` (e.g., `Mutex<T>` is `Send`, but it only provides access to `T` safely in one thread at a time).
- **`Sync` Without `Send`**: Similarly, a type can be `Sync` without `Send` (e.g., `Arc<T>` can be `Sync`, but `Rc<T>` is neither `Send` nor `Sync`).

### Practical Usage

Rust enforces these traits at compile-time, using them to prevent data races and ensure safe concurrent access. If you try to share a type that is not `Send` or `Sync` across threads, you’ll get a compile-time error.

### Send and Sync Summary

- **`Send`**: Types that can be transferred between threads by ownership.
- **`Sync`**: Types that can be safely accessed by reference from multiple threads.
- **Automatic Derivation**: Rust derives `Send` and `Sync` for types if their fields are `Send` and `Sync`.
- **Safe Concurrency**: They prevent race conditions and unsafe access in multithreaded programs.

These traits, combined with Rust’s strict compile-time checks, are foundational to its strong concurrency guarantees.



## Send and Sync possible combinations

In Rust, it is possible for `T` to be `Send` while `&T` is not, and vice versa. Rust’s decisions for whether `T` and `&T` are `Send` are based on the types and lifetimes involved, along with whether or not data can be safely shared or transferred between threads. Let’s break down how Rust determines this.

### How `T` is Determined to be `Send`

For a type `T` to be `Send`, it must be safe to move its ownership to another thread. This means:

- **All fields of `T`** must also implement `Send`. If `T` contains any non-`Send` fields (like `Rc<T>`), then `T` cannot be `Send` because moving it to another thread could allow concurrent, unsafe access.
- **Ownership semantics**: Moving a `T` to another thread transfers full ownership of the data, so `T` does not need to support shared, concurrent access—only ownership transfer.

For example, `Mutex<T>` is `Send` (if `T` is `Send`) because it implements thread-safe locking, allowing ownership transfer safely.

### How `&T` is Determined to be `Send`

For a reference `&T` to be `Send`, the type `T` must implement `Sync` (not necessarily `Send`). Here’s why:

- **Shared Access**: `&T` represents a shared, immutable reference to `T`, so it’s safe to send `&T` to another thread only if multiple threads can read from `T` simultaneously.
- **`Sync` Requirement**: If `T` implements `Sync`, it means it’s safe to have multiple `&T` references to the same data across threads. In other words, `&T` can be `Send` if `T` is `Sync`, as it implies that multiple immutable references do not cause data races.

For example, `&i32` is `Send` because `i32` is `Sync` (immutable integers are safe to read across threads), whereas `&RefCell<T>` is not `Send` because `RefCell<T>` is not `Sync` (it allows interior mutability without thread safety).

### Can `T` Be `Send` While `&T` Is Not?

Yes, `T` can be `Send` while `&T` is not. This situation occurs when `T` is safe to move to another thread, but multiple `&T` references to it would not be safe to access concurrently.

#### Example: `Cell<T>`

Consider `Cell<T>`:

- `Cell<T>` is `Send` if `T` is `Copy` because you can transfer ownership of a `Cell<T>` to another thread. However, since `Cell<T>` allows mutation of its contents through an immutable reference (`&T`), `&Cell<T>` is **not** `Send`.
- The lack of `Sync` on `Cell<T>` prevents `&Cell<T>` from being `Send`, as multiple threads could have `&Cell<T>` references and attempt to modify the same data, causing data races.

### Can `&T` Be `Send` While `T` Is Not?

Yes, `&T` can be `Send` even if `T` is not `Send`. This situation typically happens when `T` is inherently non-thread-safe due to ownership semantics but can be safely shared immutably across threads.

#### Example: `Arc<T>`

Consider `Arc<T>`:

- `Arc<T>` is `Sync` (if `T` is `Sync`), meaning `&Arc<T>` can be shared across threads. This makes `&Arc<T>` `Send`, as it’s safe to share an immutable reference to an `Arc<T>`.
- However, `Arc<T>` itself is **not** `Send` (unless `T: Send`), because moving an `Arc<T>` to another thread implies transferring ownership, and without special handling (like atomic reference counting), this would not be thread-safe.

### Summary Table

| Scenario                                               | Is `T` `Send`? | Is `&T` `Send`? |
| ------------------------------------------------------ | -------------- | --------------- |
| `T` is thread-safe (e.g., `i32`)                       | Yes            | Yes             |
| `T` is `Send` but not `Sync` (e.g., `Cell<T>`)         | Yes            | No              |
| `T` is not `Send`, but `&T` is `Sync` (e.g., `Arc<T>`) | No             | Yes             |
| `T` is neither `Send` nor `Sync` (e.g., `Rc<T>`)       | No             | No              |

### Key Takeaways

- **`T` is `Send`** if it can be safely transferred to another thread.
- **`&T` is `Send`** if `T` is `Sync`, meaning `T` allows safe, concurrent access across threads.
- **Distinct but related traits**: It’s possible for `T` to be `Send` without `&T` being `Send`, or for `&T` to be `Send` even if `T` is not `Send`.
