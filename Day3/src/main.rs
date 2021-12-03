use std::fs;
use std::time;

fn part1(lines : &Vec<&str>) {
    println!("Part 1:");

    let mut bit_counts : Vec<usize> = Vec::new();

    // Set the size of bit_counts to the length of the first line in the file
    bit_counts.resize(lines[0].len(), 0);

    // Loop through all lines and count when bits are set
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                bit_counts[i] += 1;
            }
        }
    }

    let majority_count = lines.len() / 2;
    let mut gamma_rate : usize = 0;
    let mut epsilon_rate : usize = 0;
    for count in bit_counts {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if count > majority_count {
            gamma_rate |= 1;
        } else {
            epsilon_rate |= 1;
        }
    }

    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);

    // Answer is gamme multiplied by epsilon
    println!("Answer: {}", gamma_rate * epsilon_rate);
}

fn bit_counts(lines : &Vec<&str>, pos: usize) -> (usize, usize) {
    let mut set_count : usize = 0;
    let mut unset_count : usize = 0;
    for line in lines {
        let c = line.chars().nth(pos).unwrap();
        if c == '1' {
            set_count += 1;
        } else {
            unset_count += 1;
        }
    }
    return (set_count, unset_count);
}

fn filter_lines<'a>(lines : &Vec<&'a str>, bit_pos : usize, keep_most_common : bool) -> Vec<&'a str> {
    let (set_count, unset_count) = bit_counts(lines, bit_pos);
    let keep_char : char = if keep_most_common && set_count >= unset_count || !keep_most_common && set_count < unset_count { '1' } else { '0' };

    let mut filtered_lines : Vec<&str> = Vec::new();
    for line in lines {
        if line.chars().nth(bit_pos).unwrap() == keep_char {
            filtered_lines.push(line);
        }
    }
    return filtered_lines;
}

fn run_filter_helper<'a>(lines : &Vec<&'a str>, bit_pos : usize, keep_most_common : bool) -> &'a str {
    let filtered_lines = filter_lines(lines, bit_pos, keep_most_common);
    if filtered_lines.len() == 1 {
        return filtered_lines[0];
    }
    return run_filter_helper(&filtered_lines, bit_pos + 1, keep_most_common);
}

fn run_filter<'a>(lines : &Vec<&'a str>, keep_most_common : bool) -> &'a str {
    return run_filter_helper(lines, 0, keep_most_common);
}

fn part2(lines : &Vec<&str>) {
    println!("Part 2:");

    let oxygen_line = run_filter(lines, true);
    let co2_line = run_filter(lines, false);

    let oxygen_rating : usize = usize::from_str_radix(oxygen_line, 2).unwrap();
    let co2_rating : usize = usize::from_str_radix(co2_line, 2).unwrap();

    println!("Oxygen rating: {}", oxygen_rating);
    println!("CO2 rating: {}", co2_rating);

    // Answer is the product of the ratings
    println!("Answer: {}", oxygen_rating * co2_rating);
}

fn main() {
    // Read all lines of input.txt
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let read_time_start = time::Instant::now();
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<&str> = input.lines().collect();
    let read_time = read_time_start.elapsed();

    let part1_time_start = time::Instant::now();
    part1(&lines);
    let part1_time = part1_time_start.elapsed();

    println!("");
    println!("----------------------------------");
    println!("");

    let part2_time = time::Instant::now();
    part2(&lines);
    let part2_time = part2_time.elapsed();

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
