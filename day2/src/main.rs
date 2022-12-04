use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let use_test_input = args.contains(&String::from("test_input"));
    
    let input = if use_test_input {
            "A Y\nB X\nC Z"
    } else {
        include_str!("../input.txt")
    };

    run_part1(input);
    run_part2(input);
}

fn run_part1(input: &str) {
    let result: u32 = input
        .split("\n")
        .map(|game| {
            let first = game.chars().nth(0).unwrap_or('_');
            let second = game.chars().nth(2).unwrap_or('_');
            return match (first,second) {
                // DRAW
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3 + score_of(second),
                // WIN
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6 + score_of(second),
                
                (_, 'X') | (_, 'Y') | (_, 'Z') => 0 + score_of(second),
                _ => 0 + 0
            }
        }).sum();
    
    println!("part1: {}", result);
}

fn run_part2(input: &str) {
    let result: u32 = input
        .split("\n")
        .map(|game| {
            let first = game.chars().nth(0).unwrap_or('_');
            let second = game.chars().nth(2).unwrap_or('_');
            return match (first,second) {
                // DRAW
                (_, 'Y') => 3 + score_of(first),
                // WIN
                ('A', 'Z') => 6 + score_of('Y'),
                ('B', 'Z')  => 6 + score_of('Z'),
                ('C', 'Z')  => 6 + score_of('X'),
                // lose
                ('A', 'X') =>  score_of('Z'),
                ('B', 'X')  => score_of('X'),
                ('C', 'X')  => score_of('Y'),

                _ => 0 + 0
            }
        }).sum();
    
    println!("part2: {}", result);
}

fn score_of(selection: char) -> u32 {
    let lookup = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);
    return lookup[&selection]
}