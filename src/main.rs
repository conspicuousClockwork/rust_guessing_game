//Pulling in the input/output library from the Standard library (std)
use std::io;

fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    //The keyword "let" indicates that we are initializing a new variable
    //Variables are immutable by default in Rust. Therefore we are using the keyword "mut" to indicate mutability at variable declaration
    //String::new is a function that returns a new instance of String, a type provided by the Standard library
    //The syntax "::" indicates an associated function to a type, rather than an instance of that type. This can be also referred to as a "static" method
    let mut guess = String::new();

    //Calling the associated function on io, the library we imported earlier
    //io::stdin is a function returning an instance of std::io::Stdin, a type representing a handle to the standard input from terminal
    //read_line is a function taking in a string to assign the input to
    //We use & to indicate a reference to an existing variable
    //By default, references are immutable by default, therefore we use the "mut" keyword again to indicate the reference is mutable. Otherwise, we would just use "&guess"
    io::stdin().read_line(&mut guess)
        //Hello
        .expect("Failed to read line.");

    println!("You guessed {}!", guess);
}
