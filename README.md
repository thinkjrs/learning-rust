# learning-rust

<!-- vim-markdown-toc GFM -->

* [Chapter 1. Getting Started](#chapter-1-getting-started)
    * [Installing rustup on Linux](#installing-rustup-on-linux)
        * [What's going on here](#whats-going-on-here)
    * [The first program](#the-first-program)
    * [Cargo FTW](#cargo-ftw)
        * [Ready to release?](#ready-to-release)
* [Chapter 2. A real (small) program](#chapter-2-a-real-small-program)
    * [Code for "Guess the Number"](#code-for-guess-the-number)
    * [Add the crate `rand`](#add-the-crate-rand)
    * [Play the `guessing_game`](#play-the-guessing_game)
    * [Line-by-line breakdown](#line-by-line-breakdown)
    * [Awesome details](#awesome-details)
        * [Rust's static type system](#rusts-static-type-system)
        * [Shadowing variables](#shadowing-variables)
        * [Type inference](#type-inference)
        * [The `Result` enum](#the-result-enum)
* [Chapter 3. Normal Programming Stuff](#chapter-3-normal-programming-stuff)
    * [Immutable variables](#immutable-variables)
    * [Shadowing](#shadowing)
    * [Data types](#data-types)
        * [Integer overflow](#integer-overflow)
        * [String and `char` literals](#string-and-char-literals)
    * [Tuples and arrays](#tuples-and-arrays)
    * [Statments and expressions](#statments-and-expressions)
* [Chapter 4. Ownership](#chapter-4-ownership)
    * [Stack versus heap](#stack-versus-heap)
        * [The stack](#the-stack)
        * [The heap](#the-heap)
        * [The stack is faster than the heap](#the-stack-is-faster-than-the-heap)
    * [Scope](#scope)
        * [The basics](#the-basics)
    * [References and borrowing](#references-and-borrowing)
    * [Slices](#slices)
* [Chapter 5. Structs](#chapter-5-structs)
    * [Using the `DisplayAd` struct](#using-the-displayad-struct)
    * [Instantiating another one](#instantiating-another-one)
    * [Structs without field names](#structs-without-field-names)
    * [Methods inside of structs](#methods-inside-of-structs)
* [Chapter 6. Enums and `match`](#chapter-6-enums-and-match)
    * [Modeling an Ad system](#modeling-an-ad-system)
    * [Using the `Ad` struct](#using-the-ad-struct)
    * [`Option<T>`](#optiont)
* [Chapter 7. Crates, packages and modules](#chapter-7-crates-packages-and-modules)
    * [Crates](#crates)
        * [Binary crates](#binary-crates)
        * [Library crates](#library-crates)
* [Chapter 8. Vectors, Strings and Hash Map Collections](#chapter-8-vectors-strings-and-hash-map-collections)
    * [Vectors](#vectors)
    * [Strings](#strings)
    * [Hash Maps](#hash-maps)
        * [Create your first `HashMap`](#create-your-first-hashmap)
        * [Insert some tickers and prices](#insert-some-tickers-and-prices)
        * [Get the price of GILD](#get-the-price-of-gild)
        * [Update values](#update-values)
            * [Overwrite](#overwrite)
            * [Upsert](#upsert)
            * [Update](#update)
    * [Collections Projects](#collections-projects)
        * [Median, mode, mean](#median-mode-mean)
        * [Pig latin strings](#pig-latin-strings)
        * [Stocks to a portfolio](#stocks-to-a-portfolio)

<!-- vim-markdown-toc -->

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

## Chapter 5. Structs

Structs hold related attribute data where the attributes have names, similar to tuples but named, so that it's clear what the value(s) of each attribute means.

Here's what a struct looks like:

```rust
#[derive(Debug)] // this is for printing using "{:?}" or "{:#?}"
struct DisplayAd {
    start_timestamp: i64,
    budget: u32,
    title: String,
    copy: String,
    call_to_action: String,
    media_asset_urls: Vec<String>,
    button_text: String,
    target_url: String,
}
```

### Using the `DisplayAd` struct

The `DisplayAd` struct above can be used like the below.

```rust
use chrono;

fn main() {
    let start_timestamp: i64 = chrono::Utc::now().timestamp();
    let mut my_ad = DisplayAd {
        start_timestamp,
        budget: 5000,
        title: String::from("My first ad"),
        copy: String::from("Buy whatever I'm selling. It's great!"),
        call_to_action: String::from("On sale today only!"),
        button_text: String::from("Buy now"),
        target_url: String::from("https://tincre.com/agency"),
        media_asset_urls: vec![String::from("https://https://res.cloudinary.com/tincre/video/upload/v1708121578/nfpwzh1oslr8qhdyotzs.mov")],
    }
}
```

There are a few things happening in the above. First of all, the usage of mutability `mut` is arbitrary and not required. Secondly, we're using [_field init shorthand_](https://rust-book.cs.brown.edu/ch05-01-defining-structs.html#using-the-field-init-shorthand) syntax to list the parameter/field name `start_timestamp`.

> We can only use _field init shorthand_ when the variable name and the struct field name are exactly the same.

### Instantiating another one

We can use some more shorthand syntax for instantiating another `DisplayAd`, which has some quirks we'll cover.

```rust
    let mut my_ad2 = DisplayAd {
        start_timestamp,
        budget: 1250,
        title: my_ad.title,
        call_to_action: my_ad.call_to_action,
        button_text: String::from("Don't use it"),
        target_url: String::from("https://truthsocial.com"),
        ..my_ad // no comma after this
    }
```

Now firstly, notice that we used `my_ad.title`, the `title` field from the first `my_ad` `DisplayAd` instantiation. Importantly, when we do this, the ownership for `my_ad.title` is moved to `my_ad2.title`. That means you can't use `my_ad` anymore!

Secondly, at the very end we use _struct update syntax_ to add the remaining items from `my_ad` that we didn't specify. This must come last and cannot have a trailing comma.

### Structs without field names

We can also instantiate structs without field names. For example,

```rust
struct Coordinates(f64, f64);

fn main() {
    let location = Coordinates(19.3937, 99.1746);
}
```

These can be useful if you want a tuple that comes with all the other goodies of structs.

### Methods inside of structs

One of the goodies that structs provide is the ability to place a method expression inside of them, just like a function but only within the context of the struct definition.

> This is available for traits and enums, too.

This can help massively with readability. Here's an example, assuming our `DisplayAd` struct from above.

```rust
const AVG_CPM: f64 = 3.2;

impl DisplayAd {
    fn calculate_estimated_impressions(&self) -> f64 {
        (self.budget as f64 / AVG_CPM) * 1000
    }
}

fn main {
    println!("{my_ad2.calculate_estimated_impressions()} impressions are expected for spend of {my_ad2.budget} USD")
}
```

We call these _associated functions_ in the Rust language. It's common for a struct to implement a `new` function that creates the struct. All the usual borrowing/ownership rules apply.

## Chapter 6. Enums and `match`

In covering enums we'll stick with our ads modeling from above. So what's an enum, you ask?

An enum (short for enumeration) in Rust allows you to define a type by enumerating its possible values. Each of these possible values is known as a variant. Variants of an enum can carry data (_similar_ to fields in a struct) and can have different types and amounts of associated data.

Basically, use enums when you want to model the context of your data and can enumerate it. Then use structs to actually hold that data.

### Modeling an Ad system

Revisiting our `DisplayAd` struct from above, the `Ad` enum below shows how we might use it and other structs.

```rust
enum Ad {
    Display(DisplayAd),
    Hover(HoverAd),
    Feed(FeedAd),
    Video(VideoAd),
    InlineText(InlineTextAd),
}
```

Here we've added other `*Ad` structs and enumerated them inside an enum. So we can use the `Ad` enum and reason about _what kind of ad_ we're dealing with, having the data separate from the actual reasoning mechanism itself.

_Note: you don't have to use structs to store data inside an enum._ You can store it directly. Here's what that looks like:

```rust
enum Fruit {
    Apple(String),
    Grapes(String),
}
fn main {
    let washington_apple = Fruit::Apple(String::from("Washington"));
    let green_grapes = Fruit::Grapes(String::from("Green"));
    let red_grapes = Fruit::Grapes(String::from("Red"));
}
```

### Using the `Ad` struct

Back on our struct to model types of Ads, one advantage is that we can write methods, like with structs, but that operate on all the different types of ads.

And inside those methods we can use an extremely powerful control flow in Rust called `match`, which allows you to execute code based on pattern matches, made up:

- Literals
- Destructured arrays, enums, structs, or tuples
- Variables
- Wildcards
- Placeholders

This is directly from the [Rust book section](https://doc.rust-lang.org/book/ch18-00-patterns.html#patterns-and-matching).

So let's use methods with `match` to do some setup for our `Ad`s.

```rust
impl Ad {
    fn init(&self) {
        match self {
            Ad::Display(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
            Ad::Hover(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
            Ad::Feed(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
            Ad::Video(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
            Ad::InlineText(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
        }
    }
}

fn main {
    let start_timestamp: i64 = chrono::Utc::now().timestamp();
    let my_display_ad = Ad::Display(DisplayAd {
        start_timestamp,
        budget: 5000,
        title: String::from("My first ad"),
        copy: String::from("Buy whatever I'm selling. It's great!"),
        call_to_action: String::from("On sale today only!"),
        button_text: String::from("Buy now"),
        target_url: String::from("https://tincre.com/agency"),
        media_asset_urls: vec![String::from("https://https://res.cloudinary.com/tincre/video/upload/v1708121578/nfpwzh1oslr8qhdyotzs.mov")],
    });
    my_display_ad.init();

    // and another ad
    let my_text_ad = Ad::InlineText(InlineTextAd {
        start_timestamp,
        budget: 5000,
        title: String::from("My first ad"),
        copy: String::from("Buy whatever I'm selling. It's great!"),
        call_to_action: String::from("On sale today only!"),
        target_url: String::from("https://tincre.com/agency"),
    });
    my_text_ad.init();
}
```

We created an `init` method on the `Ad` enum type that `match`es the corresponding ad struct that actually holds our data. Now we have two ads ready to rock and initialized custom to the kind of ad each represents.

### `Option<T>`

Rust has a built-in enum called `Option<T>` to represent the presence or absence of value. It is designed to avoid null references, a common source of errors in other programming languages. It has two variants: `Some(T)`, indicating the presence of a value of type `T`, and `None`, indicating the absence of a value.

Along with control flow like `match` this can be very useful. For example,

```rust
fn did_eat_fruit(fruit: Option<&str>) -> bool {
    match fruit {
        None => false,
        _ => true,
    }
}

fn main() {
    let apple = Some("Apple");
    let banana = None;
    let monkey_eating_status = if did_eat_fruit(apple) {
        "ate"
    } else {
        "did not eat"
    };
    println!("The monkey {monkey_eating_status}.");

    let monkey_eating_status = if did_eat_fruit(banana) {
        "ate"
    } else {
        "did not eat"
    };
    println!("The monkey {monkey_eating_status}.");
}
```

If we didn't handle the `None` case in the function the compiler would have screamed at us before we compiled and caused a runtime bug. This is a fantastic safety feature of Rust. It forces the developer to handle the type explicitly, always.

## Chapter 7. Crates, packages and modules

Aside from a cursory review and demonstration here, we'll dive into these concepts in more depth with an actual project, later.

In a nutshell, Rust's _module system_ consists of (these are directly from [the docs](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)):

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

### Crates

The smallest compiled piece of code Rust can consider is called a _crate_, which can contain modules, which can be defined in other files or places in the codebase that get compiled with the crate.

There are two types of crates, _binary_ and _library_ crates. A package can, and often does, have both.

#### Binary crates

A binary crate has a `main` function that you can use to actually run an executable, typically located inside `<my-project-name>/src/main.rs`.

You create a package using a binary crate via the cargo `new` command, e.g. `cargo new <my-project-name>`.

#### Library crates

Library crates define functionality to be used elsewhere. Often these are published to crates.io to be shared publicly.

You create a package with a library crate using the `--lib` flag with Cargo's `new` command, e.g. `cargo new <my-project-name> --lib`.

This creates a file under `<my-project-name>/src/lib.rs`.

## Chapter 8. Vectors, Strings and Hash Map Collections

The Rust standard library has a number of collections available for use, data structures that store data on the heap and can be grown or shrunk during runtime.

### Vectors

Vectors are datastructures that store multiple values next to each other in memory. You should use them when you have a list of things to store.

Below are some snippets demonstrating how to use them.

```rust
// create and push to the vector
let mut my_vec: Vec<u8> = Vec::new();
my_vec.push(0);
my_vec.push(1);
my_vec.push(1);
my_vec.push(0);

println!("{:?}", my_vec);
```

Use the convenience macro:

```rust
let my_vec2: Vec<u8> = vec![0, 1, 1, 0];
```

Vectors can also take types stored in enums.

```rust
#[derive(Debug)]
pub struct TextAd {
    ad_text: String,
    budget: u32,
    target_url: String,
}
#[derive(Debug)]
pub struct VideoAd {
    ad_title: String,
    budget: u32,
    target_url: String,
}

impl TextAd {
    pub fn new(ad_text: String, budget: u32, target_url: String) -> TextAd {
        TextAd {
            ad_text,
            budget,
            target_url,
        }
    }
}

impl VideoAd {
    pub fn new(ad_title: String, budget: u32, target_url: String) -> VideoAd {
        VideoAd {
            ad_title,
            budget,
            target_url,
        }
    }
}

#[derive(Debug)]
pub enum Ad {
    Text(TextAd),
    Video(VideoAd),
}
fn main() {
    let mut ads = vec![
        Ad::Video(VideoAd::new(
            String::from("Test video title"),
            1000,
            String::from("https://tincre.com"),
        )),
        Ad::Text(TextAd::new(
            String::from("Test text"),
            1250,
            String::from("https://tincre.com/agency"),
        )),
    ];

    println!("{:?}", ads);
}
```

### Strings

Strings in Rust may seem strange to those coming from dynamic languages such as Python or JavaScript. If coming from C-family languages, the way Rust treats strings may seem refreshing, as C-family developers consistently deal with the complexities a "string" presents.

Rust has the primitive [`char`](https://doc.rust-lang.org/std/primitive.char.html) type which is defined to represent a [Unicode scalar value](https://www.unicode.org/glossary/#unicode_scalar_value). It is always 4 bytes long and its syntax is represented by two enclosed single quotes `'`, e.g. `'c'`.

However, the `char` type _is not_ how strings are represented in Rust; a String is better thought of as a vector, in fact, a `vec<u8>` with some extras and restrictions.

You can create a string using the familiar `::new` or the string-specific `::from` functions, if you'd like to create your string from a string literal directly.

```rust
fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello, ");
    let s2 = String::from("world!");
}
```

> String literals also have a `to_string()` method developers can use to return a String.

Similarly to `Vec<T>` a String can have modifiable sizes and contents. It's important to remember that borrow and move operations apply here. The example below demonstrates this, asuming our `s1 and s2` variables from directly above.

```rust
fn main() {
    let s3 = s1 + &s2; // note s1 has been moved and is no longer be usable
    println!("{}", s3);
    //println!("{}", s1);
}
```

If you uncomment the //println! in your editor you'll see something like the screenshot below.

![s1 cannot be used since it was moved to s3](https://res.cloudinary.com/thinkjrs-dev/image/upload/v1712611681/learning-rust/Screenshot_from_2024-04-08_15-25-00_apvinn_zqghlj.webp).

```rust
fn main() {
    let mut s = format!("{s3} You are crazy, {s2}");
    let not_owned = "blah";
    s.push_str(not_owned);
    println!("Pushed: {}", s);
    println!("This isn't owned: {}", not_owned);
}
```

You can also slice a string to get particular _bytes_:

```rust
fn main() {
    let slice_entire = &s3[..];

    // Borrow a reference to part of the String
    let slice_part = &s3[0..5];

    println!("Entire slice: {}", slice_entire); // Prints "Hello, world!"
    println!("Part of slice: {}", slice_part); // Prints "Hello"
}
```

> Be very careful using ranges to index strings because these can become out of bounds.

In particular with regard to slicing, you can't index a String in Rust.

If you want to operate on pieces of String collections you should use iterators, of which there are two `chars` and `bytes`.

```rust
fn main() {
    for c in not_owned.chars() {
        println!("Character: {}", c);
    }

    for b in not_owned.bytes() {
        println!("Bytes: {}", b);
    }
}
```

### Hash Maps

Key-value storage in Rust is typically accomplished with the `HashMap` standard libary collection. Different from `Vector`s and `String`s you need to first `use` the collection.

For example,

```rust
use std::collections::HashMap;
```

Specifically, `HashMap<K, V>` maps keys (of type `K`) to values (type `V`) using the DoS-proof hashing function _SipHash_, created in 2012 after a slew of attacks on hash tables.

#### Create your first `HashMap`

We create hash maps via the standard `::new` constructor.

```rust
use std::collections::HashMap;

fn main() {
    let mut prices = HashMap::new();
}
```

#### Insert some tickers and prices

Let's insert some stock ticker symbols and fake prices. 

```rust
fn main() {
    let stock_ticker_1 = "AAPL";

    prices.insert(stock_ticker_1, 163.23);

    prices.insert(stock_ticker_1, 163.23);

    prices.insert("GILD", 66.76);
}
```
> These are real tickers for Apple (AAPL) and Gilead Sciences (GILD).
#### Get the price of GILD

Now let's extract those values and do something with them, like print to the console.

```rust
fn main() {
    let ticker_symbol = "GILD";

    let gild_price = prices.get(&ticker_symbol).copied().unwrap_or(0.0);

    println!("{}: {}", ticker_symbol, gild_price);
}
```
#### Update values

When you update values in a Rust `HashMap` you need to choose what you want to happen. 

You can choose to overwrite the value, insert a standard value or do nothing if there's something there already, or modify the value present in some way.

##### Overwrite

Overwriting values is simple. The hash map simply takes the last value given, in the "overwriting" case.

```rust
fn main() {
    prices.insert("GILD", 66.77);
}
```
##### Upsert

A common pattern is to insert a default value _only when_ a value is not present, otherwise leaving the current value alone.

For our ticker case, imagine that there's a vector of enums that hold a string timestamp and a `HashMap` of our tickers plus the price. This might be a nice way to organize data from various exchanges available.

In this case if we want, we can set a default value for the price, e.g. 0.

> Note setting default 0s would be terrible practice in actual financial engineering applications! 

```rust
fn main() {
    prices.entry("AAPL").or_insert(0.0);
}
```
##### Update

Now let's add a penny to a price.

```rust
fn main() {
    let appl_price = prices.entry("AAPL").or_insert(0.0);
    *aapl_price += 0.01;
}
```
### Collections Projects

The Rust book provides three suggested projects since collections have been reviewed, as these tools allow developers to make much more complex programs.

#### Median, mode, mean
- Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

- We'll also include the mean here, both arithmetic and geometric.

#### Pig latin strings
- Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

#### Stocks to a portfolio
- Using a hash map and vectors, create a text interface to allow a user to add ticker symbols to a portfolio in a fund. For example, “Add AAPL to Alpha Fund I” or “Add GILD to Global Value Fund II.” Then let the user retrieve a list of all tickers in a portfolio or all tickers in the fund by portfolio name, sorted alphabetically.

> I edited the original project suggestion to be about stocks in portfolios in a fund, rather than employees in a department in a company.
````
