use fancy_regex::Regex;
use std::fs;
fn main() {
    println!("advent of code day 2 script");
    let ranges: Vec<&str>;
    let contents = fs::read_to_string("./input.txt").unwrap();
    ranges = contents.trim().split(',').collect();
    let ranges2 = ranges.clone();
    let result: i64 = ranges.iter().map(|x| find_bad_ids(x)).sum();
    let result2: i64 = ranges2.iter().map(|y| find_bad_ids_2(y)).sum();
    println!("answer for part 1: {}", result);
    println!("answer for part 2: {}", result2);
}

fn split_bounds(boundry: &str) -> (&str, &str) {
    let bounds: Vec<&str> = boundry.split('-').collect();
    (bounds[0], bounds[1])
}
fn find_bad_ids(boundry: &str) -> i64 {
    let (lower, upper) = split_bounds(boundry);
    // println!("{}", lower);
    // println!("{}", upper);
    let i_lower: i64 = lower.parse().expect("failed to parse i64 lower");
    let i_upper: i64 = upper.parse().expect("failed to parse i64 upper");
    let valid_fakes: Vec<i64> = (i_lower..=i_upper)
        .filter(|&x| x.to_string().len() % 2 == 0)
        .collect();
    let bad_ids = |&x: &i64| {
        let x_string = x.to_string();
        let half = x_string.len() / 2;
        let (part1, part2) = x_string.split_at(half);
        if part1 == part2 { true } else { false }
    };
    let result: i64 = valid_fakes.iter().filter(|x| bad_ids(x)).sum();
    return result;
}
fn find_bad_ids_2(boundry: &str) -> i64 {
    let (lower, upper) = split_bounds(boundry);
    //println!("{}", lower);
    //println!("{}", upper);
    let i_lower: i64 = lower.parse().expect("failed to parse i64 lower");
    let i_upper: i64 = upper.parse().expect("failed to parse i64 upper");
    let reg = Regex::new(r"^(.+)\1+$").unwrap();
    let valid_fakes: Vec<String> = (i_lower..=i_upper)
        .map(|x| x.to_string())
        .filter(|y| reg.is_match(y).unwrap())
        .collect();
    let result = valid_fakes.iter().map(|z| z.parse::<i64>().unwrap()).sum();
    result
}
