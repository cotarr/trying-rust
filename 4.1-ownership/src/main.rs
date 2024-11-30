fn main() {
    // String literal, stored internal to code, literals are immutable
    let s = "hello";
    println!("{s}");

    // from function converts string literal to type "String"
    let s = String::from("hello");
    println!("{s}");

    // Mutable string, an extend by appending characters (can not change original)
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`


    // One immutable variable value copied to another
    //
    let x = 5;
    let y = x;
    println!("x {x} y {y} both in scope (stored on stack)");

    // One mutable variable value copied to another
    // Note compiler warning on both lines: variable does not need to be mutable (suggestion)
    let mut x = 5;
    let mut y = x;
    println!("x {x} y {y} both in scope (stored on stack)");


    // //
    let s1 = String::from("hello");
    let s2 = s1;
    // this will print successfully, content moved from s1 to s2
    println!("s2 {s2}");
    // // Error: borrow of moved value s1: value borrowed here after move
    // println!("s1 {s1} s2 {s2}");

    // Create a copy of data in complex structure using clone function
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // In this example, s1 is dropped as it goes out of scope,
    // but the value of s1 is returned by the function for future use.
    let s1 = String::from("hello");
    // passing s1 to function drops s1 ownership
    let (s2, len) = calculate_length(s1);
    // So s2 must be returned as tuple value becuase S still needed by program
    println!("The length of '{s2}' is {len}.");

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }
}
