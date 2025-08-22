


use std::cmp::Ordering; // Ordering type is another enum with variants -- Less, Greater, Equal
use std::io;
use rand::Rng;



fn main() {

    
    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret_number is: {secret_number}");

    loop {

    println!("Guess the number!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };


    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => { 
            println!("You win!"),
            break;
        }
    }

    }

}

