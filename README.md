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
