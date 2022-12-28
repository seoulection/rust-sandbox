fn main() {
    let number = 6;

    // "if" condition MUST evaluate to a bool
    // if number { ... } won't work
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    // "if" is an expression
    let condition = true;
    // expressions MUST return the same type
    let number = if condition { 5 } else { 6 };

    println!("number: {number}");

    // 3 types of loops: loop, while, for
    loop_ex();
    loop_label_ex();
    while_ex();
    for_ex();
}

fn loop_ex() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_label_ex() {
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("end count: {count}");
}

fn while_ex() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("LIFTOFF");
}

fn for_ex() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // imperative using "while"
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // declarative using "for"
    for element in a {
        println!("the value is: {element}");
    }

    // using a range
    // .rev() reverses the range
    // (1..4) lower bound inclusive, upper bound exclusive
    // (1..=4) lower/upper bound inclusive
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF");
}
