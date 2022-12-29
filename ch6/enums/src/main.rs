#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}, loopback; {:?}", home, loopback);

    // like structs, enums can define their own methods using "impl"
    let message = Message::Write(String::from("write me"));
    message.call();

    // Option<T> -> Some(T) or None
    let _some_number = Some(5);
    let _some_char = Some('e');

    // for None values, we need to explicitly set the type that None can be
    // let none_number = None; won't work since Rust can't infer the type of None
    let _none_number: Option<i32> = None;
}
