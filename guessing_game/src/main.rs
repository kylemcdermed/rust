

use std::io; // to obtain user input/output we need input/output standard library 



fn main() { // fn declares a new function

    // println! is a macro that prints a string to the screen 
    println!("Guess the number!"); 
    println!("Please input your guess.");

    // `let` keyword allows us to create a variable 
    // variables are immutable by default (the value wont change)
    // `mut` keyword allows us to make a variable mutable
    // `::new` is an associated function of String type, this function creates a new, empty string
    let mut guess = String::new();

    
    // if we had not imported `use std::io` we could still use std::io::stdin
    // .read_line() handles/gets the input from the user
    // passing `&mut guess` by reference allows the code to access the data without copying data
    // read_line() returns a Result value. Result's variants are `Ok` and `Err`
    // `Ok` indicates successfully generated value
    // `Err` indicates operation failed
    // if the read_line() Result returns `Ok` value, expect() will return value `Ok` so you can use
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // the {} set of curly brackets is a placeholder
    // think of it as a crabs princers that holds a value in place
    // when printing the value, the variable name can go inside the {} curly brackets
    println!("You guessed: {guess}");
}
