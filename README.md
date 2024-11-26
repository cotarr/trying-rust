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

## Some conventions

- Constants are upper case with underscores
- Unused variable can start with underscore to avoid unused variable error

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

## Chapter 3.2 - Data Types

- Every value in Rust is of a certain "data type"
- Two data type subsets: "scalar" and "compound"
- Rust is a statically typed language (all types known at compile time)

### Integer types

```text
Length     Signed    Unsigned
8-bit      i8        u8
16-bit     i16       u16
32-bit     i32       u32
64-bit     i64       u64
128-bit    i128      u128
arch       isize    usize (architecture of local machine, i.e. 64 bit)
```

### Integer literals

- Underscore ignored in integer literal, for visual separation

```text
Decimal      98_222          (underscore ignored, for visible separation)
Hex          0xff
Octal        0o77
Binary       0b1111_0000
Byte         b'A'            (u8 only)
```

### Integer overflow

- Compiler includes checks for integer overflow
- Runtime overflow causes "panic"
- Release mode with the --release flag does not include checks

### Floating Point Types

- Rust’s floating-point types are f32 and f64
- All floating-point types are signed
- Floating point literal must have decimal point, else compiler considers as integer
- The f32 type is a single-precision float, and f64 has double precision.

### Boolean type

- Type is "bool"
- Allowed "true" or "false" in lower case syntax

### Character Type

- Type is "char"
- Specify with single quotes
- char type is four bytes (unicode scalar value)
- Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF

### Tuple type

- Type is "tuple"
- Defined using parenthesis, comma separated values of different types
- Assigned from tuple using parenthesis
- Assigned from tuple using dot notation.

```rs
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    // Prints: 6.4
    println!("The value of y is: {y}");
    let m = tup.1;
    // Prints: 6.4
    println!("The value of m is: {m}");
```

### Array types

- Type is "array"
- Every element must have same type
- Length of all arrays are fixed
- Values are comma-separated list inside square brackets
- A "vector" type is similar to array but can grow

```rs
    let a = [1, 2, 3, 4, 5];
    // type and length declared
    let b: [i32; 5] = [1, 2, 3, 4, 5];
```

- Initialize an array with same value for each element
- Specifying the initial value, followed by a semicolon, and then the length

```rs
   let a = [3; 5];
```

- Compiler includes range analysis of literal assignment to index
- Runtime panic if not production, and index exceeded.

## Chapter 3.3 Functions

- Function names use snake case for function and variable names (all letters lowercase with underscores)
- Defined by "fn" keyboard followed by name, parenthesis and curly braces.
- Functions definitions do nto end in simicolon
- Call function using name followed by parenthesis
- Optional parameters
- Must declare the type of each parameter
- Multiple parameters separated by commas

```rs
    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    } // no semicolon at end
```

### Statements and Expressions

- Statements perform an action but do not return a value.
- Unlike other languages, assignment statements do not return a value, error: `let y = let z = 4;`

- Expressions evaluate to a resultant value
- Calling a function, calling a macro are an expressions
- A new scope block created with curly brackets is an expression

- Functions can return values to the code that calls them
- The body (code block) must end in an expression returning a value (the value to be returned)
- Type must be declared using an arrow "->"
- Return type must match declared function return value type

```rs
fn five() -> i32 {
    // this is an expression, returning 5
    5
}
```

## 3.4 Comments

- Comment line start with two slash "// "
- Multi line comments, each line needs new //
- Comments can be placed at the end of a line
- Typically comment is on line preceding the referenced statement

```rs
// This is a multi-line comment
// This is the second line

// Explain statement
let x = 4;

let x = 4; // Explain statement
```

## 3.5 Control Flow

### if statements

- Start with keyword "if" followed by a condition
- Blocks of code associated sometimes called "arms"
- Optional "else" statement
- The condition must be a boolean
- Will not do automatic type conversion to boolean
- Keyword "else if" check for subsequent conditions,  if .. else if ... else

- if is an expression, so it returns a value
- Blocks of code evaluate to last expression in block

```rs
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
```

- all branches of if statement must return same type, following is error

```rs
    let number = if condition { 5 } else { "six" };
```

### loop

- The "loop" keyword repeats code block
- The "break" keyword exits loop
- break expression returned as value `break 12;`
- The "continue" keyword, stops execution and starts a new loop
- Also "return" exits loop and also exits loop and function

### nested loops

- With nested loops, break and continue apply to the innermost loop
- loop label for multiple loops, determine which loop will break or continue

```rs
    // some code omitted
    // loop label
    'counting_up: loop {
        loop {
            if count == 1 {
                break;
            }
            if count == 2 {
                // break specifies label
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
```

### while

- The "while" keyword followed by condition followed by code block
- While the condition is true the loop runs
- When condition ceases to be true, the program calls break

```rs
    let mut number = 3;
    while number != 0 {
        number -= 1;
        // do something
    }
```

### for

- The for loops are the most commonly used loop construct in Rust

```rs
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
```

- The "range" keyword generates all numbers in sequence starting from one number,ending another
- The "rev" keyword reverses the order

```rs
    for number in (1..4).rev() {
        // do something
    }
```

