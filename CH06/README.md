# Rust CH06

## Enums

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### Enums with custom data

```rust
enum IpAddr {
    V4(String), // each enum can have a different data structure V4(u8, u8, u8, u8), V6(String)
    V6(String), // enums can also hold structs
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

### Enum Method Implementation

- Same as structs

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```