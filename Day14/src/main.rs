use itertools::Itertools;
use std::fs;
use std::time;

fn part1(template: &str, rules: &[Rule]) -> usize {
    do_steps(template, rules, 10)
}

fn part2(template: &str, rules: &[Rule]) -> usize {
    do_steps(template, rules, 40)
}

fn do_steps(template: &str, rules: &[Rule], num_steps: usize) -> usize {
    let mut cur_template: String = template.to_string();

    for step in 0..num_steps {
        println!("Step: {} / {}", step + 1, num_steps);
        let mut new_template: String = String::new();
        new_template.push_str(&cur_template[0..1]);
        for i in 0..cur_template.len()-1 {
            let pair = &cur_template[i..i+2];
            let output = get_output(rules, pair);
            if output.is_none() {
                continue;
            }

            let output = output.unwrap();
            new_template.push_str(&output);
            new_template.push_str(&pair[1..2]);
        }

        if step == 4 {
            println!("Step 4: {}", new_template);
        }
        cur_template = new_template;
    }

    let sorted_template = cur_template.chars().sorted().collect::<String>();
    let groups: Vec<(char, usize)> = sorted_template.chars().group_by(|c| *c).into_iter().map(|(c, g)| (c, g.count())).collect();
    let max_freq = groups.iter().max_by_key(|(_, freq)| *freq).unwrap();
    let min_freq = groups.iter().min_by_key(|(_, freq)| *freq).unwrap();

    max_freq.1 - min_freq.1
}

#[derive(Debug, Clone)]
struct Rule {
    input: String,
    output: String,
}

impl Rule {
    fn new(input: &str, output: &str) -> Rule {
        Rule {
            input: input.to_string(),
            output: output.to_string(),
        }
    }
}

fn get_output(rules: &[Rule], input: &str) -> Option<String> {
    for rule in rules {
        if rule.input == input {
            return Some(rule.output.to_string());
        }
    }
    None
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    let input = fs::read_to_string("example.txt").expect("Unable to read file");
    //let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<&str> = input.lines().collect();

    let template = input[0];
    let rules: Vec<Rule> = input[2..]
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            Rule::new(parts[0], parts[1])
        })
        .collect();

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&template, &rules);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&template, &rules);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
