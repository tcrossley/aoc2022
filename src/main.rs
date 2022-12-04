use std::time::Instant;

fn main() {
    day1();
    day2();
    day3();
    day4();
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

fn next_group<'a, I: Iterator<Item=&'a str>>(iter: &mut I) -> Option<(&'a str, &'a str, &'a str)> {
    if let Some(a) = iter.next() {
        if let Some(b) = iter.next() {
            if let Some(c) = iter.next() {
                return Some((a, b, c));
            }
        }
    }
    None
}

fn day3() {
    let start = Instant::now();
    let input = include_str!("../data/day3.txt");
    let count_items = |s: &str| {
        let mut counts = [0;52];
        for b in s.bytes() {
            match b {
                b'a'..=b'z' => counts[(b-b'a') as usize] += 1,
                b'A'..=b'Z' => counts[(b-b'A'+26) as usize] += 1,
                _ => {}
            }
        }
        counts
    };
    let mut part1 = 0;
    for line in input.lines() {
        let (p1, p2) = line.split_at(line.len()/2);
        let counts1 = count_items(p1);
        let counts2 = count_items(p2);
        for i in 0..counts1.len() {
            if counts1[i] > 0 && counts2[i] > 0 {
                part1 += i+1;
                break;
            }
        }
    }

    // Part 2
    let mut part2 = 0;
    let mut lines = input.lines();
    while let Some((a, b, c)) = next_group(&mut lines) {
        let c1 = count_items(a);
        let c2 = count_items(b);
        let c3 = count_items(c);

        for i in 0..c1.len() {
            if c1[i] > 0 && c2[i] > 0 && c3[i] > 0 {
                part2 += i+1;
                break;
            }
        }
    }


    let elapsed = Instant::now()-start;
    println!("Day 3");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
    println!("  Elapsed time: {:?}", elapsed);
}

fn day4() {
    let start = Instant::now();
    let input = include_str!("../data/day4.txt");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let (a1, a2) = a.split_once('-').unwrap();
        let (a1, a2) = (a1.parse::<i32>().unwrap(), a2.parse::<i32>().unwrap());
        let (b1, b2) = b.split_once('-').unwrap();
        let (b1, b2) = (b1.parse::<i32>().unwrap(), b2.parse::<i32>().unwrap());
        
        if (a1 <= b1 && b2 <= a2) || (b1 <= a1 && a2 <= b2) {
            part1 += 1;
        }

        if (a1 <= b2 && a2 >= b1) || (b1 <= a2 && b2 >= a1) {
            part2 += 1;
        }
    }


    let elapsed = Instant::now()-start;
    println!("Day 4");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
    println!("  Elapsed time: {:?}", elapsed);
}