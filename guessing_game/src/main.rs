use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("👋 Welcome to 'Guess the number!'");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("⬆️ Guess a bigger number, yours was too small!"),
            Ordering::Greater => println!("⬇️ Guess a smaller number, yours was too big!"),
            Ordering::Equal => {
                println!("🎯 Spot on!");
                break;
            }
        }
    }
}
