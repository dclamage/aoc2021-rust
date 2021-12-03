use std::fs;

fn part1(lines : &Vec<String>) {
    // Print part 1 header
    println!("Part 1:");

    let mut pos = 0;
    let mut depth = 0;

    // Loop through all lines
    for line in lines.iter() {
        // Split line by whitespace
        let mut words: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        if words.len() != 2 {
            continue;
        }

        // Get the direction and the amount
        let direction = words.remove(0);
        let amount = words.remove(0).parse::<i32>().unwrap();
        match direction.as_ref() {
            "forward" => pos += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!("Invalid direction"),
        };
    }

    // Print the final position and depth
    println!("Final position: {}", pos);
    println!("Final depth: {}", depth);

    // The answer is the position multiplied by the depth
    println!("Answer: {}", pos * depth);
}

fn part2(lines : &Vec<String>) {
    // Print part 2 header
    println!("Part 2:");

    let mut aim = 0;
    let mut pos = 0;
    let mut depth = 0;

    // Loop through all lines
    for line in lines.iter() {
        // Split line by whitespace
        let mut words: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        if words.len() != 2 {
            continue;
        }

        // Get the direction and the amount
        let direction = words.remove(0);
        let amount = words.remove(0).parse::<i32>().unwrap();
        match direction.as_ref() {
            "forward" => { pos += amount; depth += amount * aim; },
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!("Invalid direction"),
        };
    }

    // Print the final position and depth
    println!("Final position: {}", pos);
    println!("Final depth: {}", depth);

    // The answer is the position multiplied by the depth
    println!("Answer: {}", pos * depth);
}

fn main() {
    // Read all lines of input.txt
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    part1(&lines);
    part2(&lines);
}