use regex::Regex;

fn main() {
    println!("Running Day 1");

    let input = include_str!("../input.txt");
    
//     let input = r#"2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8"#;

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