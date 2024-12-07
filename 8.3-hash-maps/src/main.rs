use std::collections::HashMap;

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Prints: {"Yellow": 50, "Blue": 10}
    println!("{scores:?}");    

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} {score:?}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // prints: 
    //   Blue 10
    //   Yellow: 50
    //   Blue: 10

    // Example inserting owned value type String, drops after use, error if referenced
    //
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name value is moved, field_value invalid
    // Error:  borrow of moved value: `field_name`
    // println!("{field_name}");

    // Example as reference (no error)
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("{field_name} {field_value}");

    // Example scaler integer value copied (no error)
    let num1 = 4;
    let num2 = 7;
    let mut map = HashMap::new();
    map.insert(num1, num2);
    // Prints: 4 7
    println!("{num1} {num2}");

    // Example scaler integer value as reference (no error)
    let num1 = 4;
    let num2 = 7;
    let mut map = HashMap::new();
    map.insert(&num1, &num2);
    // Prints: 4 7
    println!("{num1} {num2}");

    // Example overwriting a pervious value with a new one
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // Prints: {"Blue": 25}
    println!("{scores:?}");

    // entry method checks if key exist and has value, add new, else ignores
    //
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // prints {"Blue": 10, "Yellow": 50}
    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // 
    println!("{map:?}");

}

