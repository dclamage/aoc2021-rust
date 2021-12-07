use std::fs;
use std::time;

fn part1(crab_loc: &[i32]) -> u64 {
    let (min, max) = min_max_loc(&crab_loc);
    let mut center_loc = min;
    let mut best_cost = i32::max_value();
    while center_loc != max {
        let cost = eval_cost(crab_loc, center_loc);
        if cost < best_cost {
            best_cost = cost;
        }
        center_loc += 1;
    }
    best_cost as u64
}

fn part2(crab_loc: &[i32]) -> u64 {
    let (min, max) = min_max_loc(&crab_loc);
    let mut center_loc = min;
    let mut best_cost = i32::max_value();
    while center_loc != max {
        let cost = eval_cost2(crab_loc, center_loc);
        if cost < best_cost {
            best_cost = cost;
        }
        center_loc += 1;
    }
    best_cost as u64
}

fn min_max_loc(crab_loc: &[i32]) -> (i32, i32) {
    let mut min = i32::max_value();
    let mut max = i32::min_value();
    for loc in crab_loc {
        if *loc < min {
            min = *loc;
        }
        if *loc > max {
            max = *loc;
        }
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

fn eval_cost2(crab_loc: &[i32], center_loc: i32) -> i32 {
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
