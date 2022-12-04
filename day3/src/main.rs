use std::env;
use itertools::Itertools;
use std::collections::{HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();

    let use_test_input = args.contains(&String::from("test_input"));
    
    let input = if use_test_input {
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
    } else {
        include_str!("../input.txt")
    };
    
    println!("Running Day 3. Using Test input: {}", use_test_input);

    run_part1(input);
    run_part2(input);
}

fn run_part1(input: &str) {
    let result = input
        .split("\n")
        .map(|rucksack| {
             return rucksack.split_at(rucksack.len() / 2)
        })
        .map(|rucksack| -> (HashSet<char>, HashSet<char>) {
            (HashSet::from_iter(rucksack.0.chars()), HashSet::from_iter(rucksack.1.chars()))
        })
        .map(|rucksack| {
            rucksack.0.intersection(&rucksack.1).copied().next()
        })
        .flatten()
        .map(|duplicate| {
            get_priority(duplicate)
        })
        .fold(0, |old, new| {
            old + new
        });

    println!("part1: {:?}", result);
}

fn run_part2(input: &str) {
    let result = input
    .split("\n")
    .tuples::<(_,_,_)>()
    .map(|rucksack| -> (HashSet<char>, HashSet<char>, HashSet<char>) {
        (HashSet::from_iter(rucksack.0.chars()), HashSet::from_iter(rucksack.1.chars()), HashSet::from_iter(rucksack.2.chars()) )
    })
    .map(|rucksack| {
        let first = rucksack.0.intersection(&rucksack.1).copied();
        return rucksack.2.intersection(&HashSet::from_iter(first)).copied().next();
    })
    .flatten()
    .map(|badge| {
        get_priority(badge)
    })
    .fold(0, |old, new| {
        old + new
    });

    println!("part2: {:?}", result);
}

fn get_priority(item: char) -> u32 {
    if item.is_uppercase() {
        return item as u32 - ('A' as u32) + 27;
    } else {
        return item as u32 - ('a' as u32) + 1;
    }
}