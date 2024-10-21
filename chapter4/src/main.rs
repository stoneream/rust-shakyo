fn main() {
    let s1 = givens_ownership();
    let s2 = String::from("World");
    let s3 = takes_and_gives_back(s2);
    let (s4, len) = calculate_length(s3);
}

fn givens_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}