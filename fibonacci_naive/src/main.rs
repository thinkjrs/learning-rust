use std::io;
use std::time::Instant;

fn main() {
    println!("Let's compute the value of the fibonacci sequence. Enter a number.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("That wasn't a number!"),
    };

    let now = Instant::now();
    let recursive_computed_fibonacci = fibonacci_recursive(input);
    let elapsed = now.elapsed();
    println!("Fibonacci number {input} computed recursively is {recursive_computed_fibonacci}. Execution time: {elapsed:.2?}.");

    let now = Instant::now();
    let array_computed_fibonacci = fibonacci_array(input);
    let elapsed = now.elapsed();
    println!("Fibonacci number {input} computed using an array is {array_computed_fibonacci}. Execution time: {elapsed:.2?}.");
}

// basic recurssive definition
fn fibonacci_recursive(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

// array-based definition
fn fibonacci_array(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        let mut a = 0;
        let mut b = 1;

        for _i in 2..n + 1 {
            let c = a + b;
            a = b;
            b = c;
        }

        b
    }
}
