/*
Explanation

- Method 1 is the simplest and most common way to read a single line.
- Method 2 is useful if you want to handle errors or have more control over the iterator.
- Method 3 is less conventional but works in cases where you’re reading from a buffer and expect only one line.
*/

use std::io::{self, Write};

fn m1() {
    let mut input = String::new();
    print!("Enter a line: ");
    io::stdout().flush().unwrap(); // Flush to ensure prompt appears before input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input.trim());
}

// -------------------------------------------------------------------------------------------------------------

use std::io::{self, BufRead};

fn m2() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().expect("No line found");

    match line {
        Ok(line) => println!("You entered: {}", line),
        Err(error) => eprintln!("Error reading line: {}", error),
    }
}

// -------------------------------------------------------------------------------------------------------------

use std::io::{self, Read};

fn m3() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    // Since `read_to_string` reads until EOF, you may want to split by newline
    let line = input.lines().next().unwrap_or("");
    println!("You entered: {}", line);
}
