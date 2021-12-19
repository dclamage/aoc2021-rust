use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::time;

fn part1(omega_scanner: &Scanner) -> usize {
    omega_scanner.points.len()
}

fn part2(scanner_offset_from_zero: &Vec<Point>) -> i64 {
    let mut largest_manhattan_distance = 0;
    for points in scanner_offset_from_zero.iter().combinations(2) {
        let distance = points[0].manhattan_distance(&points[1]);
        if distance > largest_manhattan_distance {
            largest_manhattan_distance = distance;
        }
    }

    largest_manhattan_distance
}

fn create_omega_scanner(scanners: &[Scanner]) -> (Scanner, Vec<Point>) {
    let mut rotated_scanners: Vec<Vec<Scanner>> = Vec::new();
    for scanner in scanners {
        let mut rotated: Vec<Scanner> = Vec::with_capacity(24);
        for r in 0..24 {
            rotated.push(scanner.rotated(r));
        }
        rotated_scanners.push(rotated);
    }

    let mut omega_scanner: Scanner = scanners[0].clone();
    let mut scanners_added: Vec<usize> = Vec::new();
    let mut scanners_properly_rotated: Vec<Scanner> = vec![Scanner::default(); scanners.len()];
    let mut scanner_offset_from_zero: Vec<Point> = vec![Point::new(0, 0, 0); scanners.len()];
    scanners_added.push(0);
    scanners_properly_rotated[0] = scanners[0].clone();

    let mut start_scanners_added_index = 0;
    while scanners_added.len() < scanners.len() {
        let mut found_any_matches = false;
        let new_start_canners_added_index = scanners_added.len();
        for i in 1..scanners.len() {
            if scanners_added.contains(&i) {
                continue;
            }

            for r in 0..24 {
                let cur_scanner = &rotated_scanners[i][r];
                let mut found_match = false;
                for base_scanner_index in scanners_added.iter().skip(start_scanners_added_index) {
                    let base_scanner = &scanners_properly_rotated[*base_scanner_index];
                    let matches = base_scanner.matches(&cur_scanner);
                    if let Some(offset) = matches {
                        let offset_from_zero = scanner_offset_from_zero[*base_scanner_index].sum(&offset);
                        scanner_offset_from_zero[i] = offset_from_zero.clone();
                        scanners_properly_rotated[i] = cur_scanner.clone();

                        omega_scanner = omega_scanner.combine(&cur_scanner, &offset_from_zero);
                        found_match = true;
                        break;
                    }
                }
                if found_match {
                    scanners_added.push(i);
                    found_any_matches = true;
                    break;
                }
            }
        }
        start_scanners_added_index = new_start_canners_added_index;
        if scanners_added.len() < scanners.len() && !found_any_matches {
            panic!("Not all scanners have a match");
        }
    }

    (omega_scanner, scanner_offset_from_zero)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    fn sum(&self, b: &Point) -> Point {
        Point {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }

    fn difference(&self, b: &Point) -> Point {
        Point {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }

    fn manhattan_distance(&self, b: &Point) -> i64 {
        (self.x - b.x).abs() + (self.y - b.y).abs() + (self.z - b.z).abs()
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    points: Vec<Point>,
}

impl Scanner {
    fn new(lines: &[&str]) -> Self {
        let points: Vec<Point> = lines
            .iter()
            .map(|line| {
                let mut parts = line.split(",");
                let x = parts.next().unwrap().parse::<i64>().unwrap();
                let y = parts.next().unwrap().parse::<i64>().unwrap();
                let z = parts.next().unwrap().parse::<i64>().unwrap();
                Point { x, y, z }
            })
            .collect();

        Scanner { points }
    }

    fn default() -> Self {
        Scanner { points: Vec::new() }
    }

    fn rotated(&self, rotate_index: usize) -> Self {
        let mut points: Vec<Point> = Vec::with_capacity(self.points.len());

        for point in &self.points {
            let mut new_point;
            match rotate_index / 4 {
                0 => {
                    new_point = Point::new(point.x, point.y, point.z);
                }
                1 => {
                    new_point = Point::new(-point.x, point.y, -point.z);
                }
                2 => {
                    new_point = Point::new(point.z, point.y, -point.x);
                }
                3 => {
                    new_point = Point::new(-point.z, point.y, point.x);
                }
                4 => {
                    new_point = Point::new(point.x, point.z, -point.y);
                }
                5 => {
                    new_point = Point::new(point.x, -point.z, point.y);
                }
                _ => panic!("Invalid rotate index"),
            }

            match rotate_index % 4 {
                0 => {}
                1 => {
                    new_point = Point::new(-new_point.y, new_point.x, new_point.z);
                }
                2 => {
                    new_point = Point::new(-new_point.x, -new_point.y, new_point.z);
                }
                3 => {
                    new_point = Point::new(new_point.y, -new_point.x, new_point.z);
                }
                _ => panic!("Invalid rotate index"),
            }

            points.push(new_point);
        }

        Scanner { points }
    }

    fn matches(&self, other: &Scanner) -> Option<Point> {
        for point_pair in self.points.iter().cartesian_product(&other.points) {
            let (p1, p2) = point_pair;
            let offset = p1.difference(p2);

            let num_matching = self
                .points
                .iter()
                .filter(|&point| other.points.iter().map(|p| Point::sum(p, &offset)).any(|p| p == *point))
                .count();

            if num_matching >= 12 {
                return Some(offset);
            }
        }

        None
    }

    fn combine(&self, other: &Scanner, offset: &Point) -> Scanner {
        let mut points_set: HashSet<Point> =
            HashSet::with_capacity(self.points.len() + other.points.len());
        points_set.extend(&self.points);

        for point in &other.points {
            points_set.insert(point.sum(&offset));
        }

        let points = points_set.iter().cloned().collect();
        Scanner { points }
    }
}

fn read_scanners(input: &str) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();
    let lines = input.lines().map(|line| line.trim()).collect::<Vec<&str>>();
    let mut cur_line_group: Vec<&str> = Vec::new();
    let mut i = 1;
    while i < lines.len() {
        if lines[i].is_empty() {
            i += 2;
            scanners.push(Scanner::new(&cur_line_group));
            cur_line_group.clear();
            continue;
        }
        cur_line_group.push(lines[i]);
        i += 1;
    }
    if cur_line_group.len() > 0 {
        scanners.push(Scanner::new(&cur_line_group));
    }
    scanners
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let scanners: Vec<Scanner> = read_scanners(&input);
    println!("Read {} scanners", scanners.len());
    let (omega_scanner, scanner_offset_from_zero) = create_omega_scanner(&scanners);

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&omega_scanner);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&scanner_offset_from_zero);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
