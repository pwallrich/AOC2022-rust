use std::fs;
use itertools::Itertools;

fn main() {
    println!("Running Day 1");

    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

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