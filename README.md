# trying-rust

With a couple friends on IRC, we decided to try some coding in rust.
We are using the online book "The Rust Programming language" available
at https://doc.rust-lang.org/book/title-page.html. I ordered the paperback book.

My first notes are a little detailed, because I am trying to familiarize
myself with the vocabulary used to describe the build process and rust language.

## Setup

- Setup VM with Debian 12
- Installed rust using `rustup` install script.

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

- Installed vscode extension "rust-analyzer"
- Created an empty git repository (this repo) and cloned it.
- Created .gitignore with "\*target\*" to avoid commit binary executable files`
- Begin at Chapter 1 section 1.1

## 1.1 Hello World

- Created folder 1.1-hello-world and copy main.rs from book
- The `fn main() {` defines main function as entry point to program
- Rust style is indent 4 spaces (not tab)
- With `println!` the exclamation point calls a macro (without ! would be function)
- Most rust expressions end in semi-colon `;`
- Compile stand alone binary executable with `rustc main.rs`
- Run with `./main` to produces: "Hello, World!"
- Added "main" (binary) to .gitignore, since not in a /target/ folder

## 1.2 Cargo

- Cargo is Rust’s build system and package manager
- New project with `cargo new hello_cargo`
- Rename folder "1.2-hello_cargo" to sort folders in order
- Cargo automatically created Cargo.toml and src folder with hello world template.
- Unless parent folder is a git repository, cargo will initialize a new Git repository
- Get help with `cargo new --help`
- Cargo.toml (Tom’s Obvious, Minimal Language) is cargo configuration
- Cargo has generated a “Hello, world!” program for you,
- Build dev version with `cargo build`
- Binary at `./target/debug/hello_cargo` will produce "Hello, World!"
- Build and run with `cargo run`
- Check syntax with `cargo check`
- Help with `cargo build --help`
- View build man pages with `cargo help build`
- Build release with `cargo build --release`

## 2.0 Guessing Game

- New project `cargo new guessing_game`, renamed folder 2.0-guessing_game.
- Copy Listing 2-1 to main.rs
- Use io (input/output) library from standard library `use std::io;`
- Curly braces for code block `fn main() {`
- The `fn` keyword declares a new function without any parameters inside `( )` parenthesis

### variable

```rs
let mut guess = String::new();
```

- In Rust, variables are immutable by default `let apples = 5; // immutable (constant)`
- New mutable variable `let mut guess =` with equals symbol to bind it to something
- `String::new()` is function that returns new instance of string, String type, growable, UTF-8.
- The `::` indicates that new is an associated function of the String type
- This has created a mutable variable that is currently bound to a new empty instance of a String.

### user input

- Call the stdin function from the io module

```rs
    io::stdin()
        .read_line(&mut guess)
```

- If we hadn’t imported the io library, could still use  `std::io::stdin.`
- The `stdin` function returns an instance of std::io::Stdin, a type that represents a handle to the standard input
- Next `.read_line(&mut guess)` calls `read_line` method passing `&mut guess` as the argument to read_line
- The read_line function will append input to end of a string (without overwriting its contents)
- The & indicates that this argument is a reference. References are immutable by default
- Added `&mut guess` rather than `&guess` to make it mutable

```rs
    .expect("Failed to read line");
```

- read_line returns a Result value. Result is an enumeration (enum), type that can be in one of multiple possible states (variants)
- read_line Result's variants are an Ok value or Error, in this case Ok value is number of characters
- To crash this program when an I/O error occurs, can use `expect`

```rs
    println!("You guessed: {}", guess);
```

- The `{}` is a placeholder to insert value.
- When printing the value of a variable, the variable name can go inside the curly brackets.
- When printing the result of evaluating an expression, place empty curly brackets in the format string
- End with comma-separated list of expressions to print in each empty curly bracket placeholder in the same order

```rs
// Example showing both variable and expression
let x = 5;
let y = 10;
println!("x = {x} and y + 2 = {}", y + 2);
```

- Testing: As expected, requests input, prints value entered so far.

### Add dependency from Rust Crate Repository

- To get random number generator, need a library
- A "crate" is a collection of Rust source code files
- In Cargo.toml file, add "rand" crate as a dependency
- Value "0.8.5" is actually shorthand for "^0.8.5", which means any version that is at least 0.8.5 but below 0.9.0.

```toml
[dependencies]
rand = "0.8.5"
```

- Before more code changes, run `cargo build` to see package manager load the dependencies.
- Rust Crate Repository is at https://crates.io
- On first build, file Cargo.lock specifies fixed dependency versions (lock file)
- To upgrade new dependencies run `cargo update`

- Update main.rs using Listing 2-3.
- The line `use rand::Rng;` and the `Rng` trait defines methods that random number generators implement

### Add random number generator

```rs
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
```

- the `rand::thread_rng` function gives us the particular random number generator we’re going to use
- The `gen_range` method takes a range expression as an argument and generates a random number in the range.
- Specify 1..=100 to request a number between 1 and 100 (=100 to be inclusive)
- The printLn! line used for debugging to see if random number generator is working

### Dependency help

- run `cargo doc --open` to download and build documentation for dependencies and open in browser
- Dependency docs go into: /target/doc/guessing_game/index.html

### Comparing the guess

- Update main.rs to include listing 2-4
- The line `use std::cmp::Ordering` adds Ordering to scope from the standard library
- Ordering type is another enum and has the variants Less, Greater, and Equal.
- A match expression is made up of "arms". An arm consists of a "pattern" to match against.
- Arm specifies code that should be run if the "value" given to match fits that arm’s "pattern"

### Mismatch Type Errors

- The code at this point is trying to match a string to a number (intentional error for learning)
- Run `cargo build` will produce error: error[E0308]: mismatched types --> src/main.rs:22:21
- Identified: expected reference `&String` found reference `&{integer}
- Rust defaults to an integer tpe: "i32" which is the type of secret_number
- To fix, add this line as follows:

```rs
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

- Rust allows to "shadow" the previous value of guess with a new one.
- Bind this new variable to the expression guess.trim().parse()
- The trim method will eliminate any whitespace including \n or \n\r
- The parse method on strings converts a string to another type
- The colon (:) after guess tells Rust we’ll annotate the variable’s type
- The expect method is added for Err Result that is not a valid number, crash with message

    ### Wrap in a loop

- Wrap code in a loop block to repeat user entry to allow multiple successive guesses

```rs
    loop {

    }
```

- Update the equal condition to exit the loop

```rs
    Ordering::Equal => {
        println!("You win!");
        break;
    }
```

- To avoid the crash, trap the error
- Replace `expect` with a `match` expression
- The underscore `_` character is a catchall to match all Err values
- The `continue` statement tells the program to go to the next iteration of the loop

```rs
    // Previously
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
```

## Chapter 3.1 Variables and Mutability

- By default all variables are mutable

```rs
   let x = 5;
```

- Add "mut" keyword to make variable mutable

```rs
    let mut w = 5;
```

- Constants are values that are bound to a name and are not allowed to change
- Type of the value must be annotated
- Constants can be declared in any scope, including the global scope
- Convention: s upper case with underscores

```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

- "Shadowing" is re-declaring a previously declared variable
- Each "let" declares a new variable with a new type
- Changing value of mutable variable constrains type else, compiler error
- Shadowing (re-declare) mutable variable (re-declare) no compiler error

