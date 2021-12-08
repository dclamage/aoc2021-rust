use itertools::Itertools;
use std::fs;
use std::time;

fn part1(input: &[&str]) -> i64 {
    let num_lines = input.len() / 14;
    let mut num_easy = 0;
    for line_index in 0..num_lines {
        for val in &input[line_index * 14 + 10..(line_index + 1) * 14] {
            match val.len() {
                2 | 3 | 4 | 7 => num_easy += 1,
                _ => (),
            }
        }
    }

    num_easy
}

fn display_to_int(display: &str) -> u32 {
    let mut bin_val = 0u32;
    for c in display.chars() {
        bin_val |= 1u32 << (c as u32 - 'a' as u32);
    }
    bin_val
}

const VALID_DIGITS: [u32; 10] = [
    0b1110111, 0b0010010, 0b1011101, 0b1011011, 0b0111010, 0b1101011, 0b1101111, 0b1010010,
    0b1111111, 0b1111011,
];
fn is_valid_permutation(line : &[u32]) -> bool {
    line.iter().all(|&digit| VALID_DIGITS.contains(&digit))
}

fn apply_permutation(perm : &[u32], digit : u32) -> u32 {
    let mut result = 0u32;
    for src_bit in 0..7 {
        let dst_bit = perm[src_bit];
        result |= ((digit >> src_bit) & 1) << dst_bit;
    }
    result
}

fn digit_to_numeral(digit : u32) -> u32 {
    VALID_DIGITS.iter().position(|&d| d == digit).unwrap() as u32
}

fn part2(input: &[&str]) -> u32 {
    let mut input_lines: Vec<Vec<u32>> = Vec::new();
    let mut output_lines: Vec<Vec<u32>> = Vec::new();

    let mut answer : u32 = 0;

    let num_lines = input.len() / 14;
    for line_index in 0..num_lines {
        let mut input_line = Vec::new();
        let mut output_line = Vec::new();
        for val in &input[line_index * 14..line_index * 14 + 10] {
            input_line.push(display_to_int(val));
        }
        input_lines.push(input_line);

        for val in &input[line_index * 14 + 10..(line_index + 1) * 14] {
            output_line.push(display_to_int(val));
        }
        output_lines.push(output_line);
    }

    for (line_index, input_line) in input_lines.iter().enumerate() {
        let mut found_valid = false;
        for perm in (0..7).permutations(7) {
            let line_perm : Vec<u32> = input_line.iter().map(|digit| apply_permutation(&perm, *digit)).collect();
            if is_valid_permutation(&line_perm) {
                let output_line : Vec<u32> = output_lines[line_index].iter().map(|digit| apply_permutation(&perm, *digit)).collect();
                if is_valid_permutation(&output_line) {
                    let line_result = digit_to_numeral(output_line[0]) * 1000 + digit_to_numeral(output_line[1]) * 100 + digit_to_numeral(output_line[2]) * 10 + digit_to_numeral(output_line[3]);
                    answer += line_result;
                    found_valid = true;
                    break;
                }
            }
        }

        if !found_valid {
            println!("No valid permutation found for line {}", line_index);
        }
    }

    answer
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<&str> = input
        .split(|c| c == ' ' || c == '|' || c == '\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

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
