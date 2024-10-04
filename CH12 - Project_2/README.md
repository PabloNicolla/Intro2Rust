# Rust CH12

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