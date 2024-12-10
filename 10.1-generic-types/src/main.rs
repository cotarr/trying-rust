#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    //
    // Example two similar patterns demonstrating duplicated code
    //
    // Emulate main() function for tutorial purposes
    fn my_main_0() {
        // First of two, find largest number
        let number_list = vec![34, 50, 25, 100, 65];
        // Using a reference
        let mut largest1 = &number_list[0];
        for number in &number_list {
            // Question, is number an integer value or a reference? I think it's an integer value.
            if number > largest1 {
                largest1 = number;
            }
        }
        // prints: The largest number is 100 (First)
        println!("The largest number is {largest1} (First)"); 

        // Second of two, find largest number
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let mut largest2 = &number_list[0];
        for number in &number_list {
            if number > largest2 {
                largest2 = number;
            }
        }
        //Prints: The largest number is 6000 (Second)
        println!("The largest number is {largest2} (Second)");
    }
    my_main_0();
    
    //
    // Example combining common code WITHOUT generic types
    //
    fn my_main_1() {
        // Common code from first and second reduced to function
        // Function accepts i32 and returns i32 types
        fn largest_a(list: &[i32]) -> &i32 {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_a(&number_list);
        println!("The largest number is {result} (with function First)");
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let result = largest_a(&number_list);
        println!("The largest number is {result} (with function Second)");
        // Prints
        //    The largest number is 100 (with function First)
        //    The largest number is 6000 (with function Second)
    }
    my_main_1();

    //
    // Example executing first and second largest of two different types
    //
    fn my_main_2() {
        // Example using different types (i32)
        fn largest_i32(list: &[i32]) -> &i32 {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
    
        // Example using different type (char)
        fn largest_char(list: &[char]) -> &char {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
    
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_i32(&number_list);
        println!("The largest number is {result}");
        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_char(&char_list);
        println!("The largest char is {result}");
    }
    my_main_2();

    //
    // Example using generic type parameter, returning a generic type
    // Intentional Error from tutorial
    //
    // fn my_main_3() {
    //     fn largest<T>(list: &[T]) -> &T {
    //         let mut largest = &list[0];
    //         for item in list {
    //             if item > largest {
    //                 largest = item;
    //             }
    //         }
    //         largest
    //     }
        
    //     let number_list = vec![34, 50, 25, 100, 65];
    //     let result = largest(&number_list);
    //     println!("The largest number is {result}");
    
    //     let char_list = vec!['y', 'm', 'a', 'q'];
    //     let result = largest(&char_list);
    //     println!("The largest char is {result}");
    // }
    // // Error: binary operation `>` cannot be applied to type `&T`
    // my_main_3();

    //    
    // Example error inside (mixing types in same parameter name)
    //
    fn my_main_4() {
        #[derive(Debug)]
        struct Point1<T> {
            x: T,
            y: T,
        }
        let integer = Point1 { x: 5, y: 10 };
        let float = Point1 { x: 1.0, y: 4.0 };
        // Example error:  mismatched types, 
        // let wont_work = Point1 { x: 5, y: 4.0 };
        //
        // prints: Point1 { x: 5, y: 10 } Point1 { x: 1.0, y: 4.0 }
        println!("{integer:?} {float:?}");
    }
    my_main_4();

    //    
    // Example using different types in struct with different names T and U
    //
    fn my_main_5() {
        #[derive(Debug)]
        struct Point2<T, U> {
            x: T,
            y: U,
        }

        let both_integer = Point2 { x: 5, y: 10 };
        let both_float = Point2 { x: 1.0, y: 4.0 };
        let integer_and_float = Point2 { x: 5, y: 4.0 };
        // Prints: Point2 { x: 5, y: 10 } Point2 { x: 1.0, y: 4.0 } Point2 { x: 5, y: 4.0 }
        println!("{both_integer:?} {both_float:?} {integer_and_float:?}")
    }
    my_main_5();    

    //
    // Example with struct method
    //
    fn my_main_6() {
        struct Point3<T> {
            x: T,
            y: T,
        }
        
        // method named x on the Point<T> struct that will return a reference to the x field of type T
        impl<T> Point3<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        let p = Point3 { x: 5, y: 10 };
        // Prints: p.x = 5
        println!("Method of generic type p.x = {}", p.x());
    }
    my_main_6();
    
   
    fn my_main_7() {
        struct Point4<T> {
            x: T,
            y: T,
        }
        impl Point4<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        // These need to be f32, x: 5, y:10 is error
        // Error: no method named `distance_from_origin` found for struct `Point4<{integer}>` in the current scope
        // let p = Point4 { x: 5, y: 10 };
           let p = Point4 { x: 5.0, y: 10.0 };
        let d: f32 = p.distance_from_origin();
        println!("Distance from origin = {d}");
    }
    my_main_7();

    // Example from book causes multiple compile errors
    // fn my_main_8() {
    //     struct Point5<T> {
    //         x: T,
    //         y: T,
    //     }
    //     impl<X1, Y1> Point5<X1, Y1> {
    //         fn mixup<X2, Y2>(self, other: Point5<X2, Y2>) -> Point5<X1, Y2> {
    //             Point5 {
    //                 x: self.x,
    //                 y: other.y,
    //             }
    //         }
    //     }
    //     let p1 = Point5 { x: 5, y: 10.4 };
    //     let p2 = Point5 { x: "Hello", y: 'c' };
    
    //     let p3 = p1.mixup(p2);
    
    //     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // }
    // my_main_8();


}
