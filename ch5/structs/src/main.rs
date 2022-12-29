struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
    username: String,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("ronswanson@parksandrec.pawnee.gov"),
        username: String::from("ronswanson"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "active: {}, email: {}, count: {}, username: {}",
        user1.active, user1.email, user1.sign_in_count, user1.username
    );

    let mut user2 = User {
        email: String::from("ronswanson@parksandrec.pawnee.gov"),
        username: String::from("ronswanson"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("aprilludgate@parksandrec.pawnee.gov");

    println!("email was changed: {}", user2.email);

    let user3 = build_user(String::from("new@email.com"), String::from("hello"));

    println!(
        "active: {}, email: {}, count: {}, username: {}",
        user3.active, user3.email, user3.sign_in_count, user3.username
    );

    // using most of user1's attributes but updating some
    // uses update syntax
    let user4 = User {
        email: String::from("thisisanew@email.com"),
        ..user1 // must come last to specify remaining fields come from user1
    };
    // user1 is no longer valid since we're moving the String username into user4
    // only using values that implement the Copy trait would make user1 still valid

    println!(
        "active: {}, email: {}, count: {}, username: {}",
        user4.active, user4.email, user4.sign_in_count, user4.username
    );

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(2, 2, 2);

    // can be accessed like regular tuples
    println!("black.0: {}, origin.1: {}", black.0, origin.1);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email, // same as email: email
        sign_in_count: 1,
        username, // same as username: username
    }
}
