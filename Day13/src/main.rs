use std::fs;
use std::time;

fn part1(initial_coords: &[Coord], instructions: &[Instruction]) -> usize {
    let new_coords = fold_along(initial_coords, instructions[0]);
    new_coords.len()
}

fn part2(initial_coords: &[Coord], instructions: &[Instruction]) -> usize {
    let mut new_coords = initial_coords.to_vec();
    for instruction in instructions {
        new_coords = fold_along(&new_coords, *instruction);
    }

    print_coords(&new_coords);
    0
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    pub dir: char,
    pub pos: i64,
}

impl Instruction {
    fn new(dir: char, pos: i64) -> Self {
        Instruction { dir, pos }
    }
}

fn fill_initial_coords(lines: &[&str]) -> Vec<Coord> {
    let mut coords = Vec::new();
    for line in lines {
        if lines.len() == 0 || !line.contains(',') {
            break;
        }
        let mut split = line.split(',');
        let x = split.next().unwrap().trim().parse::<i64>().unwrap();
        let y = split.next().unwrap().trim().parse::<i64>().unwrap();
        coords.push(Coord::new(x, y));
    }
    coords
}

fn get_fold_instructions(lines: &[&str]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        if line.starts_with("fold along ") {
            let mut split = line.split('=');
            let dir = split.next().unwrap().trim().chars().last().unwrap();
            let pos = split.next().unwrap().trim().parse::<i64>().unwrap();
            instructions.push(Instruction::new(dir, pos));
        }
    }
    instructions
}

fn fold_along(coords: &[Coord], instruction: Instruction) -> Vec<Coord> {
    let mut new_coords = Vec::new();
    for coord in coords {
        match instruction.dir {
            'x' => {
                if coord.x > instruction.pos {
                    new_coords.push(Coord::new(2 * instruction.pos - coord.x, coord.y));
                } else {
                    new_coords.push(coord.clone());
                }
            }
            'y' => {
                if coord.y > instruction.pos {
                    new_coords.push(Coord::new(coord.x, 2 * instruction.pos - coord.y));
                } else {
                    new_coords.push(coord.clone());
                }
            }
            _ => panic!("Unknown direction {}", instruction.dir),
        }
    }

    new_coords.sort();
    new_coords.dedup();

    new_coords
}

fn print_coords(coords: &[Coord]) {
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;
    for coord in coords {
        min_x = std::cmp::min(min_x, coord.x);
        max_x = std::cmp::max(max_x, coord.x);
        min_y = std::cmp::min(min_y, coord.y);
        max_y = std::cmp::max(max_y, coord.y);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let coord = Coord::new(x, y);
            if coords.binary_search(&coord).is_ok() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<&str> = input.lines().collect();
    let initial_coords = fill_initial_coords(&input);
    let instructions = get_fold_instructions(&input);

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&initial_coords, &instructions);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&initial_coords, &instructions);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
