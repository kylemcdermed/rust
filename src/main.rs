#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip -- 
}

enum Coin {
    Penny,
    Nickle,
    Dime, 
    Quarter(UsState),
}
/*
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
*/
// using curly brackets in our fn value_in_cents()
fn value_in_cents(cents: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!")
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10, 
        Coin::Quarter => {
            println!("State quarter from {state:?}!");
            25,
        }
    }
}


