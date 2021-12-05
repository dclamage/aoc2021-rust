use std::cmp;
use std::fs;
use std::time;

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

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn from_sring(section: &str) -> Point {
        let mut split = section.split(",");
        let x = split.next().unwrap().trim().parse::<i32>().unwrap();
        let y = split.next().unwrap().trim().parse::<i32>().unwrap();
        Point::new(x, y)
    }
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn from_string(line: &str) -> Line {
        let sections = line.split("->").map(|s| s.trim()).collect::<Vec<&str>>();
        if sections.len() != 2 {
            panic!("Invalid line: {}", line);
        }
        let start = Point::from_sring(sections[0]);
        let end = Point::from_sring(sections[1]);
        Line::new(start, end)
    }
}

#[derive(Debug, Clone)]
struct Grid {
    points: Vec<i32>,
    width: i32,
}

impl Grid {
    fn new(width: i32, height: i32) -> Grid {
        Grid {
            points: vec![0; (width * height) as usize],
            width,
        }
    }

    fn new_for(lines: &[Line]) -> Grid {
        let width = lines
            .iter()
            .map(|line| cmp::max(line.start.x, line.end.x))
            .max()
            .unwrap()
            + 1;
        let height = lines
            .iter()
            .map(|line| cmp::max(line.start.y, line.end.y))
            .max()
            .unwrap()
            + 1;
        Grid::new(width, height)
    }

    fn increment(&mut self, x: i32, y: i32) {
        self.points[(y * self.width + x) as usize] += 1;
    }

    fn calc_increment(start: i32, end: i32) -> i32 {
        match end - start {
            0 => 0,
            x if x > 0 => 1,
            _ => -1,
        }
    }

    fn increment_line(&mut self, line: &Line, allow_diagonals: bool) {
        if !allow_diagonals && line.start.x != line.end.x && line.start.y != line.end.y {
            return;
        }

        let mut x = line.start.x;
        let mut y = line.start.y;
        let x_incr = Grid::calc_increment(line.start.x, line.end.x);
        let y_incr = Grid::calc_increment(line.start.y, line.end.y);
        loop {
            self.increment(x, y);
            if x == line.end.x && y == line.end.y {
                break;
            }
            x += x_incr;
            y += y_incr;
        }
    }

    fn count_two_or_higher(&self) -> u32 {
        self.points.iter().filter(|&p| *p >= 2).count() as u32
    }
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<Line> = input.lines().map(|l| Line::from_string(l)).collect();
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
