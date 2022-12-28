fn main() {
    let s1 = String::from("hello");
    // &'s represent references, which allow you to refer to some value without taking ownership
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    // references are immutable by default, but we can use a mutable reference
    let mut s = String::from("hello");
    println!("s before change: {}", s);
    change(&mut s);

    println!("s after change: {}", s);

    // you cannot have multiple references to the same data in the same scope. this restriction
    // prevents data races
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;

    // we can create a new scope in order to have multiple references since they are not
    // simultaneous
    let mut x = String::from("hello");
    {
        let _r1 = &mut x;
    } // r1 goes out of scope, so we can make new references without any problems

    let _r2 = &mut s;

    // we CAN have multiple immutable references within the same scope since those references do
    // not alter the data
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but since it does not have ownership of what it refers to, it is not
  // dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
