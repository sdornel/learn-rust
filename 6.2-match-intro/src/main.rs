// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }
// fn main() {
//     let value = value_in_cents(Coin::Quarter(UsState::Alaska));
//     println!("The value of the coin is: {}", value);
// }

// There’s one other aspect of match we need to discuss: 
// the arms’ patterns must cover all possibilities
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("five: {:?}", five);
//     println!("six: {:?}", six);
//     println!("none: {:?}", none);
// }

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}


fn main() {
    let x = 1;
    let y = 2;

    // if x == 1 {
    //     println!("x is one");
    // } else if y == 2 {
    //     println!("y is two");
    // } else {
    //     println!("x is not one and y is not two");
    // }

    // _ is a special pattern that matches any value and 
    // does not bind to that value. This tells Rust we aren’t 
    // going to use the value, so Rust won’t warn us about an 
    // unused variable.
    match x {
        1 => println!("x is one"),
        _ => println!("x is not one"),
    }

    match y {
        2 => println!("y is two"),
        _ => println!("y is not two"),
    }
}

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}