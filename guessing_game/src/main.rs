

use std::io;  
use rand::Rng; // Rng trait defines the method that the random number generator implements


fn main() { 

     
    println!("Guess the number!"); 

    // rand::thread_rng function gives us the particular random number generator we will use
    // gen_range method takes a range expression as argument and generates a random number in range
    // the range expression takes the form (start..=end) , inclusive on lower/upper bounds
    // so we specify 1..=100 to request a number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    
    
    
    println!("You guessed: {guess}");
}
