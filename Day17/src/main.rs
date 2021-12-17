use std::fs;
use std::time;
use regex::Regex;

fn part1(target_area: &TargetArea) -> i64 {
    let mut highest_point_reached = 0;
    for dx in 0..target_area.x1 {
        for dy in 0..1000 {
            let mut x = 0;
            let mut y = 0;
            let mut dx = dx;
            let mut dy = dy;
            let mut cur_highest_point_reached = 0;
            while !target_area.is_beyond(x, y) {
                cur_highest_point_reached = i64::max(cur_highest_point_reached, y);
                if target_area.contains(x, y) {
                    highest_point_reached = i64::max(highest_point_reached, cur_highest_point_reached);
                    break;
                }

                x += dx;
                y += dy;
                dx -= i64::signum(dx);
                dy -= 1;
            }
        }
    }

    highest_point_reached
}

fn part2(target_area: &TargetArea) -> usize {
    let mut num_velocities = 0;
    for dx in 0..=target_area.x1 {
        for dy in target_area.y0..1000 {
            let mut x = 0;
            let mut y = 0;
            let mut dx = dx;
            let mut dy = dy;
            while !target_area.is_beyond(x, y) {
                if target_area.contains(x, y) {
                    num_velocities += 1;
                    break;
                }

                x += dx;
                y += dy;
                dx -= i64::signum(dx);
                dy -= 1;
            }
        }
    }

    num_velocities
}

#[derive(Debug, Clone, Copy)]
struct TargetArea {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
}

impl TargetArea {
    fn new(input: &str) -> TargetArea {
       let re = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
       let cap = re.captures(&input).unwrap();
       TargetArea {
            x0: cap[1].parse::<i64>().unwrap(),
            x1: cap[2].parse::<i64>().unwrap(),
            y0: cap[3].parse::<i64>().unwrap(),
            y1: cap[4].parse::<i64>().unwrap(),
        }
    }

    fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.x0 && x <= self.x1 && y >= self.y0 && y <= self.y1
    }

    fn is_beyond(&self, x: i64, y: i64) -> bool {
        x > self.x1 || y < self.y0
    }
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let target_area: TargetArea = TargetArea::new(&input);
    println!("{:?}", target_area);

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&target_area);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&target_area);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
