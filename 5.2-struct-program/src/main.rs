// #[derive(Debug)]

fn main() {
    fn get_area1() {
        let width1 = 30;
        let height1 = 50;
        
        println!(
            "The area of the rectangle is {} square pixels.",
            area_with_params(width1, height1)
        );
    }
    fn area_with_params(width: u32, height: u32) -> u32 {
        width * height
    }
    get_area1();

    fn get_area2() {
        let rect1 = (30, 50);
        println!(
            "The area of the rectangle is {} square pixels.",
            area_with_tulple(rect1)
        );
    }
    fn area_with_tulple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    get_area2();

    struct Rectangle1 {
        width: u32,
        height: u32,
    }
    fn get_area3() {
        let rect1 = Rectangle1 {
            width: 30,
            height: 50,
        };
    
        println!(
            "The area of the rectangle is {} square pixels.",
            area_with_structs(&rect1)
        );
    }
    fn area_with_structs(rectangle: &Rectangle1) -> u32 {
        rectangle.width * rectangle.height
    }
    get_area3();

    
    // Added "#[derive(Debug)]" to use debug formatting to print structured data 
    // using ":?" syntax in println! macro
    #[derive(Debug)]
    struct Rectangle2 {
        width: u32,
        height: u32,
    }
    fn print_area1() {
        let rect = Rectangle2 {
            width: 30,
            height: 50,
        };
        // // Error: `Rectangle2` doesn't implement `std::fmt::Display`
        // // Error: Rectangle2` cannot be formatted with the default formatter
        // // Hint in error: ... may be able to use `{:?}` (or {:#?} for pretty-print) instead
        // println!("rect is {}", rect);

        // // Try suggestion (First time)
        // //  Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. 
        // // Error: Debug` is not implemented for `Rectangle2`
        // println!("rect is {rect:?}");

        // try second suggestion from book, add "#[derive[debug]]" in main.rs file, at start of file
        //
        // Use ":?" to print the instantiated struct
        // Prints: "rect is Rectangle2 { width: 30, height: 50 }"
        println!("rect is {rect:?}");
        
        // Add # to pretty print in multi-line ":#?"
        // Prints:
        //   rect is Rectangle2 {
        //       width: 30,
        //       height: 50,
        //   }
        // println!("rect is {rect:#?}");
    }
    print_area1();

    #[derive(Debug)]
    struct Rectangle3 {
        width: u32,
        height: u32,
    }
    fn print_area2() {
        let scale = 2;
        let rect = Rectangle3 {
            width: dbg!(30 * scale),
            height: 50,
        };
        dbg!(&rect);
    }
    print_area2();

} // main

// Output when run:

// The area of the rectangle is 1500 square pixels.
// The area of the rectangle is 1500 square pixels.
// The area of the rectangle is 1500 square pixels.
// rect is Rectangle2 { width: 30, height: 50 }
// [src/main.rs:97:20] 30 * scale = 60
// [src/main.rs:100:9] &rect = Rectangle3 {
//     width: 60,
//     height: 50,
// }

