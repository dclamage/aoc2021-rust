use std::fs;
use std::time;
use std::collections::HashMap;

fn part1(template: &str, rules: &mut HashMap<String, Rule>) -> usize {
    do_steps(template, rules, 10)
}

fn part2(template: &str, rules: &mut HashMap<String, Rule>) -> usize {
    do_steps(template, rules, 40)
}

fn do_steps(template: &str, rules: &mut HashMap<String, Rule>, num_steps: usize) -> usize {
    let mut letter_counts = [0; 26];
    letter_counts[template.chars().nth(template.len() - 1).unwrap() as usize - 'A' as usize] += 1;
    for i in 0..template.len() - 1 {
        let key = &template[i..i + 2];
        let cur_letter_counts = get_letter_count(rules, key, num_steps);
        letter_counts = append_letter_counts(&letter_counts, &cur_letter_counts);
    }

    letter_counts.iter().max().unwrap() - letter_counts.iter().filter(|&x| *x > 0).min().unwrap()
}

#[derive(Debug, Clone)]
struct Rule {
    input: String,
    output: String,
    letter_count_cache: Vec<[usize; 26]>,
}

impl Rule {
    fn new(input: &str, output: &str) -> Rule {
        let mut rule = Rule {
            input: input.to_string(),
            output: output.to_string(),
            letter_count_cache: Vec::new(),
        };

        let mut letter_counts = [0; 26];
        for c in input[0..1].chars() {
            letter_counts[c as usize - 'A' as usize] += 1;
        }
        rule.letter_count_cache.push(letter_counts);

        rule
    }
}

fn append_letter_counts(a: &[usize; 26], b: &[usize; 26]) -> [usize; 26] {
    let mut result = [0; 26];
    for i in 0..26 {
        result[i] = a[i] + b[i];
    }
    result
}

fn get_letter_count(rules: &mut HashMap<String, Rule>, input: &str, index: usize) -> [usize; 26] {
    let (left_rule, right_rule, letter_count_cache_size) = {
        let rule = rules.get(input).unwrap();
        let left_rule = rule.input[0..1].to_string() + &rule.output[0..1].to_string();
        let right_rule = rule.output[0..1].to_string() + &rule.input[1..2].to_string();
        let letter_count_cache_size = rule.letter_count_cache.len();
        (left_rule, right_rule, letter_count_cache_size)
    };

    for cur_index in letter_count_cache_size..=index {
        let left_letter_count = get_letter_count(rules, &left_rule, cur_index - 1);
        let right_letter_count = get_letter_count(rules, &right_rule, cur_index - 1);
        let letter_counts = append_letter_counts(&left_letter_count, &right_letter_count);

        rules.get_mut(input).unwrap().letter_count_cache.push(letter_counts);
    }

    rules.get_mut(input).unwrap().letter_count_cache[index].clone()
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<&str> = input.lines().collect();

    let template = input[0];
    let mut rules: HashMap<String, Rule> = input[2..]
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            (parts[0].to_string(), Rule::new(parts[0], parts[1]))
        })
        .into_iter()
        .collect();

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&template, &mut rules);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&template, &mut rules);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
