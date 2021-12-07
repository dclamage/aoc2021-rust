use std::fs;
use std::time;

type EvalOp = fn(&[i32], i32) -> i32;

fn part1(crab_loc: &[i32]) -> u64 {
    find_best_cost(crab_loc, eval_cost)
}

fn part2(crab_loc: &[i32]) -> u64 {
    find_best_cost(crab_loc, eval_cost_triangle)
}

fn find_best_cost(crab_loc: &[i32], eval_op : EvalOp) -> u64 {
    let (min, max) = min_max_loc(&crab_loc);
    let mut best_cost = i32::max_value();
    for center_loc in min..=max {
        let cost = eval_op(crab_loc, center_loc);
        best_cost = i32::min(best_cost, cost);
    }
    best_cost as u64
}

fn min_max_loc(crab_loc: &[i32]) -> (i32, i32) {
    let mut min = i32::max_value();
    let mut max = i32::min_value();
    for &loc in crab_loc {
        min = i32::min(min, loc);
        max = i32::max(max, loc);
    }
    (min, max)
}

fn eval_cost(crab_loc: &[i32], center_loc: i32) -> i32 {
    let mut cost = 0;
    for loc in crab_loc {
        cost += (*loc - center_loc).abs();
    }
    cost
}

fn eval_cost_triangle(crab_loc: &[i32], center_loc: i32) -> i32 {
    let mut cost = 0;
    for loc in crab_loc {
        let dist = (*loc - center_loc).abs();
        cost += (dist * (dist + 1)) / 2;
    }
    cost
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
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
