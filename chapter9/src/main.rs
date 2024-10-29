use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_userName_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {

    // let v = vec![1, 2, 3];
    // 境界外アクセス
    // v[99];

    // パニックを発生させてみるテスト
    // panic!("ouch!!!!");

    // ファイルが存在しない場合のエラー処理
    match File::open("hello.txt") {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => {
                    panic!("Tried to create file but there was a problem: {:?}", error);
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        },
    };

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // panic!すべきかするまいか
    // https://doc.rust-jp.rs/book-ja/ch09-03-to-panic-or-not-to-panic.html
}
