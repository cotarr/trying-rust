fn main() {
    // Sample program to return first word of a string
    // To compare, this is a manual example, then use slice later
    //
    // Function to return integer that is a byte index of first char after the first word
    // Accepts String reference as parameter, returning a type usize integer
    fn first_word(s: &String) -> usize {
        // "as_bytes" returns an array
        let bytes = s.as_bytes();

        // Can we print bytes array to see what it is? Nope! Error: mismatched types
        // println("{bytes}");
    
        // "iter" returns each array element in a collection
        // "enumerate" returns the result as a tuple
        // The pattern "(i &item)" dereference the tuple
        for (i, &item) in bytes.iter().enumerate() {
            // "b' '" is byte literal
            if item == b' ' {
                return i;
            }
        }
    
        s.len() // return a value otype uint from len method
    }

    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5

    // Because word a separate value from the String s, s may not be valid in the future. 
    // If we clear the string, the value 5 would still remain, even if string destroyed
    s.clear();

    // prints "5", clearing original string did not generate a compiler error
    println!("With string s '{s}' first non-word char index at {word}");

    // Example to references to a portion of a string
    let p = String::from("hello world");
    println!("Using range syntax");
    let hello = &p[0..5];
    let world = &p[6..11];
    println!("var hello is {hello} and var world is {world}"); // prints hello world


    // Update function to return first word using range syntax
    //
    fn first_word2(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    // Ignore warning, does not need to be mutable, used with s2.clear()
    let mut s2 = String::from("hello world");
    let word2 = first_word2(&s2);

    // Because word2 a separate value from the String s2, s2 may not be valid in the future. 
    // If s2 become invalid, the slice made from it's reference also becomes invalid.
    // Uncomment the clear to show not slice becomes invalid if original string changes
    // the clear method now causes error,  Error: cannot borrow `s2` as mutable because it is also borrowed as immutable
    //
    // s2.clear();

    println!("With string s2 '{s2}' first word at {word2}");

    // update function to use string literals
    //
    // fn first_word3(s: &String) -> &str {
    fn first_word3(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    println!("word is {word} function using slice");

    // // `first_word` also works on references to `String`s, which are equivalent
    // // to whole slices of `String`s
    let word = first_word3(&my_string);
    println!("word is {word} function using reference");

    // Now try using string literal
    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);
    println!("word is {word} function using string literal");

    // // Because string literals *are* string slices already,
    // // this works too, without the slice syntax!
    let word = first_word3(my_string_literal);
    println!("word is {word} function using string literal without range slice syntax");

    // example of slice from array
    let array = [1, 2, 3, 4, 5];
    let array_slice = &array[1..3];
    for &item in array_slice.iter() {
        // Prints: 
        //    array_slice item: 2
        //    array_slice item: 3
        println!("array_slice item: {item}");
    }
}
