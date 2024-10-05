# Rust CH17

- [Rust CH17](#rust-ch17)
  - [OOP](#oop)
    - [Encapsulation](#encapsulation)
    - [Inheritance](#inheritance)
    - [Polymorphism](#polymorphism)

## OOP

structs and enums have data, and impl blocks provide methods on structs and enums.

### Encapsulation

use `pub` to make data accessible or not

### Inheritance

Rust does not have inheritance, but you can use `traits` to have a somewhat similar behavior...

> There is no way to define a struct that inherits the parent struct’s fields and method implementations without using a macro.

### Polymorphism

- Enabled via `traits`
- We create a `trait` object by specifying some sort of pointer, such as a `& reference` or a `Box<T> smart pointer`, then the `dyn keyword`, and then specifying the relevant `trait`. 

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,         // Somewhat similar behavior to abstract classes or interfaces from different languages
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// -- snip --

impl Draw for YourStruct {
    fn draw(&self) {
        // code to actually draw...
    }
}
```