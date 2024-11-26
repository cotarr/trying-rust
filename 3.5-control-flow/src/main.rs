fn main() {
    // if statement
    //
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // let number = 3;
    // Error: mismatched types: expected `bool`, found integer
    // if number {
    //     println!("number was three");
    // }
    // This would be same, the != expression return a boolean
    if number != 0 {
        println!("number was something other than zero");
    }

    // if, else if, else
    //
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if statement is expression, so returns a value
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Since if returns a value, can be type mismatch
    // Both branch must return same type
    //
    // let condition = true;
    // // `if` and `else` have incompatible types: expected integer, found `&str`
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");

    // Infinite loop
    //
    // loop {
    //     println!("again!");
    // }

    // loop returning a value
    //
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    // Prints 20
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while
    //
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // While to loop array
    //
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // same as previous while loop using for loop
    //
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // Using range and rev to iterate loop
    //
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
