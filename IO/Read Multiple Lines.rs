/*
Explanation

- Method 1 is great for handling input line by line and acting on each line independently.
- Method 2 is useful if you need to collect all lines into a list for later processing.
- Method 3 is ideal for reading all input as a single String, which can then be split by lines if needed.
*/

use std::io::{self, BufRead};

fn m1() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                println!("Read line: {}", line);
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------------------

use std::io::{self, BufRead};

fn m2() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok) // filters out any lines with errors
        .collect();

    // Print all lines at once
    for line in &lines {
        println!("Line: {}", line);
    }
}

// -------------------------------------------------------------------------------------------------------------

use std::io::{self, Read};

fn m3() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("All input:\n{}", input);
}
