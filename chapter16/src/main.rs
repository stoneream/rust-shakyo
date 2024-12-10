use std::sync::{mpsc, Arc, Mutex};
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

    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("u")
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // mainスレッドでメッセージを受信し、表示する
    for received in rx {
        println!("Got: {}", received);
    }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
