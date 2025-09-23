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
/*
fn plus_one(x: Option<i32>) -> Optoin<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
*/

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
    }
}
/*
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
*/

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}


