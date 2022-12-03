use itertools::Itertools;

fn main() {
    println!("Running Day 1");

    let input = include_str!("../input.txt");

    let elves: Vec<u32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
            .map(|food| food.parse::<u32>().unwrap_or(0))
            .sum()
        })
        .sorted()
        .rev()
        .collect();
    
    println!("part1: {}", elves[0]);
    println!("part2: {:?}", elves.iter().take(3).sum::<u32>());
}