# Rust CH20

## Temporary right hand values

```rust
let job = receiver.lock().unwrap().recv().unwrap();
```

using just `let`: any temporary values used in the expression on the **right hand side** of the equals sign are **immediately dropped** when **the let statement ends**.

However,

```rust
while let /*...*/

// and

if let /*...*/ 

// and

match /*...*/
```

these do **not drop temporary values until the end of the associated block**.