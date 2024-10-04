# Rust CH15

## Smart Pointers

Implemented Traits

- `Deref` trait allows an instance of the smart pointer struct to behave like a reference
  - so you can write your code to work with either references or smart pointers
- `Drop` trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope

Common Smart Pointers

- `Box<T>` for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

### Reference and Pointers

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### `Box<T>`

- Rust's most basic pointer
  - Similar to `unique_ptr` from C++

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### Rc<T>

- abbreviation for reference counting.
- If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
- **only for use in single-threaded scenarios**
- `Rc<T>` allows **reading only**
  - immutable references
  - `RefCell<T>` can be used to work with this immutability restriction

- strong_count
- weak_count

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));         // no deep copy is made here
    let c = Cons(4, Rc::clone(&a));         // clone is implemented differently from other types
}                                           // it also increases the reference count
```

### Interior Mutability Pattern and Summary

- Borrowing Rules
  - At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
  - References must always be valid.

- Ownership:
  - Rc<T> enables multiple owners of the same data;
  - Box<T> and RefCell<T> have single owners.

- Borrowing Rules Check
  - Box<T> allows immutable or mutable borrows checked at compile time;
  - Rc<T> allows only immutable borrows checked at compile time;
  - RefCell<T> allows immutable or mutable borrows checked at runtime.

- Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

- With references, if you break these rules, you'll get a compiler error. 
- With RefCell<T>, if you break these rules, your program will panic and exit.

- The RefCell<T> type is useful when you're sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

### `RefCell<T>`

- single-threaded

## Deref Trait Implementation

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;                    // Associated types

    fn deref(&self) -> &Self::Target {  // returns a reference to the value to be accessed with * operator
        &self.0                         // .0 accesses the first value in a tuple struct
    }
}
```

- `*y` of type `MyBox`
- translates to
- `*(y.deref())`

## Deref Coercion

Rust does deref coercion when it finds types and trait implementations in three cases:

- From &T to &U when T: `Deref<Target=U>`
- From &mut T to &mut U when T: `DerefMut<Target=U>`
- From &mut T to &U when T: `Deref<Target=U>`

### Deref Coercion Example

```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // rust performs Deref Coercion here
}
```

if coercion did not exist, this would be needed

```rust
hello(&(*m)[..]);
```

- steps:
  - 1: `m.deref()`
  - 2: `&m_value[..]`

## Drop Implementation

- Called automatically on instance destruction
- Called in reverse order of construction... `Stack`

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

### Manual Drop call

- Rust doesn’t let you call the `Drop` trait’s drop method manually;
- instead you have to call the `std::mem::drop function` provided by the standard library.

```rust
// std::mem::drop
let c = CustomSmartPointer {
    data: String::from("some data"),
};
drop(c);
```