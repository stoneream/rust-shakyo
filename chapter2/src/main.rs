use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("guess the number...");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("please input your guess!");

        // mut は「ミュータブル」の意
        let mut guess_number = String::new();

        io::stdin().read_line(&mut guess_number).expect("failed to read line");

        let guess: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number!");
                continue;
            }
        };

        println!("you guessed: {}", guess_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
