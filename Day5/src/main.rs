pub mod grid;
pub mod line;
pub mod point;

use std::fs;
use std::time;
use grid::Grid;
use line::Line;

fn part1(lines: &[Line]) -> u32 {
    let mut grid = Grid::new_for(lines);
    for line in lines {
        grid.increment_line(line, false);
    }

    grid.count_two_or_higher()
}

fn part2(lines: &[Line]) -> u32 {
    let mut grid = Grid::new_for(lines);
    for line in lines {
        grid.increment_line(line, true);
    }

    grid.count_two_or_higher()
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines : Vec<Line> = input.lines().map(|l| Line::from_string(l)).collect();
    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&lines);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&lines);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
