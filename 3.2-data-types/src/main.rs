use std::io;

fn main() {

    // Note, starting variable with underscore to avoid unused variable compiler error

    // Example: parse requires a type, else compiler error
    let _guess: u32 = "42".parse().expect("Not a number!");
    // Error: type annotations needed: type must be known at this point
    // let _guess = "42".parse().expect("Not a number!");

    let _g: i32 = 98_222;      // Decimal, the underscore is ignored, visual separator
    let _g: i32 = 98222;       // Same
    let _g: i32 = 0xff;        // Hexadecimal
    let _g: i32 = 0o77;        // Octal
    let _g: i32 = 0b1111_0000; // Binary, the underscore is ignored, visual separator
    let _g: i32 = 0b11110000;  // Same
    let _b: u8  = b'A';        // Byte, only u8 type allowed
    println!("value of _g {_g} and _b {_b}");

    let mut c: u8 = 0xE0;
    // This will be ok, 0xE1 is in range
    c += 1;
    // Runtime panic: thread 'main' panicked at src/main.rs:20:5: attempt to add with overflow
    // Uncomment to panic
    // c += 0x20;
    println!("Should panic here if c overflows, c: {c}");

    // Floating Point
    //
    let d = 2.1; // f64
    let e: f32 = 3.123; // f32
    let f: f64 = 1.0 / 3.0;  // 0.3333....
    println!("Floating numbers d {d} and e {e}");
    println!("Repeating decimal f {f}");

    // Floating point literal must have decimal point
    // Compiler error
    // Error: mismatched types: expected `f64`, found integer
    // let _f: f64 = 8;

    // Numeric operations
    //
    let _sum = 5 + 10;             // addition
    let _difference = 95.5 - 4.3;  // subtraction
    let _product = 4 * 30;         // multiplication
    let _quotient = 56.7 / 32.2;   // division
    let _truncated = -5 / 3;       // Results in -1
    let _remainder = 43 % 5;       // remainder

    // Boolean Type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Char type
    //
    let c1 = 'z';
    let c2: char = 'ℤ'; // with explicit type annotation
    let c3 = '😻';
    println!("Chars: {c1} {c2} {c3}");

    // Tuples
    //
    // Tuple  with types declared
    let _tup1: (i32, f64, u8) = (500, 6.4, 1);
    // Error: cannot be formatted with the default formatter
    // println!("tup1 {tup1}");
    

    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    // Prints: 500 6.4 1
    println!("The value of x y z  are: {x} {y} {z}");
    // using dot syntax
    let m = tup2.0;
    let n = tup2.1;
    let o = tup2.2;
    // Prints: 500 6.4 1
    println!("The value of m n o  are: {m} {n} {o}");

    // But printing dot syntax is error
    // Invalid format string: tuple index access isn't supported: not supported in format string
    // println!("Error printing tup.0 {tup2.0}");

    // Arrays
    //
    let _a = [1, 2, 3, 4, 5];
    // type and length declared
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let k = a[2]; // 3
    let l = months[4]; // may
    // Prints: a[2] 3 months[4] May
    println!("a[2] {k} months[4] {l}");

    // create array, each elements = 3 (same), length = 5
    // let g = [<value>, <length>];
    let g = [3; 5];
    let a = g[0];
    let b = g[1];
    println!("Initialized same values: {a} {b}");

    // Compile error, due to analysis at compile time
    let array1 = [1, 2, 3, 4, 5,];
    let mut index1 = 1;
    index1 += 2;
    let a = array1[index1];
    println!("out of range experiment value {a} at index1 {index1}");

    // request array index from keyboard to generate runtime panic
    // 
    // let array2 = [1, 2, 3, 4, 5,];
    // println!("Please enter an array index.");
    // let mut index2 = String::new();
    // io::stdin()
    //     .read_line(&mut index2)
    //     .expect("Failed to read line");
    // let index2: usize = index2
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    // let element = array2[index2];
    // println!("out of range experiment, element {element} at index2 {index2}");
}   
