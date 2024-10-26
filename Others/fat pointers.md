# Fat Pointers

- [Fat Pointers](#fat-pointers)
  - [Introduction](#introduction)
  - [Slice pointers (`&[T]`): These contain](#slice-pointers-t-these-contain)
  - [Trait object pointers (`dyn Trait`): These contain](#trait-object-pointers-dyn-trait-these-contain)

## Introduction

A fat pointer in Rust is a special kind of pointer that contains not just the memory address (like a regular pointer), but also additional metadata needed to work with the pointed-to value. There are two main types of fat pointers in Rust:

## Slice pointers (`&[T]`): These contain
   - A pointer to the first element
   - The length of the slice

```rust
let array = [1, 2, 3, 4, 5];
let slice = &array[1..4];  // Creates a fat pointer
// The slice pointer contains both:
// - Address of element '2'
// - Length of 3
```

## Trait object pointers (`dyn Trait`): These contain
   - A pointer to the instance
   - A pointer to the virtual method table (vtable)

```rust
trait Animal {
    fn make_sound(&self);
}

let animal: Box<dyn Animal> = Box::new(Dog);  // Creates a fat pointer
// The trait object pointer contains:
// - Address of the Dog instance
// - Pointer to Animal vtable for Dog
```

Fat pointers are twice the size of regular pointers (16 bytes on 64-bit systems instead of 8 bytes) because they need to store this extra information.
