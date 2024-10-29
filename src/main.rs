use std::fs::{
    self, DirBuilder, rename
    };


fn main() {
    fs::rename("from", "to").unwrap();
    println!("Hello, world!");
}
