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
    let input = include_str!("../data/day2.txt");
    let start = Instant::now();

    let calc_score1 = |game: &str| {
        let b = game.as_bytes();
        // opponents play as 0,1,2
        let opp = (b[0] - b'A') as i32;
        // my play as 0,1,2
        let my = (b[2] - b'X') as i32;
        if (my+1)%3 == opp { 0+my+1 }
        else if my == opp { 3+my+1 }
        else { 6+my+1 }
    };
    let calc_score2 = |game: &str| {
        let b = game.as_bytes();
        // opponents play as 0,1,2
        let opp = (b[0] - b'A') as i32;
        // my strat as 0,1,2
        let strat = (b[2] - b'X') as i32;
        if strat == 0 { 0+(opp+2)%3+1 }
        else if strat == 1 { 3+opp+1 }
        else { 6+(opp+1)%3+1 }
    };

    let part1: i32 = input.lines().map(calc_score1).sum();
    let part2: i32 = input.lines().map(calc_score2).sum();
    let elapsed = Instant::now()-start;
    println!("Day 2");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
    println!("  Elapsed time: {:?}", elapsed);
}