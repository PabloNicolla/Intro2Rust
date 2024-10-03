# Rust CH06

## Enums

```rust
enum IpAddrKind {
    V4, // variant
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

## Option Enum

- Rust does not implement `null`
- However, it does have `enum Option<T>`
  - A value can be present with Some(my_value)
  - A value can be absent with None

```rust
enum Option<T> {
    None,
    Some(T),
}

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

> [!IMPORTANT]
> You have to convert an `Option<T>` to a `T` before you can perform `T` operations with it.

## Match and Enums

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // use MyEnum::MyVariant to match the variant
        Some(i) => Some(i + 1), // use MyEnum::MyVariant(variable_name) to access data stored by the variant
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### Catch-all Pattern

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // covers every other possible value
    // _ => reroll(), // `_` covers every other possible value, but tells Rust that this variable will not be used
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

## if let

- syntactic sugar for match
- useful when considering only one possible value

```rust
let config_max = Some(3u8);
if let Some(max) = config_max { // if config_max is Some; execute logic else ignore
    println!("The maximum is configured to be {max}");
}
```