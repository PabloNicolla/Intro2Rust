# dyn Trait

- [dyn Trait](#dyn-trait)
  - [`&dyn Trait` is better when](#dyn-trait-is-better-when)
  - [`Box<dyn Trait>` is better when](#boxdyn-trait-is-better-when)

## `&dyn Trait` is better when

1. You only need temporary access to the trait object
2. The concrete type's size is known at the call site
3. You want to avoid heap allocation for better performance

```rust
fn print_shape(shape: &dyn Display) {  // Good: just needs temporary access
    println!("{}", shape);
}

fn process_iterator(iter: &dyn Iterator<Item = u32>) {  // Good: temporary usage
    for item in iter {
        // do something
    }
}
```

## `Box<dyn Trait>` is better when

1. You need to store the trait object in a struct
2. You need to return trait objects from functions
3. You need owned data rather than borrowed references
4. The size of the concrete type isn't known at compile time

```rust
// Must use Box because we're storing trait objects
struct DrawableCollection {
    shapes: Vec<Box<dyn Draw>>,  
}

// Must use Box because we're returning a trait object
fn create_random_shape() -> Box<dyn Shape> {
    if rand::random() {
        Box::new(Circle::new())
    } else {
        Box::new(Square::new())
    }
}

// Must use Box in async contexts
async fn process_handler(handler: Box<dyn Handler>) {
    handler.handle().await;
}
```

The key consideration is ownership and lifetime. If you just need to borrow and use the trait object temporarily, `&dyn Trait` is more efficient. If you need to own it or store it somewhere, you'll need `Box<dyn Trait>`.
