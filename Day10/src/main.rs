//use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::time;

fn part1(input: &[&str]) -> u64 {
    let mut map: HashMap<char, char> = HashMap::new();
    map.insert('(', ')');
    map.insert('{', '}');
    map.insert('[', ']');
    map.insert('<', '>');

    let mut score = 0;
    let mut stack: Vec<char> = Vec::new();
    for line in input.iter() {
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                let last = stack.pop().unwrap();
                let endc = map[&last];
                if endc != c {
                    match c {
                        ')' => score += 3,
                        '}' => score += 1197,
                        ']' => score += 57,
                        '>' => score += 25137,
                        _ => panic!("unmatched"),
                    }
                    break;
                }
            }
        }
    }

    score
}

fn part2(input: &[&str]) -> u64 {
    let mut map: HashMap<char, char> = HashMap::new();
    map.insert('(', ')');
    map.insert('{', '}');
    map.insert('[', ']');
    map.insert('<', '>');

    let mut scores: Vec<u64> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    for line in input.iter() {
        stack.clear();

        let mut is_invalid = false;
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                let last = stack.pop().unwrap();
                let endc = map[&last];
                if endc != c {
                    is_invalid = true;
                    break;
                }
            }
        }
        if is_invalid {
            continue;
        }

        let mut cur_score = 0;
        let score_str: String = stack.iter().collect();
        while stack.len() > 0 {
            let last = stack.pop().unwrap();
            cur_score *= 5;
            match last {
                '(' => cur_score += 1,
                '[' => cur_score += 2,
                '{' => cur_score += 3,
                '<' => cur_score += 4,
                _ => panic!("unmatched"),
            }
        }
        println!("{} -> {} = {}", line, score_str, cur_score);

        scores.push(cur_score);
    }

    scores.sort();

    scores[scores.len() / 2]
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<&str> = input.lines().collect();

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&input);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&input);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
