fn main() {

    // Compare to previous example
    //
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
    
    fn calculate_length(s: &String) -> usize {
        s.len() // len() returns the length of a String
    }

    // Same with 2 strings, 2 references that are different
    //
    let s1 = String::from("hello1");
    let s2 = String::from("hello2");
    let len: usize = duplicate_ref_calculate_length(&s1, &s2);
    println!("The length of '{s1}' + '{s2}' is {len} with different references");
    fn duplicate_ref_calculate_length(s1: &String, s2: &String) -> usize {
        s1.len() + s2.len()
    }

    // Same with 1 strings, 2 references
    // This works, why, I assume because they are immutable.
    // 
    let s1 = String::from("hello1");
    let len: usize = double_ref_calculate_length(&s1, &s1);
    println!("The length of '{s1}' + '{s1}' is {len} with double references");
    fn double_ref_calculate_length(s1: &String, s2: &String) -> usize {
        s1.len() + s2.len()
    }

    // Same with 1 strings, 2 different functions
    //
    let s1 = String::from("hello1");
    let len1: usize = separate_function1_calculate_length(&s1);
    let len2: usize = separate_function2_calculate_length(&s1);
    println!("The length of '{s1}' + '{s2}' is {} with separate functions", len1 + len2);
    fn separate_function1_calculate_length(s1: &String) -> usize {
        s1.len()
    }
    fn separate_function2_calculate_length(s1: &String) -> usize {
        s1.len()
        // Trying stuff (semicolon, then no semicolon)
        // let ss = s1.len();
        // ss
    }

    // Error caused by trying to change a reference, by default references are immutable
    // Error: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    //
    // let s = String::from("hello");
    // change(&s);
    // fn change(some_string: &String) {
    //     some_string.push_str(", world");
    // }

    let mut s = String::from("hello");

    change(&mut s);
    println!("Printing mutable reference after change: {s}");
    fn change(some_string: &mut String) {
        some_string.push_str(" World");
    }

    // This is ok because all are immutable
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // // This is ok, because & references are immutable by default
    // // However, has warning: variable does not need to be mutable
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{}, {}", r1, r2);


    // // This is hard compiler error
    // // Error: cannot borrow `s` as mutable more than once at a time
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // Example of using mutable reference, serially, but scope ends
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 {r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("r2 {r2}");

    // Example error attempting to combine mutable and immutable references
    // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    //
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // Example, immutable, goes out of scope, so mutable variable is allowed
    //
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2} r1 and r2 immutable");
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3} r3 mutable");


    // Example dangling reference (reference not in scope after returned)
    // Error: missing lifetime specifier
    // let reference_to_nothing = dangle();
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     // reference is only valid inside function, 
    //     &s
    // }

    // This is not error, because s is not a reference
    //
    let s = no_dangle();
    println!("{s} no dangle example");
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }

}
