fn main() {
    // stored in heap
    let s1 = String::from("hello");
    // let s2 = s1; is bad since s2 copies the pointer, length, and capacity
    // of String. it does not copy the data on the heap that the pointer refers
    // to. when s1 and s2 go out of scope, this causes a double-free error, and
    // Rust prevents this by "moving"
    // therefore, let s2 = s1 means that s1 is invalidated and cannot be used

    // clone allows for deep copy of the heap data
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);

    // stack-only data
    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);

    // Copy trait is placed on types that are stored on the stack

    // ownership and functions
    let s = String::from("hello");      // s comes into scope
    takes_ownership(s);                 // s's value moves into the function, no longer valid in
                                        // this scope
    let r = 5;                          // r comes into scope
    makes_copy(r);                      // r moves into function, but i32 is Copy so we can still
                                        // use r afterwards

    println!("{}", r);

    // return values and scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}, s3: {}", s1, s3);
}

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{}", some_string);            // some_string goes out of scope and "drop" is called.
                                            // the backing memory is freed
}

fn makes_copy(some_integer: i32) {          // some_integer comes into scope
    println!("{}", some_integer);           // some_integer goes out of scope. nothing special
                                            // happens
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
