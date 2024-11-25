fn main() {
    println!("Hello, world!");

    another_function1();
    another_function2(5);
    print_labeled_measurement(5, 'h');
    another_function3();
    another_function4();
    another_function5();
    another_function6();
    another_function7();
}

fn another_function1() {
    println!("Another function.");
}


fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements
//
fn another_function3() {
    let y = 6;
    // Error: unused parenthesis
    // let x = (let y = 6);
    //
    // Error: expected expression, found `let` statement
    // let x = let y = 6;
    println!("y is {y}");  
}

// Expressions
fn another_function4() {
    // arithmetic expression
    let _y = 1 + 2 + 3;

    // new code block {} returns a value
    // returns 4
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y (code block expression) is: {y}");
}

// Function returning a value
//
fn five() -> i32 {
    5
}
fn another_function5() {
    let x = five();
    println!("The value of x (function return value) is: {x}");
}

// Function with parameter returning a value
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn another_function6() {
    let x = plus_one(5);
    println!("The value of x (add + 1) is: {x}");
}

// return type not match declaration
//
// fn wrong_type() -> i32 {
//     1.2345
// }
// fn another_function7() {
//     let x = wrong_type();
//     println!("The value of x (wrong time) is: {x}");
// }

