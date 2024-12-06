#[allow(unused_variables)]
fn main() {
    // Create an empty vector of type i32
    let v: Vec<i32> = Vec::new();

    // Use vec! macro to create vector where type is determined from intial values
    let v = vec![1, 2, 3];
    println!("v: {v:?}");
    // prints: v: [1, 2, 3]

    let mut v = Vec::new();

    // type is inferred from data
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v: {v:?}");
    // prints: v: [5, 6, 7, 8]

    // Access vector values using array type "indexing syntax" brackets
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Access vector values using "get" method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Using indexing syntax will throw error if index out of range
    // Error: thread 'main' panicked at src/main.rs:33:28: index out of bounds: the len is 5 but the index is 100
    // let does_not_exist = &v[100];

    // using get method will not have a value, returning None
    let does_not_exist = v.get(100);    
    println!("Vector index out of range example {does_not_exist:?}");
    // Prints: None

    // Access vector values using "get" method
    let third: Option<&i32> = v.get(100);
    match third {
        Some(third) => println!("The 100th element is {third}"),
        None => println!("There is no 100th  element."),
    }

    // Example iterating elements of an immutable vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Example iterating and modification of mutable vector
    let mut v = vec![100, 32, 57];
    println!("Before Modification {v:?}");
    for i in &mut v {
        // Note * is de-reference operator, covered in later chapters
        *i += 50;
    }
    println!("Modified mutable vector {v:?}");

    // Example use enum type to wrap different type of values in a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // Example, all elements are type enum, so same type 
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {row:?}");
    // Prints: row: [Int(3), Text("blue"), Float(10.12)]
}
