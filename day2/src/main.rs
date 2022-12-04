use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("Running Day 2");

    let input = include_str!("../input.txt");

//     let input = r#"A Y
// B X
// C Z"#;

    let lookup = HashMap::from([
        (Some('X'), 1),
        (Some('Y'), 2),
        (Some('Z'), 3)
    ]);

    let games: u32 = input
        .split("\n")
        .map(|game| {
            let first = game.chars().nth(0);
            let second = game.chars().nth(2);
            return match (first,second) {
                // DRAW
                (Some('A'), Some('X')) | (Some('B'), Some('Y')) | (Some('C'), Some('Z')) => 3 + lookup[&second],
                // WIN
                (Some('A'), Some('Y')) | (Some('B'), Some('Z')) | (Some('C'), Some('X')) => 6 + lookup[&second],
                
                (_, Some('X')) | (_, Some('Y')) | (_, Some('Z')) => 0 + lookup[&second],
                _ => 0 + 0
            }
        }).sum();
    
    println!("part1: {}", games);
}