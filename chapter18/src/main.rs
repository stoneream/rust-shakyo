
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let x = None;
    let y = 10;

    match x {
        Some(10..50) => println!("Got 10 ~ 50"),
        Some(y) => println!("Matched, y={:?}", y),
        _ => println!("Default case, x={:?}", x),
    }

    println!("at the end: x={:?}, y={:?}", x, y);

    let p = Point { x:0 , y:7 };
    let Point { x: a, y: b } = p;
    assert!(a == 0 && b == 7);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move {x, y} => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }

    let points = vec![
        Point { x: 0, y: 0},
        Point { x: 1, y: 5},
        Point { x: 10, y: -5},
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    println!("sum_of_squares: {}", sum_of_squares);

    let ((feet, inches), Point {x, y}) = ((3,10), Point{x:3, y:-10});

    let mut settings_value = Some(5);
    let new_settings_value = Some(10);

    match (settings_value, new_settings_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            settings_value = new_settings_value;
        }
    }

    println!("settings is {:?}", settings_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let s = Some(String::from("Hello"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point {x, ..} => println!("x is {}", x),
    }

    let mut robot_name = Some(String::from("Bors"));

    // refキーワードで参照を取得
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    println!("robot_name is {:?}", robot_name);

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(n) if n == y => println!("{}", x),
        None => (),
        _ => {}
    }

    println!("at the end: x={:?}, y={:?}", x, y);


}
