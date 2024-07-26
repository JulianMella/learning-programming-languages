/*
Enum and structs are the building blocks for creating new types in Rust

Chapter 5 covers coupling related data using structs also cover using methods and
associated functions on structs and compare them to tuples

*/

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Implementation blocks houses the functions and methods associated with a struct
impl Rectangle {
    fn area(&self) -> u32 {
        // Can take a mutable reference or in rare cases ownership..?
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Can create many implementation blocks
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        // Associative functions dont have self
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Like a tuple, our struct allows us to group related data together of different types.
// The benefit is that we can name our structure and we also can name the data inside
// Easier to reference data by name than index location.

fn main() {
    let mut user1 = User {
        email: String::from("bogdan@mail.com"),
        username: String::from("bogdan123"),
        active: true,
        sign_in_count: 1,
    };

    let _name = user1.username;
    user1.username = String::from("wallace123"); // Must make entire struct mutable

    let user2 = build_user(String::from("kyle@mail.com"), String::from("kyle123"));

    //Reusing instance data
    let user3 = User {
        email: String::from("james@mail.com"),
        username: String::from("james123"),
        ..user2 // Rest of fields come from user2
    };

    // Same values but differentiated by name so that if a function expects color, it wont let a point be used.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Example usage
    //
    /*
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> {
        width * height
    }

    */
    /* Possible solution, but it is ambiguous what dimensions.0 and dimensions.1 refer to Width or height?
    let rect = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(rect));

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    */

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    //println!("Area {}", area(&rect));
    println!("Area {}", rect.area());

    // Need to add a display trait to print it out..
    //
    println!("rect: {:#?}", rect);

    // Can also group the function to the struct by tying it as a method. ^
    //
    // . notation to call the area method on our rect instance
    // In c++ there is different syntax when you are calling a method on a object or pointer to the object
    // In rust it is the same since it has a function called automatic referencing and deferencing.

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 60,
    };

    println!("rect2 can hold rect1 {}", rect2.can_hold(&rect1));
}

/*fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}*/

fn build_user(email: String, username: String) -> User {
    User {
        email, // Field init shorthand syntax where the initialization parameter is the same as the struct field name
        username,
        sign_in_count: 0,
        active: false,
    }
}
