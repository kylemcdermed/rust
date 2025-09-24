
/*
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
*/

/*
// using if let  
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
*/

/*
// using else with if let 
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}"),
    _ => count += 1,
}
*/

// using if let and else expression like this 
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}");
} else {
    count += 1
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // --snip--
        }
    }
}
/*
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
*/

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new"))
    } 
} 







