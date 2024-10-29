enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // ...
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(c: &Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let message = Message::Write(String::from("hello"));
    message.call();

    let some_number = Some(5);
    let optional_value: Option<i32> = None;

    println!("some_number + 1 {:?}", plus_one(&some_number));
    println!("optional_value + 1 {:?}", plus_one(&optional_value));

    let coin = Coin::Quarter;
    println!("Value in cents: {}", value_in_cents(&coin));

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Not three");
    }

}
