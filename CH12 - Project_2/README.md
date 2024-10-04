# Rust CH12

- [Rust CH12](#rust-ch12)
  - [Program Arguments](#program-arguments)
  - [Reading Environment Variables](#reading-environment-variables)
  - [stderr](#stderr)

## Program Arguments

- similar to argc and argc

```sh
cargo run # filename

cargo run -- needle haystack # filename, "needle", "haystack"
```

## Reading Environment Variables

BASH

```sh
IGNORE_CASE=1

cargo run -- to poem.txt
```

POWERSHELL

```ps1
$Env:IGNORE_CASE=1

cargo run -- to poem.txt

Remove-Item Env:IGNORE_CASE
```

```rust
let ignore_case = env::var("IGNORE_CASE")/*...*/;
```

## stderr

- `println!` prints to stdout
- `eprintln!` prints to stderr