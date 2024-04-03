use std::io;
use std::time::Instant;

const M: [[usize; 2]; 2] = [[1, 1], [1, 0]];

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

    let now = Instant::now();
    let matrix_computed_fibonacci = fibonacci_matrix(input);
    let elapsed = now.elapsed();
    println!("Fibonacci number {input} computed using matrix method is {matrix_computed_fibonacci}. Execution time: {elapsed:.2?}.");
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
    match n {
        0 => return 0,
        1 => return 1,
        _ => {}
    }

    let mut fib: Vec<usize> = vec![0; n + 1];

    fib[1] = 1;

    for i in 2..=n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    fib[n]
}

fn fibonacci_matrix(n: usize) -> usize {
    let mut fib: [[usize; 2]; 2] = [[1, 1], [1, 0]];
    if n != 0 {
        power(&mut fib, n - 1);
        fib[0][0]
    } else {
        n
    }
}

fn power(fib: &mut [[usize; 2]; 2], n: usize) {
    if n > 1 {
        power(fib, n / 2);
        multiply(fib, *fib);
        if n % 2 != 0 {
            multiply(fib, M);
        }
    } else {
        return;
    }
}

fn multiply(fib: &mut [[usize; 2]; 2], mib: [[usize; 2]; 2]) {
    let x = fib[0][0] * mib[0][0] + fib[0][1] * mib[1][0];
    let y = fib[0][0] * mib[0][1] + fib[0][1] * mib[1][1];
    let z = fib[1][0] * mib[0][0] + fib[1][1] * mib[1][0];
    let w = fib[1][0] * mib[0][1] + fib[1][1] * mib[1][1];
    fib[0][0] = x;
    fib[0][1] = y;
    fib[1][0] = z;
    fib[1][1] = w;
}
