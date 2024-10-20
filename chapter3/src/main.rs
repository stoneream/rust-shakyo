use std::io;

fn main() {

    let a = [1,2,3,4,5,6,7,8,9];

    let mut input_index = String::new();

    io::stdin().read_line(&mut input_index).expect("Failed to read line");

    let index: usize = input_index.trim().parse().expect("Please type a number!");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);

    println!("Hello, world!");
}
