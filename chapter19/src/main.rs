
unsafe fn dangerous() {}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "HELLO, WORLD!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // セグフォで落ちる
    // let address = 0x7ffeeb1b3b7c as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("address is: {}", *address);
    }

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // Spot
    println!("A baby dog is called a {}", Dog::baby_name());
    // puppy
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Thunk) {
    //     // --snip--
    // }
    //
    // fn returns_long_type() -> Thunk {
    //     // --snip--
    // }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

}
