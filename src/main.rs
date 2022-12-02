use std::time::Instant;

fn main() {
    day1();
}

pub fn day1() {
    let input = include_str!("../data/day1.txt");
    let start = Instant::now();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            if current > a { c=b; b=a; a=current; }
            else if current > b { c=b; b=current; }
            else if current > c { c=current; }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    let elapsed = Instant::now()-start;
    println!("Day 1");
    println!("  Part 1: {}", a);
    println!("  Part 2: {}", a+b+c);
    println!("  Elapsed time: {:?}", elapsed);
}