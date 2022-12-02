fn main() {
    day1();
}

pub fn day1() {
    let input = std::fs::read_to_string("data/day1.txt").unwrap();
    let mut elves = input.split("\r\n\r\n")
        .map(|elf| {
            elf.lines().map(|n| n.parse::<i32>().unwrap()).sum::<i32>()
        }
        )
        .collect::<Vec<i32>>();
    elves.sort();
    
    let len = elves.len();

    println!("Part 1: {}", elves[len-1]);
    println!("Part 2: {}", elves[len-1]+elves[len-2]+elves[len-3]);
}