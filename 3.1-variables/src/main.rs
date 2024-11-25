fn main() {

    // Immutable variable
    //
    let x = 5;
    println!("The value of immutable variable x is: {x}");
    // Compiler error: cannot assign twice to immutable variable
    // x = 6;
    // println!("The value of x is: {x}");

    // Mutable Variable
    // 
    let mut w = 5;
    println!("The value of mutable w is: {w}");
    w = 6;
    println!("The value of w was changed to: {w}");

    // Constant
    //
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    //
    let v = 5;
    let v = v + 1;
    {
        let v = v * 2;
        println!("The value of x in the inner scope is: {v}");
    }
    println!("The value of x is: {v}");

    let spaces1 = "   ";
    let spaces1 = spaces1.len();
    println!("Variable spaces1 is shadowed with different type: {spaces1}");

    // Using mutable variable constrains type to remain same when reassigned
    let spaces2 = "   ";
    // Error: mismatched types: expected `&str`, found `usize`
    // spaces2 = spaces2.len();
    // println!("Variable spaces2 is shadowed with different type: {spaces2}");

}