// Compiler directives where added to avoid warnings when trying tutorial declarations to check syntax
#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {
    // // Define enumeration with enum keyword with two "variants"
    // //
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }
    // // Some example instances, namespaced with double colon "::
    // //
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // // Define a struct using the enum as a field, and create two instances of it.
    // //
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
        
    // // Add String value to each enum variant of defined type
    // // automatically generates constructor function
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));
            
    // each variant can have different types and amounts of associated data. 
    //
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // will print: home: V4(127, 0, 0, 1)
    println!("home: {:?}", &home);

    // will print: loopback: V6("::1")
    println!("loopback: {:?}", &loopback);

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // Add implementation block
    //
    impl Message {
        fn call(&self) {
            // method body would be defined here
            println!("enum method called with: {:?}", &self);
        }
    }

    // will print: enum method called with: Write("hello")
    let m = Message::Write(String::from("hello"));
    m.call();

    // will print: Instance of 'm' contains: Write("hello")
    println!("Instance of 'm' contains: {:?}", &m);

    // The standard library includes predefined "Option" enum
    // This is to differentiate between has a valid value, and does not have a value.
    // For variable that have value, Sum() can represent any type
    let some_number = Some(5);
    let some_char = Some('e');

    // To use none, the Option enum must be explicitly annotated with a type
    // Format is Option<T> where T represents type
    let absent_number: Option<i32> = None;

    // prints: some_number: Some(5) somechar: Some('e')
    println!("some_number: {some_number:?} somechar: {some_char:?}");
    // prints: absent_number: None
    println!("absent_number: {absent_number:?}");
}
