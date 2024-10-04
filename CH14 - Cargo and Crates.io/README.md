# Rust CH14

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

## Crates.io

