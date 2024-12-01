// Compiler directives where added to avoid warnings when trying tutorial declarations to check syntax
#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    // Example: Use match to pattern match an enum variant
    //
    fn value_in_cents1(coin: Coin) -> u8 {
        // define match "arms"  
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    println!("A Nickel is {} cents", value_in_cents1(Coin::Nickel));

    // Example: code block to execute statement and then return value
    //
    fn value_in_cents2(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    // prints
    //   Lucky penny!
    //   A Penny is 1 cents, Penny prints message
    println!("A Penny is {} cents, Penny prints message", value_in_cents2(Coin::Penny));

    // Example one enum includes a second enum stored inside it
    //
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents3(coin2: Coin2) -> u8 {
        match coin2 {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
    // prints:
    //   State quarter from Alaska!
    //   A Quarter is 25 cents from Alaska
    println!("A Quarter is {:?} cents from {:?}", value_in_cents3(Coin2::Quarter(UsState::Alaska)), UsState::Alaska);

    // Example matching option<T>
    //
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    // five will have i32 value 5
    let five = Some(5);
    // six will have i32 value 6
    let six = plus_one(five);
    // none will not have a value (type: None)
    let none = plus_one(None);
    // Prints: five: Some(5)
    println!("five: {:?}", &five);
    // Prints: six: Some(6)
    println!("six: {:?}", &six);
    // Prints: none: None
    println!("none: {:?}", &none);

  
    let dice_roll1 = 9;
    match dice_roll1 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    fn add_fancy_hat() {
        println!("add_fancy_hat")
    }
    fn remove_fancy_hat() {
        println!("remove_fancy_hat")
    }
    fn move_player(num_spaces: u8) {
        println!("mode_player {num_spaces} spaces")
    }
    // Prints: mode_player 9 spaces

    let dice_roll2 = 9;
    match dice_roll2 {
        3 => add_fancy_hat2(),
        7 => remove_fancy_hat2(),
        _ => reroll2(),
    }
    fn add_fancy_hat2() {
        println!("add_fancy_hat")
    }
    fn remove_fancy_hat2() {
        println!("remove_fancy_hat")
    }
    fn reroll2() {
        println!("reroll")
    }   
    // Prints:  reroll 

    let dice_roll3 = 9;
    match dice_roll3 {
        3 => add_fancy_hat3(),
        7 => remove_fancy_hat3(),
        // Use empty unit tuple, do nothing if not match 3 or 7
        _ => (),
    }
    fn add_fancy_hat3() {}
    fn remove_fancy_hat3() {}
    // No printout generated


}
