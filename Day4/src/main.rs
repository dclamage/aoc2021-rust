use std::fs;
use std::time;

fn part1(_lines : &Vec<&str>) {
    println!("Part 1:");
}

fn part2(_lines : &Vec<&str>) {
    println!("Part 2:");
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
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
