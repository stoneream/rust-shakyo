use std::collections::HashMap;

use std::io::Result as IoResult;
use std::fmt::Result;

fn func1() -> Result {
    Ok(())
}

fn func2() -> IoResult<()> {
    Ok(())
}


fn main() {

    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("Hello, world!");
}
