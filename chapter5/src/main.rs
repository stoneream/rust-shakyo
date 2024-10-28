struct Person {
    name: String,
    age: u8,
}

fn print_person(p: &Person) {
    println!("Name: {}, Age: {}", p.name, p.age);
}

fn build_person(name: String, age: u8) -> Person {
    Person { name, age }
}

struct RGB(i32, i32, i32);

fn print_rgb(rgb: &RGB) {
    println!("R: {}, G: {}, B: {}", rgb.0, rgb.1, rgb.2);
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let p1 = build_person("Alice".to_string(), 30);

    print_person(&p1);

    let black = RGB(0, 0, 0);
    print_rgb(&black);

    let rect = Rect { width: 30, height: 50 };
    println!("Rect: {:?}", rect);
    println!("Area: {}", rect.area());
}
