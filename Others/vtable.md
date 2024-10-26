# vtables for dynamic dispatch

- [vtables for dynamic dispatch](#vtables-for-dynamic-dispatch)
  - [Introduction](#introduction)

## Introduction

Rust builds vtables for dynamic dispatch when using `dyn Trait` objects. Here's how it works:

1. When you create a `dyn Trait` object, Rust creates a "fat pointer" consisting of:
   - A pointer to the actual data
   - A pointer to the vtable

2. The vtable is created at compile time and contains:
   - The size of the concrete type
   - Alignment of the concrete type
   - Destructor function pointer
   - Function pointers for all trait methods

Here's a simple example:

```rust
trait Animal {
    fn make_sound(&self);
    fn get_name(&self) -> String;
}

struct Dog {
    name: String
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
    
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

// When we create a dyn Animal, Rust creates a vtable
let dog = Dog { name: "Rover".to_string() };
let animal: Box<dyn Animal> = Box::new(dog);
```

The vtable for `Dog` as `Animal` would contain:

- Size of `Dog`
- Alignment of `Dog`
- `Dog`'s destructor
- Pointer to `Dog`'s implementation of `make_sound`
- Pointer to `Dog`'s implementation of `get_name`

This is why dynamic dispatch has a small runtime cost compared to static dispatch (like with generics) - there's an extra pointer indirection through the vtable to call methods.
