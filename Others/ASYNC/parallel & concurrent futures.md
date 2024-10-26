# parallel & concurrent futures

- [parallel \& concurrent futures](#parallel--concurrent-futures)
  - [Code Example](#code-example)
  - [Explanation](#explanation)

## Code Example

```rust
use futures::future::{join, try_join, join_all, select};
use tokio;

async fn example_patterns() {
    // 1. join! macro - waits for all futures to complete
    let (result1, result2) = join!(ope1(), ope2());
    // ... use result1 and result2
    
    // 2. try_join! macro - for futures that return Result
    // Stops if any future returns an error
    let (result1, result2) = try_join!(
        async { Ok::<_, Error>(ope1().await) },
        async { Ok::<_, Error>(ope2().await) }
    )?;
    
    // 3. FuturesUnordered - for dynamic collection of futures
    use futures::stream::{StreamExt, FuturesUnordered};
    
    let mut futures = FuturesUnordered::new();
    futures.push(ope1());
    futures.push(ope2());
    
    while let Some(result) = futures.next().await {
        // Handle each result as it completes
    }
    
    // 4. select! macro - races futures against each other
    use tokio::select;
    
    select! {
        result1 = ope1() => {
            println!("ope1 finished first");
        }
        result2 = ope2() => {
            println!("ope2 finished first");
        }
    }
    
    // 5. spawn + join_handle - for true parallelism with threads
    let handle1 = tokio::spawn(ope1());
    let handle2 = tokio::spawn(ope2());
    
    let (result1, result2) = join!(handle1, handle2);
    let result1 = result1.expect("task 1 panicked");
    let result2 = result2.expect("task 2 panicked");
}

// Example with error handling using try_join!
async fn example_with_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug)]
    struct CustomError;
    
    async fn fallible_ope1() -> Result<i32, CustomError> {
        Ok(1)
    }
    
    async fn fallible_ope2() -> Result<String, CustomError> {
        Ok("success".to_string())
    }
    
    let (num, text) = try_join!(
        fallible_ope1(),
        fallible_ope2()
    )?;
    
    println!("Got {} and {}", num, text);
    Ok(())
}
```

## Explanation

Here are the main approaches to handle concurrent futures, from simplest to more complex:

1. `join!` macro: Simplest way to run multiple futures concurrently and wait for all of them
   ```rust
   let (result1, result2) = join!(ope1(), ope2());
   ```

2. `try_join!`: Similar to `join!` but for futures that return `Result`
   - Stops at first error
   - All futures must return the same error type

3. `FuturesUnordered`: For dynamic collections of futures
   - Processes futures as they complete
   - Good when you don't know the number of futures in advance

4. `select!`: Races futures against each other
   - Completes when the first future finishes
   - Can handle the results differently based on which completes first

5. `select! + loop`: 4. variant: tries to complete all futures
   ```rust
    loop {
        select! {
            //...
        }
    }
   ```

6. `spawn + join_handle`: For true parallelism
   - Creates new tasks that can run on different threads
   - More overhead but better for CPU-bound tasks
