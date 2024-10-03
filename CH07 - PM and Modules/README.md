# Rust CH07

## module system

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

### Packages and Crates

[Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)

- A crate can come in one of two forms: a binary crate or a library crate.
- A package can contain as many binary crates as you like, but at most only one library crate.
- A package must contain at least one crate, whether that’s a library or binary crate.

### Modules Privacy and Scope

- the `use` keyword that brings a path into scope.
- and the `pub` keyword to make items public.

#### Modules Cheat Sheet

- declare `mod garden` in the **crate root file**
  - The compiler will look for the module’s code in these places:
    - Inline, within curly brackets that replace the semicolon following mod garden
    - In the file src/garden.rs
    - In the file src/garden/mod.rs

- declare `mod vegetables` not in the **crate root file**
  - The compiler will look for the module’s code in these places:
    - Inline, directly following mod vegetables, within curly brackets instead of the semicolon
    - In the file src/garden/vegetables.rs
    - In the file src/garden/vegetables/mod.rs

- Paths to code in modules:
  - crate::garden::vegetables::Asparagus

- Private vs Public
  - `pub mod name`
  - `mod name`

- The `use` keyword:
  - `use crate::garden::vegetables::Asparagus;`
  - then
  - `Asparagus` is accessible without the namespace syntax

#### mod example

```rust
// src/main.rs
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

```rust
// src/garden.rs
pub mod vegetables;
```

```rust
// src/garden/vegetables.rs
#[derive(Debug)]
pub struct Asparagus {}
```

### Relative Path and Absolute Path

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // absolute starting with crate::

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // relative
}
```

### super keyword

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // similar to `$ cd ../folderName`, where `super` is `..`
    }

    fn cook_order() {}
}
```

### as keyword

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

### re-exporting names

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### Nested Paths

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
use std::io;
use std::io::Write;
```

same as

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
use std::io::{self, Write};
```

### Glob Operator

```rust
use std::collections::*;
```

## Create Library

```sh
cargo new restaurant --lib
```