use std::env;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let use_test_input = args.contains(&String::from("test_input"));
    
    let input = if use_test_input {
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
    } else {
        include_str!("../input.txt")
    };

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