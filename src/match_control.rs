#[derive(Debug)] // so we can inspect the state in println!
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn main() {
    let res = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("res: {}", res);

    // Matching all other cases
    let dice_roll = 9;
    match dice_roll {
        3 => println!("You rolled a 3!"),
        7 => println!("You rolled a 7!"),
        other => println!("You rolled a some other value: {}", other),
    }
    match dice_roll {
        3 => println!("You rolled a 3!"),
        7 => println!("You rolled a 7!"),
        _ => println!("You rolled a some other value!"),
    }
    match dice_roll {
        3 => println!("You rolled a 3!"),
        7 => println!("You rolled a 7!"),
        _ => (),
    }
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not configured");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
