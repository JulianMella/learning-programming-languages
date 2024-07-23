fn main() {

    // --------  Variables and Mutability

    // if this was let x = 5, it would be immutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 2;
    println!("The value of x is now: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant value: {THREE_HOURS_IN_SECONDS}");
    // Can do simple computations at compile time in constants, but not allowed to based off of runtime computations.
    // Valid for entire program run

    //Shadowing is declaring a new variable with the same name as a previous variable
    //The first variable is shadowed by the second variable, which means that
    //the second variable is what the compiler will see when you use the name of the variable
    //The variable is shadowed until the scope ends or it itself is shadowed.

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");

    //Shadowing is different from mut because we will get a compile time error if we try to assign a new
    //value without using the let keyword. The other difference is that we are creating a new variable
    //when using let, which means that we can change the type of the value.


    // --------  DATA TYPES

    //Every value in Rust is a certain data type
    //Rust is a statically typed language, which means that it must know the types of all variables at compile time
    //If we are parsing to another type we need to add type annotation ... let guess : u32 = ... .parse()
    //Otherwise we get an error because the compiler needs more information to know what to parse to in this case.


    // Scalar types

    // A scalar type represents a single value
    // We have Integers, Floating-point numbers, Booleans, and Characters.


    //Integers

    //Integers have 8-, 16-, 32-, 64-, 128- bit values either signed or unsigned. unsigned is from 0 to positive limit. signed is negative to positive halved values of unsigned.
    //When compiling an integer that is outside of the range of a selected type such as 257 in u8 which only goes up to 255, two things can happen
    //if compiling in debug mode, it will panic in runtime.
    //if compiling in --release mode, it does not include checks for integer overflows, it rather performs complement wrapping.
    //Values greater than the maximum value wrap around to the minimum values that the type can hold. 256 becomes 0, 257 becomes 1.

    //Floating type

    //f32 and f64 are the two primitve types for floating-numbers. f32 has single-precision, 24 bits of precision, which allows for
    //approximately 7 decimal digits of accuracy.
    //f64 has double-precision, 53 bits of precision, which allows for approximately 15 decimal digits of accuracy.

    let single_precision: f32 = 1.2345678;
    let double_precision: f64 = 1.234567890123456;
    println!("{single_precision} and {double_precision}");

    // Rust supports + - / * % numeric operations

    //Rust has bools

    let _t = true;
    let _f: bool = false;

    //Rust has chars - They are four bytes of size, so they support way more than ASCII (Unicode Scalar Value)

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';


    // Compound Types

    // Tuple Type

    // Tuples have fixed length and cannot grow or shrink in size. Can have varying types and values.
    // Added optional type annotations in the example.

    // Pattern matching to get out values


    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x,y,_z) = tup;
    println!("The value of y is {y}");

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // Array Type

    let _a = [1,2,3,4,5];

    // Data is allocated in stack instead of heap

    // Number of elements won't change

    let _b: [i32; 5] = [1,2,3,4,5];

    let _c: [i32; 5];

    let _d = [3; 5];

    let _first = _a[0];
    let _second = _a[1];


    // --------  COMMENTS


    // --------  CONTROL FLOW
}
