


use std::cmp::Ordering; // Ordering type is another enum with variants -- Less, Greater, Equal
use std::io;
use rand::Rng;



fn main() {

    // --snip--
    
    println!("You guessed: {guess}");

    // cmp method compares two values and can be called on anything being compared
    // here its comparing guess to secret number
    // then it returns a variant of the Ordering enum we brought into scope with the use statement
    // we use match expression to decide what to do next based on which variant of Ordering was
    // returns from the call to cmp with values in guess and secret_number
    // match expression is made up of arms, an arm consists of a pattern to match against
    // Rust takes the value guven to match and looks through each arms pattern in turn
    match(guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win~"),
    }
    // lets say the user guessed 50 and randomly generated secret_number 38
    // when match compares guess onto secret_number, it will returns Ordering::Greater because 50
    // is greater than 38
    // the code checks each arms pattern which arms matches the expression and will execute and
    // print, in this case "Too big!"

}


