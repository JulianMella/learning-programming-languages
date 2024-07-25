fn main() {
    println!("Hello, world!");

    another_function();

    //func_parameters("Hello"); - ERROR! Since it is pre defined what type the arguments is expected to be
    func_parameters(2);

    print_label_measurements(5, 'h');

    //Statements and expressions

    //Functions can optionally end with an expression

    //There is a clear distinction between a statement and expression

    //Statements are instructions that perform some action and do not return a value.
    //Expressions evaluate to a resultant value

    let _y = 6; //Statement
    // Function bodies are also statements and do not return a value, therefore the following gives an error:

    // let x = (let y = 6), not the same as C where you can type x = y = 6 and assign both x and y to be equal to 6. Rust is an expression based language

    let _z = {
        let x = 3;
        x + 1 // Expression doesn't have semi colon at the end
    };

    let _test_func_with_return_value = func_with_return_value();


}

fn func_with_return_value() -> u32 {
    let z = 2;
    z //Can use return, but not needed.
}
fn another_function() {
    println!("Another function.");
}

fn func_parameters(x: i32) {
    println!("The value of x is {x}");
}

fn print_label_measurements(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}")
}