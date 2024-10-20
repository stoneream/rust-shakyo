use std::io;

fn factorial(n: u64) -> u64 {
    return if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn array_test() {
    let a = [1,2,3,4,5,6,7,8,9];

    let mut input_index = String::new();

    io::stdin().read_line(&mut input_index).expect("Failed to read line");

    let index: usize = input_index.trim().parse().expect("Please type a number!");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}

fn loop_test() {
    let b = [10, 20, 30, 40, 50];

    for element in b {
        println!("The value of b is: {}", element);
    }

    let mut c = 10;

    while c != 0 {
        println!("The value of c is: {}", c);
        c -= 1;
    }

    for element in (1..10).rev() {
        println!("The value of element is: {}", element);
    }
}

fn main() {

    array_test();

    let n = 5;
    let result = factorial(n);
    println!("The factorial of {} is: {}", n, result);

    loop_test();

}
