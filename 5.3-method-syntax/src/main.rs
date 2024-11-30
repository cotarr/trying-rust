fn main() {
    #[derive(Debug)]
    // Define struct to hold the method
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    // Define the struct method
    // Implementation block for struct Rectangle
    // Several methods added following Chapter 5-3
    //
    impl Rectangle {
        // Parameter self of type Self.
        // Note that "&self" is short for "self: &Self"
        //
        // Example 1 area method
        //
        // This is same...
        // fn area(self: &Self) -> u32 {
        //
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Example 2 multiple methods on struct
        //
        // Define a second method width() on the same struct
        fn width(&self) -> bool {
            self.width > 0
        }

        // Example 3 multiple implementations use method
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        //
        // Example 3 Associated function
        // The function parameter not one of parent struct key: value pairs
        // In this case the side of a square returns a Rectangle object
        //
        fn square(size: u32) -> Self {
            // Self is type Rectangle
            // Return a type Rectangle
            Self {
                width: size,
                height: size,
            }
        }

    }
    
    fn get_area() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
    
        println!(
            "The area of the rectangle is {} square pixels.",
            // method syntax on rect1 implementation of struct Rectangle
            rect1.area()
        );  

        // Automatic referencing replaces rect1.width() with (&rect1).width)
        //
        // This is the same without automatic referencing
        // if (&rect1).width() {
        //
        if rect1.width() {
            println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }
    }
    get_area();

    fn check_can_hold() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
    
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
    check_can_hold();

    fn get_square() {
        let side = 10;
        // Use the :: syntax with the struct name to call "associated function"
        // This is like String::from("...")
        let rect1 = Rectangle::square(side);

        println!(
            "The area of the square side of {side} is {} square pixels.",
            rect1.area()
        ); 
    }
    get_square();
}
