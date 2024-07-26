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

// Like a tuple, our struct allows us to group related data together of different types.
// The benefit is that we can name our structure and we also can name the data inside
// Easier to reference data by name than index location.

fn main() {}
