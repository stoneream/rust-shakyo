use std::thread;
use std::time::Duration;

fn main() {

    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // スレッドをブロックして終了を待つ
    handle1.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    // moveキーワードを使ってスレッドに所有権を移す
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();
}
