fn main() {
    // a string slice is a reference to part of a String
    let s = String::from("hello world");
    let len = s.len();
    let hello = &s[0..=4];      // same as [0..5], [..5], [..=4]
    let world = &s[6..11];      // same as [6..len], [6..]

    // slice of entire string
    let entire_slice = &s[0..len]; // same as [..]

    println!("{} {}", hello, world);
    println!("entire slice: {}", entire_slice);

    let the_first_word = first_word(&s);

    println!("first word: {the_first_word}");

    let my_string = String::from("goodbye world");

    // changing the function signature from &String to &str allows for references to Strings and
    // string slices to be passed
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string);

    let my_string_literal = "goodbye world"; // of type &str

    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
