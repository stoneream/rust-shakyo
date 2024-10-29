use rand::Rng;

fn main() {
    // Vectorで遊んでみるテスト
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(1..101);
    let mut v: Vec<i32> = Vec::new();

    println!("x = {}", x);

    for i in 0..x {
        v.push(i + 1);
    }

    for i in v.iter() {
        println!("{}", i);
    }

    // Stringで遊んでみるテスト
    // StringはVec<u8>のラッパー
    let mut s = String::from("Foo");
    s.push_str("Bar");
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // s1の所有権がs3に移動する
    let s3 = s1 + &s2;

    println!("{}", s3);
    // これはエラー!!
    // println!("{}", s1);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", s7);
    println!("s4 = {}", s4);
    println!("s5 = {}", s5);
    println!("s6 = {}", s6);

    let s8 = String::from("ハロワハロワー");
    let s8_chars_count = s8.chars().count();
    let s8_len = s8.len();
    println!("s8_len = {}", s8_len); // バイト数であることに注意
    println!("s8_chars_count = {}", s8_chars_count);
    // 書記素クラスタを扱う場合は何らかのクレート使ったほうがいいっぽい

    // ハッシュマップで遊んでみるテスト
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let field_name = String::from("Red");
    let field_value = 30;
    scores.insert(field_name, field_value);

    // 所有権が移動しているのでエラーになる
    // println!("{:?}", field_name);

}
