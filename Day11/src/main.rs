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
            4 => {
                self.adjacent_index += 1;
                if self.base_i > 0 && self.base_j > 0 {
                    Some((self.base_i - 1) * self.width + self.base_j - 1)
                } else {
                    self.next()
                }
            }
            5 => {
                self.adjacent_index += 1;
                if self.base_i > 0 && self.base_j < self.width - 1 {
                    Some((self.base_i - 1) * self.width + self.base_j + 1)
                } else {
                    self.next()
                }
            }
            6 => {
                self.adjacent_index += 1;
                if self.base_i < self.height - 1 && self.base_j > 0 {
                    Some((self.base_i + 1) * self.width + self.base_j - 1)
                } else {
                    self.next()
                }
            }
            7 => {
                self.adjacent_index += 1;
                if self.base_i < self.height - 1 && self.base_j < self.width - 1 {
                    Some((self.base_i + 1) * self.width + self.base_j + 1)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn adj(index: usize, width: usize, height: usize) -> AdjacentIterator {
    AdjacentIterator::new(index, width, height)
}

fn part1(input: &[u8], width: usize, height: usize) -> u64 {
    let mut input = input.to_vec();
    let mut total_flashes = 0;
    for _ in 0..100 {
        for i in 0..input.len() {
            input[i] += 1;
        }

        let mut flashed = vec![false; input.len()];
        let mut had_flash;
        loop {
            had_flash = false;
            for i in 0..input.len() {
                if flashed[i] || input[i] <= 9 {
                    continue;
                }

                for j in adj(i, width, height) {
                    input[j] += 1;
                }
                flashed[i] = true;
                had_flash = true;
                total_flashes += 1;
            }

            if !had_flash {
                break;
            }
        }

        for i in 0..input.len() {
            if flashed[i] {
                input[i] = 0;
            }
        }
    }

    total_flashes
}

fn part2(input: &[u8], width: usize, height: usize) -> u64 {
    let mut input = input.to_vec();
    let mut step_num = 0;
    loop {
        for i in 0..input.len() {
            input[i] += 1;
        }

        let mut flashed = vec![false; input.len()];
        let mut had_flash;
        loop {
            had_flash = false;
            for i in 0..input.len() {
                if flashed[i] || input[i] <= 9 {
                    continue;
                }

                for j in adj(i, width, height) {
                    input[j] += 1;
                }
                flashed[i] = true;
                had_flash = true;
            }

            if !had_flash {
                break;
            }
        }

        for i in 0..input.len() {
            if flashed[i] {
                input[i] = 0;
            }
        }

        step_num += 1;
        if flashed.iter().all(|x| *x) {
            break;
        }
    }

    step_num
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<&str> = input.lines().collect();
    let width = input[0].len();
    let height = input.len();
    let input: Vec<u8> = {
        let mut input_vals = Vec::new();
        for line in input {
            for c in line.chars() {
                input_vals.push(c as u8 - '0' as u8);
            }
        }
        input_vals
    };

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&input, width, height);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&input, width, height);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
