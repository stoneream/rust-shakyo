enum List1 {
    Cons(i32, Box<List1>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!!", name)
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum List2{
    Cons(i32, Rc<List2>),
    Nil,
}

use std::rc::Rc;

#[derive(Debug)]
enum List3{
    Cons(Rc<RefCell<i32>>, Rc<List3>),
    Nil,
}

use std::cell::RefCell;

#[derive(Debug)]
enum List4 {
    Cons(i32, RefCell<Rc<List4>>),
    Nil,
}

impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<List4>>> {
        match *self {
            List4::Cons(_, ref item) => Some(item),
            List4::Nil => None,
        }
    }
}

use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = List1::Cons(1,
                    Box::new(List1::Cons(2,
                                  Box::new(List1::Cons(3,
                                                Box::new(List1::Nil))))));

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let t = 5;
    let u = MyBox::new(t);

    assert_eq!(5, t);
    assert_eq!(5, *u);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff")};
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let e = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    println!("count after creating e = {}", Rc::strong_count(&e));
    let f = List2::Cons(3, Rc::clone(&e));
    println!("count after creating f = {}", Rc::strong_count(&e));
    {
        let g = List2::Cons(4, Rc::clone(&e));
        println!("count after creating g = {}", Rc::strong_count(&e));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&e));

    let     value = Rc::new(RefCell::new(5));
    let h = Rc::new(List3::Cons(Rc::clone(&value), Rc::new(List3::Nil)));
    let i = List3::Cons(Rc::new(RefCell::new(6)), Rc::clone(&h));
    let j = List3::Cons(Rc::new(RefCell::new(10)), Rc::clone(&h));

    *value.borrow_mut() += 10;

    println!("h after = {:?}", h);
    println!("i after = {:?}", i);
    println!("j after = {:?}", j);

    let k = Rc::new(List4::Cons(5, RefCell::new(Rc::new(List4::Nil))));
    println!("k initial rc count = {}", Rc::strong_count(&k));
    println!("k next item = {:?}", k.tail());
    let l = Rc::new(List4::Cons(10, RefCell::new(Rc::clone(&k))));
    println!("k rc count after creating l = {}", Rc::strong_count(&k));
    println!("l initial rc count = {}", Rc::strong_count(&l));
    println!("l next item = {:?}", l.tail());
    if let Some(link) = k.tail() {
        *link.borrow_mut() = Rc::clone(&l);
    }
    println!("l rc count after changing k = {}", Rc::strong_count(&l));
    println!("k rc count after changing k = {}", Rc::strong_count(&k));
    // stack overflow !!
    // println!("k next item = {:?}", k.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

}
