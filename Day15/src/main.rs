use std::collections::{HashMap, HashSet};
use std::fs;
use std::time;

type CostMap = HashMap<(usize, usize), usize>;
type VisitedMap = HashSet<(usize, usize)>;

fn get_cost(costs: &CostMap, x: usize, y: usize) -> usize {
    *costs.get(&(x, y)).unwrap_or(&0)
}

struct StackEntry {
    x: usize,
    y: usize,
    cost: usize,
    visited: HashSet<(usize, usize)>,
}

impl StackEntry {
    fn new(x: usize, y: usize, cost: usize, visited: VisitedMap) -> StackEntry {
        StackEntry {
            x,
            y,
            cost,
            visited,
        }
    }
}

fn part1(costs: &CostMap) -> usize {
    let max_x = costs.iter().map(|(k, _)| k.0).max().unwrap();
    let max_y = costs.iter().map(|(k, _)| k.1).max().unwrap();
    let mut stack: Vec<StackEntry> = Vec::new();
    {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        stack.push(StackEntry {
            x: 0,
            y: 0,
            cost: 0,
            visited,
        });
    }
    let mut best_cost: usize = usize::max_value();
    while !stack.is_empty() {
        let entry = stack.pop().unwrap();
        if entry.x == max_x && entry.y == max_y {
            best_cost = entry.cost.min(best_cost);
            continue;
        }
        if entry.cost >= best_cost {
            continue;
        }

        let mut new_visited = entry.visited.clone();
        new_visited.insert((entry.x, entry.y));

        if entry.x < max_x && !entry.visited.contains(&(entry.x + 1, entry.y)) {
            stack.push(StackEntry::new(
                entry.x + 1,
                entry.y,
                entry.cost + get_cost(costs, entry.x + 1, entry.y),
                new_visited.clone(),
            ));
        }
        if entry.y < max_y && !entry.visited.contains(&(entry.x, entry.y + 1)) {
            stack.push(StackEntry::new(
                entry.x,
                entry.y + 1,
                entry.cost + get_cost(costs, entry.x, entry.y + 1),
                new_visited.clone(),
            ));
        }
        if entry.x > 0 && !entry.visited.contains(&(entry.x - 1, entry.y)) {
            stack.push(StackEntry::new(
                entry.x - 1,
                entry.y,
                entry.cost + get_cost(costs, entry.x - 1, entry.y),
                new_visited.clone(),
            ));
        }
        if entry.y > 0 && !entry.visited.contains(&(entry.x, entry.y - 1)) {
            stack.push(StackEntry::new(
                entry.x,
                entry.y - 1,
                entry.cost + get_cost(costs, entry.x, entry.y - 1),
                new_visited.clone(),
            ));
        }
    }

    best_cost
}

fn part2(_input: &CostMap) -> usize {
    0
}

fn parse_costs(input: &str) -> CostMap {
    let mut cost = CostMap::new();
    let mut i = 0usize;
    for line in input.lines() {
        let mut j = 0usize;
        for c in line.chars() {
            cost.insert((i, j), c as usize - '0' as usize);
            j += 1;
        }
        i += 1;
    }
    cost
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    let input = fs::read_to_string("example.txt").expect("Unable to read file");
    //let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input = parse_costs(&input);

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
