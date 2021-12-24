use regex::Regex;
use std::fs;
use std::time;

fn part1(div_z: &[i64], offsets_x: &[i64], offsets_y: &[i64]) -> i64 {
    find_serial(div_z, offsets_x, offsets_y, false)
}

fn part2(div_z: &[i64], offsets_x: &[i64], offsets_y: &[i64]) -> i64 {
    find_serial(div_z, offsets_x, offsets_y, true)
}

fn find_serial(div_z: &[i64], offsets_x: &[i64], offsets_y: &[i64], increment: bool) -> i64 {
    let mut serial = first_serial(div_z, offsets_x, offsets_y, increment);
    loop {
        let result = simulate_program(&serial, div_z, offsets_x, offsets_y);
        if result == 0 {
            return serial.iter().map(|x| (*x as u8 + '0' as u8) as char).collect::<String>().parse::<i64>().unwrap();
        }

        let new_serial = next_serial(&serial, div_z, offsets_x, offsets_y, increment);
        if new_serial.is_none() {
            break;
        }
        serial = new_serial.unwrap();
    }

    0
}

fn first_serial(div_z: &[i64], offsets_x: &[i64], offsets_y: &[i64], increment: bool) -> [i64; 14] {
    let mut serial = [0i64; 14];
    let default_digit = if increment { 1 } else { 9 };
    serial[0] = default_digit;
    let mut z = serial[0] + offsets_y[0];
    for i in 1..14 {
        serial[i] = default_digit;

        let z_div = div_z[i];
        let x_offset = offsets_x[i];
        let y_offset = offsets_y[i];

        let check = (z % 26) + x_offset;
        z /= z_div;

        if check >= 1 && check <= 9 {
            serial[i] = check;
        } else {
            z = (z * 26) + serial[i] + y_offset;
        }
    }
    serial
}

fn next_serial(serial: &[i64], div_z: &[i64], offsets_x: &[i64], offsets_y: &[i64], increment: bool) -> Option<[i64; 14]> {
    let mut next_serial = [0i64; 14];
    
    let mut start_change_index = 0;
    {
        let mut z: i64 = 0;
        for i in 0..14 {
            let inp = serial[i];
            let z_div = div_z[i];
            let x_offset = offsets_x[i];
            let y_offset = offsets_y[i];

            let check = (z % 26) + x_offset;
            z /= z_div;
            if check != inp {
                if div_z[i] == 26 {
                    start_change_index = i - 1;
                    break;
                }

                z = (z * 26) + inp + y_offset;
            }
        }
    }

    for change_index in (0..=start_change_index).rev() {
        if increment && serial[change_index] == 9 || !increment && serial[change_index] == 1 {
            continue;
        }

        let mut z: i64 = 0;
        for i in 0..change_index {
            let inp = serial[i];
            next_serial[i] = inp;

            let z_div = div_z[i];
            let x_offset = offsets_x[i];
            let y_offset = offsets_y[i];
            
            let check = (z % 26) + x_offset;
            z /= z_div;

            if check != inp {
                z = (z * 26) + inp + y_offset;
            }
        }

        if increment {
            next_serial[change_index] = serial[change_index] + 1;
        } else {
            next_serial[change_index] = serial[change_index] - 1;
        }

        let check = (z % 26) + offsets_x[change_index];
        z /= div_z[change_index];

        if check != next_serial[change_index] {
            z = (z * 26) + next_serial[change_index] + offsets_y[change_index];
        }

        for i in change_index + 1..14 {
            let z_div = div_z[i];
            let x_offset = offsets_x[i];
            let y_offset = offsets_y[i];
            
            let check = (z % 26) + x_offset;
            z /= z_div;

            if check >= 1 && check <= 9 {
                next_serial[i] = check;
            } else {
                next_serial[i] = if increment { 1 } else { 9 };
                z = (z * 26) + next_serial[i] + y_offset;
            }
        }
        return Some(next_serial)
    }

    None
}

fn simulate_program(serial: &[i64], div_z: &[i64], offsets_x: &[i64], offsets_y: &[i64]) -> i64 {
    let mut z: i64 = 0;
    for i in 0..14 {
        let inp = serial[i];
        let z_div = div_z[i];
        let x_offset = offsets_x[i];
        let y_offset = offsets_y[i];

        let check = (z % 26) + x_offset;
        z /= z_div;
        if check != inp {
            z = (z * 26) + inp + y_offset;
        }
    }
    z
}

fn parse_input(input: &str) -> [i64; 42] {
    let re = Regex::new(r"... [xyz] (-?\d+)").unwrap();

    let mut result = [0; 42];
    let mut cur_line: usize = 0;
    for line in input.lines() {
        let segment = cur_line / 18;
        let offset = cur_line % 18;
        let caps = re.captures(line);
        if let Some(caps) = caps {
            if offset == 4 {
                result[segment] = caps[1].parse::<i64>().unwrap();
            } else if offset == 5 {
                result[segment + 14] = caps[1].parse::<i64>().unwrap();
            } else if offset == 15 {
                result[segment + 28] = caps[1].parse::<i64>().unwrap();
            }
        }

        cur_line += 1;
    }
    result
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let offsets = parse_input(&input);
    let div_z = &offsets[0..14];
    let offsets_x = &offsets[14..28];
    let offsets_y = &offsets[28..42];

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(div_z, offsets_x, offsets_y);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(div_z, offsets_x, offsets_y);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
