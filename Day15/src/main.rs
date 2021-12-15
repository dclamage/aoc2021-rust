use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    prev_x: usize,
    prev_y: usize,
    cost: usize,
}

impl Node {
    fn new(x: usize, y: usize) -> Node {
        Node {
            x,
            y,
            prev_x: usize::max_value(),
            prev_y: usize::max_value(),
            cost: usize::max_value(),
        }
    }
}

fn part1(costs: &CostMap) -> usize {
    let max_x = costs.iter().map(|(k, _)| k.0).max().unwrap();
    let max_y = costs.iter().map(|(k, _)| k.1).max().unwrap();
    let mut nodes: HashMap<(usize, usize), Node> = HashMap::new();
    for p in costs.keys() {
        nodes.insert(*p, Node::new(p.0, p.1));
    }
    nodes.get_mut(&(0, 0)).unwrap().cost = 0;
    
    let mut unvisited_nodes: Vec<(usize, usize)> = nodes.keys().map(|k| *k).collect();
    while !unvisited_nodes.is_empty() {
        let (node_index, node_pos) = unvisited_nodes.iter().enumerate().min_by_key(|(_, p)| {
            nodes.get(p).unwrap().cost
        }).unwrap();
        let node_pos = node_pos.clone();
        unvisited_nodes.remove(node_index);

        let node = nodes.get(&node_pos).unwrap().clone();

        for adj_pos in adj(node.x, node.y, max_x + 1, max_y + 1) {
            if unvisited_nodes.contains(&adj_pos) {
                let new_cost = node.cost + get_cost(costs, &adj_pos);
                let next_node = nodes.get_mut(&adj_pos).unwrap();
                if new_cost < next_node.cost {
                    next_node.cost = new_cost;
                    next_node.prev_x = node.x;
                    next_node.prev_y = node.y;
                }
            }
        }
    }

    nodes.get(&(max_x, max_y)).unwrap().cost
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
