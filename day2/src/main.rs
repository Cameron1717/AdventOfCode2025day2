use std::fs;
use std::io::{self, BufRead};
fn main() {
    println!("advent of code day 2 script");
    let mut ranges: Vec<&str>;
    let contents = fs::read_to_string("./input.txt").unwrap();
    ranges = contents.split(',').collect();
    println!("{:?}", ranges);
}
