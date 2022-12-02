use std::time::Instant;

fn main() {
    day1();
    day2();
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

fn day2() {
    let input = include_bytes!("../data/day2.txt");
    // outcomes using strategy 1: where outcomes1[3*i+j] is score if opponent plays i, I play j (i, j are 0, 1, or 2)
    let outcomes1 = [3+1, 6+2, 0+3, 0+1, 3+2, 6+3, 6+1, 0+2, 3+3];
    // outcomes using strategy 2: where outcomes1[3*i+j] is score if opponent plays i, I play j (i, j are 0, 1, or 2)
    let outcomes2 = [0+3, 3+1, 6+2, 0+1, 3+2, 6+3, 0+2, 3+3, 6+1];

    let start = Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    for c in input.chunks_exact(4) {
        let index = (3*(c[0]-b'A') + (c[2]-b'X')) as usize;
        part1 += outcomes1[index];
        part2 += outcomes2[index];
    }
    let elapsed = Instant::now()-start;
    println!("Day 2");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
    println!("  Elapsed time: {:?}", elapsed);
}