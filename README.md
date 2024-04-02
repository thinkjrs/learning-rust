# learning-rust

My journey learning the Rust programming language. Examples are probably not my own, but might be.

## Chapter 1. Getting Started

### Installing rustup on Linux

```
curl --proto 'https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

#### What's going on here

This command is used to download and install Rust via `curl`, a command-line tool for transferring data with URLs. It fetches a script and executes it immediately with `sh`, the Unix shell.

Let's break down this command:

1. **`curl`**: The command itself, a tool for transferring data from or to a server, using the https protocol.

2. **`--proto 'https'`**: This option tells `curl` to use only the HTTPS protocol. It restricts `curl` from attempting to use any other protocol that might normally be attempted in other circumstances.

3. **`--tlsv1.3`**: Specifies that `curl` should use TLSv1.3 as the cryptographic protocol for secure communication.

> TLS (Transport Layer Security) v1.3 is the latest version that provides security improvements over previous versions.

4. **`https://sh.rustup.rs`**: This is the URL from which `curl` will fetch data. In this case, it's a script provided by the Rust language maintainers to install `rustup`, the Rust toolchain installer.

5. **`-sSf`**: These are options combined together and passed to `curl`:

   - `-s` or `--silent`: Silent mode. Don't show progress meter or error messages. Makes Curl mute.
   - `-S` or `--show-error`: When used with `-s`, it makes `curl` show an error message if it fails.
   - `-f` or `--fail`: Tells `curl` to fail silently on server errors (when HTTP servers return a 4xx or 5xx error), preventing scripts or other erroneous data from being executed or processed if the requested URL points to an error page.

6. **`| sh`**: This part is known as a pipe (`|`). It takes the output of the preceding command (in this case, the script downloaded by `curl`) and passes it as input to the `sh` command, which is the command interpreter (or shell) that executes the script.

The overall command fetches the `rustup` installation script securely using HTTPS and TLSv1.3, and if successful, passes the script directly to the shell for execution. The use of `-sSf` ensures that the operation proceeds quietly but will show an error if something goes wrong, helping to maintain the cleanliness of the output and the security of the operation.

For more, see [the Rust installation docs](https://doc.rust-lang.org/book/ch01-01-installation.html).

### The first program

Let's create the classic "Hello, world!" program.

> The origin of "Hello, World!" can be traced back to the seminal book The C Programming Language by Brian Kernighan and Dennis Ritchie, published in 1978. This book, often referred to simply as "K&R," was instrumental in popularizing the C programming language and served as its de facto standard for years.

Assuming you're in a directory in which you're tracking your Rust learning, `mkdir hello_world && cd hello_world` to create a directory `hello_world` and move into it immediately after creation.

Now create a file `main.rs` inside of this directory. From now on, we'll use [vIM](https://www.vim.org/) since it's the editor I use.

`vim main.rs`

Now inside that file, type the following:

```rust
fn main() {
    println!("Hello, world!");
}
```

Format it:

`rustfmt main.rs`

And compile then run it:

`rustc main.rs && ./main`

You should see `Hello, world!` printed in your terminal.

Read more about [what's happening under the hood in the docs](https://doc.rust-lang.org/book/ch01-02-hello-world.html#anatomy-of-a-rust-program).

### Cargo FTW

Most Rust developers (Rustaceans) use the language's built-in package manager and build system called [Cargo](https://doc.rust-lang.org/cargo/) to build "real" programs within Rust.

So let's move from what we were working on to try out cargo, making the directory `hello_cargo` in the process: `cd ../ && cargo new hello_cargo && cd hello_cargo`.

Now open the `Cargo.toml` file via `vim Cargo.toml`. You'll see something like:

```rust
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

> As of April 1st, 2024, there are "three Rust editions are available: Rust 2015, Rust 2018, and Rust 2021. This book is written using Rust 2021 edition idioms." - [read the docs](https://doc.rust-lang.org/book/appendix-05-editions.html).

The `[dependencies]` section heading is where you'll add dependencies for your program, which you'll most certainly have writing anything of substance in the real world.

Close this file and notice that there's a `src/main.rs` path/file that the `cargo new` command created. This is the same `Hello, world!` program as before. Let's build it with `cargo`:

`cargo build`

Now let's run it:

`./target/debug/hello_cargo`

Let's do the same thing in one command:

`cargo run`

> Use `cargo check` to compile the code without outputing an executable, as it is much faster than a full compilation of the project!

#### Ready to release?

Releases are simple and only require the `--release` flag:

`cargo build --release`

Now you'll find the executable in `./target/release/hello_cargo` instead of `./target/debug/hello_cargo`.

## Chapter 2. A real (small) program

Rather than directly rehash what the official Rust book covers, let's dive
directly into the full code example.

### Code for "Guess the Number"

Start by running `cargo new guessing_game && cd guessing_game`.

Then open `src/main.rs` and add the following code.

```rust
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

        let guess: u32 = match guess.trim().parse() {
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
```

If you run this you'll get an error because `rand` is not a dependency yet.

### Add the crate `rand`

To make a random number that we have to guess we should add the [`rand` crate](https://crates.io/crates/rand) from the Rust team.

Run the following or edit your `Cargo.toml` directly:

```sh
cargo add rand@=0.8.5
```

### Play the `guessing_game`

Compile and run the program with the usual `cargo run` command.

### Line-by-line breakdown

The first three lines import dependencies for the program.

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;
```

Both `cmp::Ordering` and `io` are from the Standard Library, as you can see using the `std::` to import them.

We import the `Rng` trait from the `rand` crate we previously added as a dependency.

Inside the `main` function we then print out a welcome message and use the `Rng` trait from the `rand` crate to generate a random integer between 1 and 100 called `secret_number`.

Next you see the `loop` keyword which creates an infinite loop for the user to input a `guess` and compare that guess with the `secret_number` created above.

Inside the loop we first instantiate the `guess` variable as a string, which we then read from the standard input:

```rust
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

Now our `guess` variable is a string and we need to compare that to the `secret_number` variable, a numeric type. You can see this if you hover over `secret_number` in your editor.

To covert `guess` to an unsigned 32-bit integer (this is 1-100, afterall), we then do the following:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

For now we'll leave out an explanation of the `trim` and `parse` methods, as it's somewhat self-explanatory.

Next we use the `cmp` module's `Ordering` enum to match the user input `guess` with the generated random number `secret_number`:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("⬆️ Guess a bigger number, yours was too small!"),
    Ordering::Greater => println!("⬇️ Guess a smaller number, yours was too big!"),
    Ordering::Equal => {
        println!("🎯 Spot on!");
        break;
    }
}
```

### Awesome details

So let's dig into some of this code further and alter a few things, for the sake of example.

#### Rust's static type system

TO start, comment away the following code, save your file, and attempt to compile the program.

```rust
//let guess: u32 = match guess.trim().parse() {
//    Ok(num) => num,
//    Err(_) => continue,
//};
```

You'll notice an error right away that looks something like the below:

```sh

cargo run
   Compiling guessing_game v0.1.0 (/home/jason/repos/learning-rust/guessing_game)
error[E0308]: mismatched types
   --> src/main.rs:24:25
    |
24  |         match guess.cmp(&secret_number) {
    |                     --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
    |                     |
    |                     arguments to this method are incorrect
    |
    = note: expected reference `&String`
               found reference `&{integer}`
note: method defined here
   --> /home/jason/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:815:8
    |
815 |     fn cmp(&self, other: &Self) -> Ordering;
    |        ^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error
```

This tells you that you have `error[E0308]: mismatched types` and then spits those out exactly:

```sh
24  |         match guess.cmp(&secret_number) {
    |                     --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`

    = note: expected reference `&String`
               found reference `&{integer}`

```

_Expected `&String`, found `${integer}`. in `match guess.cmp(&secret_number)`._

Ah yes, that's right, without converting `guess` into an integer (which we commented away) we get a comparison error because `&secret_number` is a reference to an integer `${integer}`!

Notice how amazing Rust is with strong static typing, forcing you to fix the error before the program will run. And to boot, its errors are incredibly clear compared with many other statically typed languages, such as C and C++.

#### Shadowing variables

Now while we're on it, this `guess` variable was declared twice. What's up with that? Well Rust has a feature called [_shadowing_](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing) which allows the developer to reuse the variable name.

#### Type inference

Let's mess with `guess` some more. When you shadow the `guess` variable, annotating its type to `u32`, what happens to the type of `secret_number`?

First, hover over `secret_number` and notice that your editor says something like

```rust
// size = 4, align = 0x4
let secret_number: u32
```

Notice that this is the same type as `guess` after you annotate it. Now change the annotation on `guess` to `i32` and hover over `secret_number` again (you may need to save your file).

You get something like the following:

```rust
// size = 4, align = 0x4
let secret_number: i32
```

Notice that Rust automagically inferred the type change. Cool, right?

#### The `Result` enum

Staying on the `guess` shadowing, notice that we have a match and then some curly brackets with `Ok` and `Err` inside of them.

This is how we handle errors in Rust. `parse` returns a `Result` type which is an enum that has the variants `Ok` and `Err`. If the user input is a valid number then it will match `Ok` and simply return the number. If the user input is not a valid number the `Err(_)` catchall simply returns `continue`, which means ignore the error and loop again (asking the user for another number).
