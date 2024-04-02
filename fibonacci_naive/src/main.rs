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
    let iterative_computed_fibonacci = fibonacci_iterative(input);
    let elapsed = now.elapsed();
    println!("Fibonacci number {input} computed using iteration is {iterative_computed_fibonacci}. Execution time: {elapsed:.2?}.");

    let input = input as usize;
    let now = Instant::now();
    let array_computed_fibonacci = fibonacci_array(input);
    let elapsed = now.elapsed();
    println!("Fibonacci number {input} computed using bottom up array method is {array_computed_fibonacci}. Execution time: {elapsed:.2?}.");
}

// basic recurssive definition
fn fibonacci_recursive(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

// iteration-based definition
fn fibonacci_iterative(n: u32) -> u32 {
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

// array-based (bottom up) definition
fn fibonacci_array(n: usize) -> usize {
    let mut fib: Vec<usize> = Vec::new();
    fib.push(0);
    fib.push(1);
    for i in 2..n + 1 {
        fib.push(fib.get(i - 1).unwrap().clone() + fib.get(i - 2).unwrap().clone())
    }
    fib.get(n).unwrap().clone()
}
