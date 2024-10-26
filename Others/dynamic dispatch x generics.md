# dynamic dispatch x generics

- [dynamic dispatch x generics](#dynamic-dispatch-x-generics)
  - [Reasons to use dynamic dispatch (trait objects)](#reasons-to-use-dynamic-dispatch-trait-objects)
  - [Reasons to prefer generics](#reasons-to-prefer-generics)
  - [Generally](#generally)

## Reasons to use dynamic dispatch (trait objects)

1. When you need to store different types in the same collection:
```rust
// With trait objects
let mut objects: Vec<Box<dyn Draw>> = vec![
    Box::new(Button::new()),
    Box::new(SelectBox::new())
];

// Not possible with generics:
let objects: Vec<T: Draw> = vec![] // Won't compile
```

2. When you don't know the concrete types at compile time:
```rust
// Dynamic dispatch works with runtime type determination
fn load_plugin(path: &str) -> Box<dyn Plugin> { ... }
```

3. When you want to reduce binary size:
```rust
// Generic version creates separate code for each type
fn process<T: MyTrait>(item: T) { ... }

// Dynamic dispatch shares one implementation
fn process(item: &dyn MyTrait) { ... }
```

## Reasons to prefer generics

1. Better performance (zero-cost abstraction)
2. Compile-time type checking
3. No runtime overhead
4. Inline optimization possible

## Generally

- Use generics when you know the types at compile time and want maximum performance
- Use dynamic dispatch when you need runtime flexibility or want smaller binary size
