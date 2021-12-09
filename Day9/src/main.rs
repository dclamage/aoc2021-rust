//use itertools::Itertools;
use std::fs;
use std::time;

struct AdjacentIterator {
    base_i: usize,
    base_j: usize,
    width: usize,
    height: usize,
    adjacent_index: usize,
}

impl AdjacentIterator {
    fn new(base_index: usize, width: usize, height: usize) -> AdjacentIterator {
        AdjacentIterator {
            base_i: base_index / width,
            base_j: base_index % width,
            width,
            height,
            adjacent_index: 0,
        }
    }
}

impl Iterator for AdjacentIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.adjacent_index {
            0 => {
                self.adjacent_index += 1;
                if self.base_i > 0 {
                    Some((self.base_i - 1) * self.width + self.base_j)
                } else {
                    self.next()
                }
            }
            1 => {
                self.adjacent_index += 1;
                if self.base_j > 0 {
                    Some(self.base_i * self.width + self.base_j - 1)
                } else {
                    self.next()
                }
            }
            2 => {
                self.adjacent_index += 1;
                if self.base_i < self.height - 1 {
                    Some((self.base_i + 1) * self.width + self.base_j)
                } else {
                    self.next()
                }
            }
            3 => {
                self.adjacent_index += 1;
                if self.base_j < self.width - 1 {
                    Some(self.base_i * self.width + self.base_j + 1)
                } else {
                    self.next()
                }
            }
            _ => None,
        }
    }
}

fn adj(index: usize, width: usize, height: usize) -> AdjacentIterator {
    AdjacentIterator::new(index, width, height)
}

fn get_low_points(input: &[i32], width: usize) -> Vec<usize> {
    let height = input.len() / width;
    input
        .iter()
        .enumerate()
        .filter(|(i, &v)| adj(*i, width, height).all(|adjacent_index| input[adjacent_index] > v))
        .map(|(i, _)| i)
        .collect()
}

fn part1(input: &[i32], width: usize) -> i32 {
    get_low_points(input, width)
        .iter()
        .map(|&i| input[i] + 1)
        .sum::<i32>()
}

fn part2(input: &[i32], width: usize) -> u64 {
    let low_points = get_low_points(input, width);
    let height = input.len() / width;

    let mut seen_points = vec![false; input.len()];
    let mut basins_sizes: Vec<u64> = Vec::new();
    for low_point in low_points {
        let mut point_queue: Vec<usize> = Vec::new();
        point_queue.push(low_point);
        seen_points[low_point] = true;
        let mut basin_size = 1;

        while point_queue.len() > 0 {
            let i = point_queue.pop().unwrap();
            for adjacent_index in adj(i, width, height) {
                if !seen_points[adjacent_index] && input[adjacent_index] < 9 {
                    point_queue.push(adjacent_index);
                    seen_points[adjacent_index] = true;
                    basin_size += 1;
                }
            }
        }

        basins_sizes.push(basin_size);
    }

    basins_sizes.sort();
    basins_sizes.iter().rev().take(3).product()
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let width = input.chars().position(|c| c == '\r' || c == '\n').unwrap();
    let input: Vec<i32> = input
        .chars()
        .filter(|&c| c >= '0' && c <= '9')
        .map(|c| c as i32 - '0' as i32)
        .collect();

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&input, width);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&input, width);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
