# Rust CH14

- [Rust CH14](#rust-ch14)
  - [Build Profiles](#build-profiles)
    - [Customizing Profiles](#customizing-profiles)
  - [documentation comment](#documentation-comment)
    - [Documentation Comments as Tests](#documentation-comments-as-tests)
    - [Commenting Contained Items](#commenting-contained-items)
  - [re-export items](#re-export-items)
  - [workspaces](#workspaces)
  - [Installing Binaries with cargo install](#installing-binaries-with-cargo-install)

## Build Profiles

- Cargo has two main profiles: 
  - the **dev profile** Cargo uses when you `run cargo build`
  - and the **release profile** Cargo uses when you `run cargo build --release`
  - The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.

### Customizing Profiles

override a default setting by adding a different value for it in Cargo.toml.

Filename: Cargo.toml

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## documentation comment

- documentation comment will generate HTML documentation.
- support Markdown notation for formatting the text.

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

```sh
cargo doc --open # will build the HTML for your current crate’s documentation
```

### Documentation Comments as Tests

- running cargo test will run the code examples in your documentation as tests!


### Commenting Contained Items

- The style of doc comment `//!` adds documentation to the item that contains the comments rather than to the items following the comments.
- We typically use these doc comments inside the crate root file (src/lib.rs by convention) or inside a module to document the crate or the module as a whole.

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

## re-export items

- `pub use` can re-export items without changing the code structure.
- Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.

```rust
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

## workspaces

- A workspace is a set of packages that share the same Cargo.lock and output directory.
- good for managing large projects.

structure example

```text
├── Cargo.lock
├── Cargo.toml            // Root config
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

## Installing Binaries with cargo install

- Similar to npm i ... -g
- This command installs a Rust binary crate globally on your system.

example

```sh
cargo install ripgrep

# if $PATH is correctly configure to use binaries installed with cargo install

rg --help
```

default installation directory `$HOME/.cargo/bin`
