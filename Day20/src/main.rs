//use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs;
use std::time;

type Image = BTreeSet<(i64, i64)>;

fn part1(decoder: &Vec<bool>, input_image: &Image) -> usize {
    let mut image = input_image.clone();
    for i in 0..2 {
        image = step_image(&image, decoder, i + 1);
    }

    image.len()
}

fn part2(decoder: &Vec<bool>, input_image: &Image) -> usize {
    let mut image = input_image.clone();
    for i in 0..50 {
        image = step_image(&image, decoder, i + 1);
    }

    image.len()
}

fn get_minmax(input_image: &Image) -> (i64, i64, i64, i64) {
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;
    for point in input_image.iter() {
        if point.0 < min_x {
            min_x = point.0;
        }
        if point.0 > max_x {
            max_x = point.0;
        }
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    (min_x, max_x, min_y, max_y)
}

#[allow(dead_code)]
fn print_image(image: &Image) {
    let (min_x, max_x, min_y, max_y) = get_minmax(image);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if image.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn step_image(image: &Image, decoder: &Vec<bool>, step_num: usize) -> Image {
    let (min_x, max_x, min_y, max_y) = get_minmax(image);

    const RADIUS: i64 = 1;
    let mut output_image: Image = Image::new();
    let start_x = min_x - RADIUS;
    let end_x = max_x + RADIUS;
    let start_y = min_y - RADIUS;
    let end_y = max_y + RADIUS;
    for x in start_x..=end_x {
        for y in start_y..=end_y {
            let mut decoder_index: usize = 0;
            for y1 in (y - 1)..=(y + 1) {
                for x1 in (x - 1)..=(x + 1) {
                    decoder_index <<= 1;
                    if image.contains(&(x1, y1)) || (decoder[0] && (step_num % 2 == 0) && (x1 < min_x || x1 > max_x || y1 < min_y || y1 > max_y)) {
                        decoder_index |= 1;
                    }
                }
            }
            if decoder[decoder_index] {
                output_image.insert((x, y));
            }
        }
    }

    output_image
}

fn parse_input(input: &str) -> (Vec<bool>, Image) {
    let decoder_line = input.split_once('\n').unwrap().0;
    let input_lines: Vec<&str> = input.lines().skip(2).collect();

    let decoder: Vec<bool> = decoder_line.chars().map(|c| c == '#').collect();
    let mut image: Image = Image::new();
    let mut y = 0;
    for line in input_lines {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                image.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }

    (decoder, image)
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let (decoder, image) = parse_input(&input);

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&decoder, &image);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&decoder, &image);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
