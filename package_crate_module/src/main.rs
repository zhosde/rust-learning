use std::collections::HashMap;
use std::fmt;
//use std::io::Result as IoResult;
use std::{cmp::Ordering, io}; // nested paths to clean up
use std::io::{self, Write};

fn f1() -> fmt::Result {}

fn f2() -> IoResult {}

fn main() {
    println!("Hello, world!");

    let mut map = HashMap::new();
    map.insert(1, 2);
}
