// variables are immutable by default. they hold data or references to data.
// Rust is block scoped.
pub fn run() {

    // define variable
    let name = "james"; // an immutable variable
    let mut age = 35; // a mutable type

    println!("my, name is {} and am {} yrs old", name, age);

    age = 36;

    println!("my, name is {} and am {} yrs old", name, age);


    // define constant
    // constants must have a type
    const ID: i32 = 001;
    println!("my, name is {} and am {} yrs old", name, age);
    println!("My ID is: {}", ID);

    // multiple variables
    let (var_a, var_b) = ("asdf", "qwerty");
    println!("variable a: {} variable b: {}", var_a, var_b);
}
