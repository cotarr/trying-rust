fn main() {

    // Define a struct
    //
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Create an instance of the struct User
    //
    let user1 = User {
        // order does not matter
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Make entire instance mutable, mutable internal fields not allowed on immutable struct
    let mut _user2 = User {
        // order does not matter
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // change it using dot notation
    _user2.email = String::from("anotheremail@example.com");

    // Use a function
    //
    // takes an email and username and returns a User instance
    fn _build_user1(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    // // with "field init shorthand" syntax (key names are same as parameters)
    fn _build_user2(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // Create instance from other instance
    // The uses = like an assignment
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };


    // Create instance from other instance
    // Using "struct update syntax" with = like an assignment
    let _user4 = User {
        email: String::from("another@example.com"),
        ..user3
    };

}
