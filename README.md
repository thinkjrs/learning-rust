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

## Chapter 3. Normal Programming Stuff

Here I suggest a thorough reading of [Chapter 3 of The Rust Programming Language](), as I won't dive too in-depth on anything covered. This chapter mainly covers standard idioms modern languages have on a high-level basis, setting up further exploration later.

For now, let's record a few facts about the Rust programming language that are useful at this stage.

### Immutable variables

By default, variables are immutable in the Rust programming language. If you want to change them, you need to specify that to the compiler using `mut`.

For example,

```rust
let x: u32 = 42;
x = 41; // will not compile since you cannot re-assign to the immutable x
```

Here's how to make that mutable:

```rust
let mut x: u32 = 42;
x = 43;
```

> Rust has constants as well. Simply use `const` and `AN_ALL_CAPS_VARIABLE_NAME`.

### Shadowing

As mentioned before, the language allows developers to _shadow_ variable. If you simply reuse `let` the variable can be _shadowed_.

Shockingly, the following will compile and the compiler will forget the first value of `x`.

```rust
let x = 42;

let x = 42 + 42; // 2 * (meaning of life)
```

The variable `x` is still immutable and the compiler will complain if we assign to it without using `let`.

### Data types

Rust has integers, floating point numbers, booleans and a character type. [Read about those via the docs](https://rust-book.cs.brown.edu/ch03-02-data-types.html).

#### Integer overflow

Rust will throw a compiler error when you compile your program for `debug` that will overflow, e.g. when using `cargo run`.

Here's what that looks like:

```rust
let overflower: u8 = 4200;
println!("{}", overflower);
```

```
cargo run
...
error: literal out of range for `u8`
...
```

Yet again, the lane bumpers save us. However, what happens for release builds? Rust performs [_two's complement wrapping_](https://en.wikipedia.org/wiki/Two%27s_complement). All that means is that the value wraps around to 0 after the max has been reached.

For example, for a `u8` type, 256 becomes 0, 257 becomes 1 and 258 becomes 2. Easy.

> Do not rely on this behavior to write your programs. That's considered a design error.

#### String and `char` literals

String literals and character literals aren't the same. `char` literals start with a single quote while string literals a double quote.

The `char` type in Rust represents a Unicode scalar value so you can display accented letters, Japaneses, Chinese, Arabic and emojis. Anything UTF-8.

### Tuples and arrays

As usual, the tuple type is immutable and can hold different types in the same tuple.

```rust
struct Apple;
struct Orange;
struct ResponsiblySourcedTrout;

let tup: (Apple, Orange, u32, ResponsiblySourcedTrout) = (Apple, Orange, 3, ResponsiblySourcedTrout);
```

The array is a fixed size array you are probably used to from other languages. Here's what that looks like with some syntax sugar, too.

```rust
let my_array = [1, 2, 3];
let my_array = [3; 1028]; // 1028 elements all with the value of 3
```

### Statments and expressions

Rust is an expression-based language. We'll skip over basic "statements" like `let x = 42` and jump right to the meat on the bone.

An "expression" is a fundamental concept that represents a sequence of operations that computes a value. Expressions can consist of literals, variable references, operators, function calls, and control flow constructs among other components. Unlike statements, which perform actions but do not necessarily return a value, expressions always evaluate to a value and can be a part of other expressions.

Let's gloss over a few examples. Here's a simple one, where the block that evaluates to 42 is the expression.

```rust
let the_meaning_of_life = {
    42
};
```

And if we use the `println!` macro to print it, that's an expression too!

```rust
println!("{}", the_meaning_of_life)
```

Of note, expressions don't have semicolons after them. Adding a semicolon makes it a statement. Nuff said.

## Chapter 4. Ownership

Ownership is a concept that is somewhat unusual and not present in other languages. Of the ones I know well -- Python, C++, JavaScript (TypeScript), Matlab, R -- I've been exposed to something like this only with respect to Smart Pointers within the C++ standard library.

So what is _ownership_?

> "_Ownership_ is a set of rules that govern how a Rust program manages memory. [Rust] memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile. None of the features of ownership will slow down your program while it's running." - [Page 59 of the _Rust Programming Languages, 2nd Edition_](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)

### Stack versus heap

Without diving into the details of the modern computing machine, a Rust programmer needs to pay attention to and consider the _stack_ versus the _heap_.

Let's hit up two heuristic ways to think about these things.

#### The stack

The _stack_ is like a literal stack of empty, ready-to-use pizza boxes. Assume you make pizzas and the boxes are next to you. You just pulled a piping hot pepperoni pie out of the oven and need to put it somewhere. Well you grab a box right off the top of the pile next to you and plop that fresh pie right in it.

The stack is LIFO - last-in, first-out (like the pizza boxes on top).

#### The heap

The heap is like a giant [apple bobbing](https://en.wikipedia.org/wiki/Apple_bobbing) bucket full of water where all the apples are your data. When you allocated to the heap it's like putting your data in the bucket and it's floating. But imagine that it has an address carved into it like `0x2001FFE4`.

> `0x2001FFE4` is super boring, right? I agree. But if we didn't use hexadecimal, we'd run out of literal bit space for numbers quickly. Hell, 0x2001FFE4 is 537,001,956 in decimal representation. Every single thing every single program does has one of these. Imagine how quickly that runs up.

#### The stack is faster than the heap

Because the allocator can just store something on the top, the stack is much faster at storing data than the heap, where the allocator has to search for a spot in the bucket to put its apples (figuratively, of course).

I like to think of things like this: if I can statically allocate space it's vastly faster. But sometimes we need to dynamically allocate space, such as with user input.

> Yes, writing a program to input what things Donald Trump says would use a lot of heap allocation.

### Scope

Rust automatically returns memory once the variable that owns that memory goes out of scope. It has no garbage collector and you don't need to manage the memory yourself.

Yes, this is like bowling with the bumpers off but you never go into the gutter.

> You might recognize a pattern like this from C++ smart pointers called [RAII - Resource Acquisition Is Initialization](https://learn.microsoft.com/en-us/cpp/cpp/object-lifetime-and-resource-management-modern-cpp?view=msvc-160).

> _I know, I know...That's a Microsoft link. Use linux. Fck MSFT. Nah, they're cool. Cálmate! Satya made the company awesome._

#### The basics

Taking all the details out of it, which you should totally read from the actual proper rust book, let's dive into what you have to know.

- Primitive data types, like `let x = 5` or `let y = [3; 5]`, are on the stack and get copied when you do things like assign them to other variables.

- Complex and dynamic data types, on the other hand, are `drop`ed at the end of their lifetime, since they live on the heap.

```rust
// Stack-allocated variables
let x: i32 = 10; // `x` is an integer stored directly on the stack.
let y: bool = true; // `y` is a boolean value, also stored on the stack.

// Heap-allocated
let mut vec: Vec<i32> = Vec::new(); // `vec` is a struct with a pointer, length, and capacity, all of which are stored on the stack. The actual data of the vector is stored on the heap.
vec.push(42); // Add an element to the heap-allocated vector.
println!("Heap-allocated vector accessed through a stack-allocated struct: {:?}", vec);
```

Read this chapter thrice.

### References and borrowing

Creating a reference is called _borrowing_ in Rust. It's really, really important. You do it with placing an ampersand before a variable, e.g.

```rust
fn myfunc(my_string: &str) {}
```

It's [said best in the book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing) so here goes:

> "As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it."

You can't modify things you borrow, you can only use them, unless the owner knows (and agrees that) you're going to modify things.

Here's an example of a mutable reference:

```rust
let mut nose = String::from("my nose");

operate_on_nose(&mut nose);

fn operate_on_nose(something_on_which_to_perform_plastic_surgery: &mut String) {
    something_on_which_to_perform_plastic_surgery.push(" is different now");
}
```

### Slices

A unique type of reference in Rust is the _slice_ which is a representation of a sequence of elements that are contiguous.

You might be familiar with this concept from Python or other languages. Let's look at syntax and save the rest for later. In the meantime, read the [actual book on the topic.](https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type)

```rust
// Slice example
let s = String::from("hello world");

let hello = first_word(&s);

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```
