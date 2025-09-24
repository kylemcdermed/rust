
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


