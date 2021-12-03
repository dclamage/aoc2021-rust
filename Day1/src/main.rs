use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() {
    println!("**** Part 1 ****");
    if let Ok(lines) = read_lines("input.txt") {
        let mut last_val = -1;
        let mut num_increases = 0;
        for line in lines {
            if let Ok(line) = line {
                let val = line.parse::<i32>().unwrap();
                if last_val != -1 && last_val < val {
                    num_increases += 1;
                }
                last_val = val;
            }
        }
        println!("Number of increases: {}", num_increases);
    }
}

fn part2() {
    println!("**** Part 2 ****");
    if let Ok(lines) = read_lines("input.txt") {
        let mut window: [i32; 3] = [0, 0, 0];
        let mut window_index = 0;
        let mut last_window_sum = -1;
        let mut num_window_increases = 0;
        let mut num_lines_visited = 0;
        for line in lines {
            if let Ok(line) = line {
                let val = line.parse::<i32>().unwrap();
                window[window_index] = val;
                window_index = (window_index + 1) % 3;
                num_lines_visited += 1;
                if num_lines_visited >= 3 {
                    let window_sum = window[0] + window[1] + window[2];
                    if last_window_sum != -1 && last_window_sum < window_sum {
                        num_window_increases += 1;
                    }
                    last_window_sum = window_sum;
                }
            }
        }
        println!("Number of window increases: {}", num_window_increases);
    }
}

fn main() {
    part1();
    part2();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
