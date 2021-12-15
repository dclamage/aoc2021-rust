use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::fs;
use std::time;

type CostMap = HashMap<(usize, usize), usize>;

struct AdjacentIterator {
    base_i: usize,
    base_j: usize,
    width: usize,
    height: usize,
    adjacent_index: usize,
}

impl AdjacentIterator {
    fn new(base_i: usize, base_j: usize, width: usize, height: usize) -> AdjacentIterator {
        AdjacentIterator {
            base_i,
            base_j,
            width,
            height,
            adjacent_index: 0,
        }
    }
}

impl Iterator for AdjacentIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.adjacent_index {
            0 => {
                self.adjacent_index += 1;
                if self.base_i > 0 {
                    Some((self.base_i - 1, self.base_j))
                } else {
                    self.next()
                }
            }
            1 => {
                self.adjacent_index += 1;
                if self.base_j > 0 {
                    Some((self.base_i,self.base_j - 1))
                } else {
                    self.next()
                }
            }
            2 => {
                self.adjacent_index += 1;
                if self.base_i < self.height - 1 {
                    Some((self.base_i + 1, self.base_j))
                } else {
                    self.next()
                }
            }
            3 => {
                self.adjacent_index += 1;
                if self.base_j < self.width - 1 {
                    Some((self.base_i, self.base_j + 1))
                } else {
                    self.next()
                }
            }
            _ => None,
        }
    }
}

fn adj(i: usize, j: usize, width: usize, height: usize) -> AdjacentIterator {
    AdjacentIterator::new(i, j, width, height)
}

fn get_cost(costs: &CostMap, p: &(usize, usize)) -> usize {
    *costs.get(&p).unwrap_or(&0)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodeCost {
    x: usize,
    y: usize,
    cost: usize,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_cost(costs: &CostMap) -> usize {
    let max_x = costs.iter().map(|(k, _)| k.0).max().unwrap();
    let max_y = costs.iter().map(|(k, _)| k.1).max().unwrap();

    let mut best_costs: HashMap<(usize, usize), usize> = HashMap::new();

    let mut heap = BinaryHeap::new();
    heap.push(NodeCost { x: 0, y: 0, cost: 0 });

    while let Some(NodeCost { x, y, cost }) = heap.pop() {
        if cost > *best_costs.get(&(x, y)).unwrap_or(&usize::max_value()) {
            continue;
        }

        for adj_pos in adj(x, y, max_x + 1, max_y + 1) {
            let next = NodeCost { x: adj_pos.0, y: adj_pos.1, cost: cost + get_cost(costs, &adj_pos) };
            if next.cost < *best_costs.get(&adj_pos).unwrap_or(&usize::max_value()) {
                heap.push(next);
                best_costs.insert(adj_pos, next.cost);
            }
        }
    }

    *best_costs.get(&(max_x, max_y)).unwrap()
}

fn part1(costs: &CostMap) -> usize {
    find_shortest_cost(costs)
}

fn part2(costs: &CostMap) -> usize {
    let size_x = costs.iter().map(|(k, _)| k.0).max().unwrap() + 1;
    let size_y = costs.iter().map(|(k, _)| k.1).max().unwrap() + 1;

    let orig_costs: Vec<((usize, usize), usize)> = costs.iter().map(|(k, v)| (*k, *v)).collect();
    let mut costs = costs.clone();
    for (p, cost) in orig_costs {
        for i in 0..5 {
            for j in 0..5 {
                if i == 0 && j == 0 {
                    continue;
                }

                let dist = i + j;
                let new_cost = (((cost - 1) + dist) % 9) + 1;
                costs.insert((p.0 + i * size_x, p.1 + j * size_y), new_cost);
            }
        }
    }

    find_shortest_cost(&costs)
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
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
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
