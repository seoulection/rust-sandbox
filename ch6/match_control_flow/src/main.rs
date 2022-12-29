#[derive(Debug)]
enum UsState {
    California,
    Illinois,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("penny value: {}", value_in_cents(Coin::Penny));
    println!("nickel value: {}", value_in_cents(Coin::Nickel));
    println!("dime value: {}", value_in_cents(Coin::Dime));
    println!(
        "quarter value: {}",
        value_in_cents(Coin::Quarter(UsState::California))
    );
    println!(
        "quarter value: {}",
        value_in_cents(Coin::Quarter(UsState::Illinois))
    );

    // matching with Option<T>
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // catch-all using value
    let dice_roll = 9;
    match dice_roll {
        3 => handle_three(),
        7 => handle_seven(),
        other => handle_other(other),
    }

    // catch-all ignoring value
    // _ => () means don't run any code in all other cases
    match dice_roll {
        3 => handle_three(),
        7 => handle_seven(),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Lucky dime");
            10
        }
        Coin::Quarter(state) => {
            println!("state: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match is exhaustive, and Rust will know if we didn't cover all cases
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn handle_three() {}
fn handle_seven() {}
fn handle_other(_other: i32) {}
