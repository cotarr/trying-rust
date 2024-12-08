use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;
use std::error::Error;

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    // Example, generate file not found error
    //
    // let greeting_file_result = File::open("hello.txt");

    // // Example, trap the error using match expression
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     // Result enum and its variants have been brought into scope,
    //     // so `Result::` not needed before the Ok and Err variants
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    // First, try opening the file for read access
    let greeting_file_result = File::open("hello2.txt");
    // Check Result enum
    let greeting_file = match greeting_file_result {
        // If no error, return file descriptor
        Ok(file) => file,
        // Else error, see if the error is file NotFound
        Err(error) => match error.kind() {
            // Case of NotFound, create the file, returning a second Result enum
            ErrorKind::NotFound => match File::create("hello2.txt") {
                // New created file, success, return file descriptor
                Ok(fc) => fc,
                // Else, panic into a crash with message
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                // In case of other error, panic to crash with message
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // Alternative using .unwrap_or_else() (unwrap_or_else chapter 13)
    //
    let greeting_file = File::open("hello3.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello3.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // Example of unwrap method on Result Ok variant
    // In case of error, .unwrap will automatically panic, else return result
    //
    // Error: called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    // let greeting_file = File::open("hello.txt").unwrap();

    // Example of return a Result enum from a function
    //
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
            let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            // If success, calling function will receive an Ok value that holds a String
            Ok(_) => Ok(username),
            // If problems, return Err value holding an instance of io::Error with error description.
            Err(e) => Err(e),
        }
    }
    let user_data = read_username_from_file();
    // Prints: Result: Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
    println!("Result: {user_data:?}");


    // Example using ? operator after a Result
    //
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    let user_data = read_username_from_file2();
    println!("Result: {user_data:?}");

    // Example error, wrong return type
    //
    // Error: cannot use the `?` operator in a function that returns `()`
    // let greeting_file = File::open("hello.txt")?;

    // C hanged return type to `Result<(), Box<dyn Error>>`
    // Box<dyn Error> type is a trait object, wait until Chapter 17
    // Generally means: “any kind of error.” Using ? on a Result value 
    //
    // The code will now compile`
    fn main2() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;
        Ok(())
    }
    let main2_result = main2();
    // prints: main2_result: Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
    println!("main2_result: {main2_result:?}");
}
