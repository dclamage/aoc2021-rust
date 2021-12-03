use std::fs;
use std::io::Write;
use std::time;

fn part1(values: &[usize]) -> usize {
    let mut num_increases = 0;
    for i in 1..values.len() {
        if values[i] > values[i - 1] {
            num_increases += 1;
        }
    }
    num_increases
}

// a0 + a1 + a2 < a1 + a2 + a3
// a0 < a3
fn part2(values: &[usize]) -> usize {
    let mut num_increases = 0;
    for i in 3..values.len() {
        if values[i] > values[i - 3] {
            num_increases += 1;
        }
    }
    num_increases
}

fn main() {
    let total_time_start = time::Instant::now();
    let stdout = std::io::stdout();
    let mut stdout_handle = stdout.lock();
    writeln!(stdout_handle, "AoC 2021 Day 1").unwrap();
    writeln!(stdout_handle, "----------------------------------\n").unwrap();

    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let read_time = read_time_start.elapsed();
    let convert_time_start = time::Instant::now();
    let lines = input
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let convert_time = convert_time_start.elapsed();

    let part1_time_start = time::Instant::now();
    let part1_answer = part1(&lines);
    let part1_time = part1_time_start.elapsed();
    writeln!(stdout_handle, "Part 1 Answer: {}", part1_answer).unwrap();

    writeln!(stdout_handle, "\n----------------------------------\n").unwrap();

    let part2_time = time::Instant::now();
    let part2_answer = part2(&lines);
    let part2_time = part2_time.elapsed();
    writeln!(stdout_handle, "Part 2 Answer: {}", part2_answer).unwrap();

    writeln!(stdout_handle, "\n----------------------------------\n").unwrap();

    writeln!(stdout_handle, "Read time: {:?}", read_time).unwrap();
    writeln!(stdout_handle, "Convert time: {:?}", convert_time).unwrap();
    writeln!(stdout_handle, "Part 1 time: {:?}", part1_time).unwrap();
    writeln!(stdout_handle, "Part 2 time: {:?}", part2_time).unwrap();

    let total_time = total_time_start.elapsed();
    writeln!(stdout_handle, "Total time: {:?}", total_time).unwrap();
}
