/*

There are different ways that programming languages manage their memory in

1. Garbage collection

Garbage collection is an automatized management method implemented in programming languages
where the language handles it without any form of modifications done by the user.

The pros are that it is error free and faster write time, but the cons are that
there is no control over memory, slower and unpredictable runtime performance and larger program size

2. Manual memory management (MMM)

MMM is when the coder itself has to allocate memory for the program manually by writing it down.
The pros are that there is better control over memory than Garbage collection, faster runtime and smaller program size, but the cons are
that it is very error prone because the coder can easily write something incorrectly and it takes longer to figure out logic and
essentially has slower write time.

3. Ownership model

Rust is a memory safe language where we don't use manual memory management. This is done by doing a bunch of compile time checks to make sure memory is done in a safe way.

The pros are control over memory, error free (unless unsafe rust is utilized), faster runtime and smaller program size. The cons are slower write time and a learning curve.

---

Rust makes certain decisions on whether something will be stored on the stack or on the heap.

During runtime, the program has access to the stack and the heap.

--- Properties of the stack ---

The stack is fixed in size and cannot grow or shrink during runtime.
It stores stack frames which are created for every funciton that executes.
Stack frames stores the local variables of the function being executed.
The size of the stack frame is calculated at compile time.
Variables only live as long as the stack frame lives (Until it goes out of scope).


--- Properties of heap ---

Can grow or shrink during runtime.
Data stored can be dynamic in size, large amounts of data.
We control the lifetime of the data.

A clear example of something that can be stored in either the stack or the heap is a string and a string literal.

A string literal has a predefined size of how many characters it includes and since it is immutable, it fits right into the stack because it will not grow or shrink.

String on the other hand which can be dynamic in size and cannot be stored directly on the stack and therefore allocates memory for it on the heap.
The pointer to the String variable will be stored in the stack.

Storing on the Stack is faster than the Heap since when something is stored in the Heap, the program takes time on finding a memory area where the variable (or something else) can be stored.

Access time is also slower since you first need to go the pointer in the stack and from there go to the heap.

*/

fn main() {
    // --- Ownership rules ---
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped

    {
        // s is not valid here, it's not yet declared
        let _s: &str = "hello"; // s is valid from this point forward
        let _heap: String = String::from("hello");
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x: i32 = 5;
    let _y: i32 = x; // Copy is permitted on types that have the Copy tag.

    let s1: String = String::from("Hello");
    // s1 in stack looks as follows
    // ----------------
    // | name | value |
    // |------|-------|
    // |  ptr |  ---------------> HEAP i0 = h, i1 = e, i2 = l, i3 = l, i4 = o
    // |------|-------|
    // |  len |    5  |
    // |------|-------|
    // | capac|ity 5  |
    // |------|-------|

    let _s2: String = s1;

    // What is expected to happen here?
    // Some might expect the value to be cloned, where a new table is created as the one above
    // That doesn't happen as it is very expensive to create a copy on the heap
    //
    // Some others think that it is a shallow copy, where s2 has a pointer that points to the same area in the heap as s1
    // To ensure memory safety, Rust invalidates s1. Instead of being a shallow copy, this is a move.
    //
    // println!("{s1}, world!"); This is invalid unless s2 is called with s1.clone();
    //
    //

    let s: String = String::from("hello");
    takes_ownership(s);
    // When we pass s into a function, its taking over its ownership / moved into the function.
    //println!("{s}"); this is illegal because the function takes over the ownership of s and discards it after the function goes out of scope.

    let x: i32 = 5;
    makes_copy(x);
    println!("{x}"); // This is not moved, but copied instead.

    let s1: String = gives_ownership();
    println!("s1 = {s1}");
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2); // Moves ownership into function and returning it back outside of function
    println!("s1 = {s1}, s3 = {s3}");

    // All of this is tedious, what if we want to use a variable without moving it?
    // Utilize references

    let s1: String = String::from("hello");
    let (s2, len) = calculate_length(s1); // Takes in string and returns length of str, but we don't want to take ownership.
                                          // Returns a tuple with string and length. Looks strange and not usual
    println!("The length of {s2} is {len}.");

    // A reference points to the pointer in the stack which points to the heap.
    // We are borrowing

    let s1: String = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of {s1} is {len}");

    // Keep in mind, references are immutable!

    let s1: String = String::from("hello");
    let _s2 = &s1;
    // s2.push_str(", world");  ERROR! unless we do a mutable reference...

    let mut s1_mut = String::from("hello");
    change(&mut s1_mut); // Change function mutates string without taking ownership of the underlying value

    // Mutable references have a big restriction
    // Can only have one mutable reference to one particular piece of data in a particular scope
    //
    let mut s2_mut = String::from("hello");
    let _r1 = &mut s2_mut;
    // let r2 = &mut s2_mut; ERROR!

    // Can also not have an immutable reference if there is a mutable reference
    // unless the immutable reference has gone out of scope. (!) a println is enough (!)
    // Can have multiple immutable references
    //
    // Rules of reference
    //
    // 1. At any given time, you can either have one mutable reference
    // or any number of immutable references
    //
    // 2. References must always be valid. The data the point to must be valid
    //
    // Dangling pointers are not allowed
    //

    //Slices

    // Slices let you reference a contiguous sequence of elements within a collection instead of referencing a whole collection
    // It does not take ownership of the underlying data
    //
    let mut s = String::from("hello world");
    let word = first_word(&s); //Gets index 5 as space is 5.
    s.clear(); //The problem is here is that it makes it an empty string
               // Variable word is still 5 even though the string is empty
               // We need to manually keep the variable word in sync with the string

    // If we want the second word we need to return a tuple, start of word - end of word

    let hello = &s[..5]; // 0 inclusive - 5 exclusive, can also omit 0
    let world = &s[6..]; // 6 inclusive - 11 exclusive
    let whole = &s[..];

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

/*  ILLEGAL! since the function has created a local variable and gets dropped after going out of scope.
fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}
*/
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
    // some_string gets dropped.
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string: String = String::from("Hello");

    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    let length: usize = s.len();
    length
}
