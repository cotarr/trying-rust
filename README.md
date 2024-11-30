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
    // note: no semicolon after 5
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

## 4.1 Ownership

Traditionally programs either allocate memory manually or dynamically manage with garbage collection.

Rust uses new concept called "Ownership"

- Under the hood
  - Stack stores values in order (push), removes in the opposite order (pop)
  - Heap, allocating returns a pointer and fixed size, must be deallocated after use

- In Rust, variable has "ownership" over variables memory until dropped.
- Passing variable to a function as argument drops original variable, since now function has ownership.

### Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

- Scope: A variable is valid from the point at which it’s declared until the end of the current scope.

- Compare ownership to string literal, stored internal to code, literals are immutable

```rs
    let s = "hello";
```

- Consider `let s = String::from("hello");`
- The "::" operator allows us to namespace this particular from function under the String type
- The "from" function converts string literal to type "String"
- Calling `String::from` requests the memory to be allocated.
- Under the hood the String will use a pointer, size, available size.
- The variables's memory is automatically returned once the variable that owns it goes out of scope.
- Memory is freed using internal "drop" function so garbage collection is not needed.

```rs
    // memory allocated
    let s = String::from("hello");
    
    // the s variable as argument to function is dropped from current scope, added to println scope
    // When printls! is done, it goes out of scope, is dropped.
    println!("{s}");
```

### Copy and Move

Simple values with a known, fixed size, are pushed onto the stack.

- Value 5 is bound to x
- Current value of x is 5, the value 5 (from x) is bound to variable y
- Both x an y have value 5 (separate variables)
- Rust compiler reserves space on the stack for x and y (simple fixed size variables)

```rs
    let x = 5;
    let y = x;
```

- Types capable of simple copy (those stored on stack)
  - Integer types (all), such as u32.
  - The Boolean type, bool
  - Floating-point types, such as f64.
  - The character type, char.
  - Tuples, with only simple fixed size type like (i32, i32), but not (i32, String).

Complex data structures of variable size can not be copied like previous using stack. Complex variables need to use allocated memory from heap. This is managed internally by rust.

- Attempting to copy complex data generate compiler error

```rs
    let s1 = String::from("hello");
    // s1 goes out of scope, no longer available to use
    let s2 = s1;
    // Do something with s2, but s1 is out of scope, doing something with s2 generates error
```

Example returning original value in tuple along side result value

- Passing variable to function, variable goes out of scope, drops variable's allocated memory
- Original value returned in a tuple so it can continue to be used in the program after subroutine used it
- (See first example next section for alternate way)

```rs
    let s1 = String::from("hello");
    // passing s1 to function drops s1 ownership
    let (s2, len) = calculate_length(s1);
    // So s2 must be returned as tuple value because it's value is still needed by program
    println!("The length of '{s2}' is {len}.");

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        // a tuple is returned containing both values
        (s, length)
    }
```

## 4.2 References and Borrowing

- A "reference" is like a pointer that can be followed to access the data (owned by other variable) stored at that address.
- A reference is guaranteed to point to a valid value of a particular type for the life of that reference.
- Function signature "&" represents  a reference. The "*" is dereference (discussed later)

```rs
    // &s1 refers to S1, but does not own it.
    let s1 = String::from("hello");
    // s1 will not be dropped when used by function
    let len = calculate_length(&s1);

    // Accepts a reference of type String, returning value of type usize.
    fn calculate_length(s: &String) -> usize {
       s.len() // len() returns the length of a String
    }
```

- Creating a reference is called "borrowing"
- References are immutable by default, and changing is compiler error.

### Mutable references

- A mutable variable may have one (and only one) mutable reference
- Mutable reference defined by "&mut" in front of parameter variable
- In `(&mut s)` no type is listed, using type of parent
- The mutable borrow exists until it is used in a function

- More than one reference will be an error.
- Simultaneous mutable and immutable reference is error
- Borrow mutable reference prevents race conditions
- References prevent dangling references

```rs
    let mut s = String::from("hello");
    change(&mut s);

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
```

(see 4.2 example)

### Rules

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

## 4.3 Slice Type

- Slices let you reference a contiguous sequence of elements in a collection (subset of items).
- Slices do not have ownership
- It is a type of reference

### String slices

Rust’s .. range syntax to return slices

```rs
let s = String::from("hello");
let len = s.len();

// Omit starting value assumes first character
let slice = &s[0..2];
let slice = &s[..2];

// Omit ending value assumes last character
let slice = &s[3..len];
let slice = &s[3..];

// These are equal to original string
let slice = &s[0..len];
let slice = &s[..];
```

- String slice from range syntax must occur at valid UTF-8 character boundaries
- The compiler will ensure the references into the String remain valid
- If the original string s becomes invalid, such as `s.clear()`, then the slice from it's reference is also invalid, an error

### String slices as parameters

- "&String" is reference to a type String
- "&str" is a reference to a "string slice"
- The syntax "&s[0..2];" returns type string slice (&str)
- Rust considers string literals to be a slice
- Both String and string literal can be function parameters using "&str" as parameter type.

```rs
fn first_word(s: &String) -> &str {
// this accepts both
fn first_word(s: &str)    -> &str {
```

### Array slice example

```rs
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

## 5.1 Structs

- Keyword "struct" defines a struct
- Struct's name should be descriptive
- Names of types inside struct called "fields"

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

- Create an instance of the struct containing key: value pairs
- Use dot notation to access a value
- Use field "init shorthand syntax" to refer to variable of same name as key (see 5.1 code)
- Using = like an assignment, value moved to user2, it's a "move" not "copy", can no longer use user1 after user3

```rs
    // user 1 instance already defined (see 5.1 code section better example)
    user2 = User {
        active: true,
        user2.username, // dot notation
        email,          // init shorthand syntax
        sign_in_count: 1,
    }
```

- Can use values from one instance to create a new instance
- Use "struct update syntax" to copy boilerplate key:value that are not changed
- The `..user2` must come last
- Using = like an assignment, value moved to user3, it's a "move" not a "copy", can no longer use user2 after user3
- Could have used scaler (on stack) mixed with creating new strings, then user2 would still be valid.

```rs
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2 // struct update syntax
    };
```

### tuple structs

- Define with "struct" keyword followed by tuple containing types
- Otherwise similar to structs

```rs
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
```

### unit-like structs

- Structs without any fields are "unit-like structs"
- Useful to represent a trait (more in later chapters)

```rs
struct AlwaysEqual;
fn main() {
    let subject = AlwaysEqual;
}
```

### struct ownership

- Examples used &String so each instance own's all it's values
- Structs can store references that are not owned, but this requires "lifetime" covered later in book

## 5.2 Example program (struct)

Multi-variations of example program to calculate area of rectangle (see 5.2 code)

- Example using integer variables as parameters
- Example using tuple as parameters
- Example using struct as parameters
- Example pretty print using println!() macro and debug
- Example pretty print using deg!() macro
- Note: Calling dbg! sends to stderr, println! to stdout.

## 5.3 Method syntax

- "Methods" are similar to functions
- Methods are defined in an "impl" implementation block, must be same name as related struct
- Declare using "fn" keyword and a name
- Can have parameters
- Parameter &self (short for "self: &Self), where Self is alias for type that is implementation block (parent)
- Can have return values, type specified by arrow syntax "->" like function
- Method called using "method syntax" which is dot notation on instance of it's parent struct
- Can have multiple methods in an implementation block
- Can have multiple implementation blocks (same as multiple in one block)
- Uses automatic referencing `p1.distance(&p2);` same as `(&p1).distance(&p2);` so, "p1." is an automatic reference

```rs
    struct Rectangle {
        width: u32,
        height: u32,
    }
    // Declare the method
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // Do somethings with the method rect1.area()
    let theArea = rect1.area();
```

### Associated functions

- Defined in implementation block like method
- As opposed to method, the function's parameter not one of parent struct key: value pairs
- Function is namespaced on the struct using "::" double colin notation
- This is like previous example: `String::from("hello");

```rs
    // Associated function square takes integer value as side of a square returns a Rectangle object
    fn square(size: u32) -> Self {
        // Self is type Rectangle
        // Return a type Rectangle
        Self {
            width: size,
            height: size,
        }
    }
    let side = 10;
    let rect1 = Rectangle::square(side);
    // Do somethings with the method rect1.area()
    let theArea = rect1.area();
```

Compare, "." and "::"

```rs
    let theArea = rect1.area();          // Struct method
    let rect1 = Rectangle::square(side); // Struct associated function
```
