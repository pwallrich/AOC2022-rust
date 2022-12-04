use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let use_test_input = args.contains(&String::from("test_input"));
    
    let input = if use_test_input {
            "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"
    } else {
        include_str!("../input.txt")
    };
    
    println!("Running Day 4. Using Test input: {}", use_test_input);

    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    for found in re.captures_iter(input) {
        let p1 = (&found[1].parse::<i32>().unwrap(), &found[2].parse::<i32>().unwrap());
        let p2 = (&found[3].parse::<i32>().unwrap(), &found[4].parse::<i32>().unwrap());

        if p1.0 <= p2.0 && p2.0 <= p1.1 || p2.0 <= p1.0 && p1.0 <= p2.1 {
            part2 += 1;
        } else {
            continue;
        }
        if p1.0 <= p2.0 && p1.1 >= p2.1 || p2.0 <= p1.0 && p2.1 >= p1.1 {
            part1 += 1;
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}