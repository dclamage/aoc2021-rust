use std::fs;
use std::time;

fn part1(initial_fish: &[i32]) -> u64 {
    let mut num_fish_per_day = init_fish_per_day(initial_fish);

    for _ in 0..80 {
        num_fish_per_day = increment_day(&num_fish_per_day);
    }

    num_fish_per_day.iter().sum()
}

fn part2(initial_fish: &[i32]) -> u64 {
    let mut num_fish_per_day = init_fish_per_day(initial_fish);

    for _ in 0..256 {
        num_fish_per_day = increment_day(&num_fish_per_day);
    }

    num_fish_per_day.iter().sum()
}

fn init_fish_per_day(initial_fish: &[i32]) -> Vec<u64> {
    let mut num_fish_per_day = vec![0u64; 9];
    for fish in initial_fish {
        num_fish_per_day[*fish as usize] += 1;
    }
    num_fish_per_day
}

fn increment_day(num_fish_per_day : &[u64]) -> Vec<u64> {
    let mut new_num_fish_per_day = vec![0u64; 9];
    for day in 0..9 {
        let cur_num_fish = num_fish_per_day[day];
        if day == 0 {
            new_num_fish_per_day[6] = cur_num_fish;
            new_num_fish_per_day[8] = cur_num_fish;
        } else {
            new_num_fish_per_day[day - 1] += cur_num_fish;
        }
    }
    new_num_fish_per_day
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let initial_fish: Vec<i32> = input.split(',').map(|s| s.trim().parse().unwrap()).collect();
    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&initial_fish);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&initial_fish);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
