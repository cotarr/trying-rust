#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_mut)]
fn main() {

    // Mutable empty string
    let mut s = String::new();

    // String literal
    let data = "initial contents";

    // to_string is method to load data
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("s = \"{s}\"");

    let s = String::from("initial contents");
    println!("s = \"{s}\"");

    // push_str method
    //
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s = \"{s}\"");

    // Example showing push_str does not drop variable
    //
    let mut s1 = String::from("foo");
    let s2 = "bar";
    // s2 is not dropped because push_str uses a string slice does not take ownership
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    // Example + operator, 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // compiler can coerce s2 type &String argument into different type &str. 
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //
    // Error: borrow of moved value: `s1`
    // println!("s1 is {s1}");
    //
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");

    // Example format! macro
    //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");

    // Indexing strings
    //
    // Error: the type `str` cannot be indexed by `{integer}`
    // let s1 = String::from("hello");
    // let h = s1[0];

    // Word for 'hello' in multi-bye characters, integer word > 255
    // Error: the type `str` cannot be indexed by `{integer}
    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // Example string slice)
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // Prints: s is Зд
    println!("s is {s}");
    //
    // This panics a crash: byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
    // let hello = "Здравствуйте";
    // let s = &hello[0..1];

    // example iterating a string as characters
    for c in "Зд".chars() {
        println!("{c}");
    }
    // outputs:
    //   3
    //   д
    //

    // example iterating a string as bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // outputs:
    //   208
    //   151
    //   208
    //   180
}
