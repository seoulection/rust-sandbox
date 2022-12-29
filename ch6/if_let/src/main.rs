fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("max is configured to be {}", max),
        _ => (),
    }

    // in cases where we only want to pattern match on one value and ignore the rest, use if-let
    if let Some(max) = config_max {
        println!("max is configured to be {}", max);
    }

    // in cases where catch-all runs code, we can use if-let-else
    let some_number = Some(4u8);
    // let some_number: Option<u8> = None;
    match some_number {
        Some(number) => println!("some number: {}", number),
        _ => println!("wut"),
    }

    if let Some(number) = some_number {
        println!("some number: {}", number);
    } else {
        println!("wut");
    }
}
