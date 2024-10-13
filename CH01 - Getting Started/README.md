# Rust CH1

- [Rust CH1](#rust-ch1)
  - [Basic Commands](#basic-commands)
    - [Compiling](#compiling)
    - [Formatting](#formatting)
    - [Cargo](#cargo)
    - [Debug](#debug)

## Basic Commands

```sh
rustc --version
```

```sh
rustup update
```

```sh
rustup self uninstall
```
### Compiling

```sh
rustc main.rs
```

### Formatting

```sh
rustfmt main.rs
```

### Cargo

```sh
cargo --version
```

```sh
cargo new hello_cargo
```

```sh
cargo build
```

```sh
cargo run
```

```sh
cargo check
```

```sh
cargo build --release
```

### Debug

- get compiled/expanded code. Good for debugging macros

```sh
cargo expand >> expanded.rs
```
