use std::fmt::Display;

fn largest_number(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item >largest {
            largest = item;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> T
where T: PartialOrd + Copy
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
fn largest<T>(list: &[T: PartialOrd + Copy]) -> T
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
 */

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Pointt<T> {
    x: T,
    y: T
}

impl<T> Pointt<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("largest number is {}", largest_number(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("largest char is {}", largest_char(&char_list));

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    let p = Pointt { x: 5, y: 10 };
    println!("p.x = {}", p.x());


    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    // p2の所有権はp3に移動している点に注意
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
