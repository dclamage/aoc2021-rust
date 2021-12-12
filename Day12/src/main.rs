use std::collections::HashMap;
use std::fs;
use std::time;

fn part1(rooms: &[Room]) -> u64 {
    let mut stack: Vec<Vec<usize>> = Vec::new();
    let mut num_paths = 0;
    stack.push(vec![0]);
    while stack.len() > 0 {
        let current_path = stack.pop().unwrap();
        let current_room_id = current_path[current_path.len() - 1];
        let current_room = &rooms[current_room_id];
        for next_room_id in current_room.neighbors.iter() {
            if *next_room_id == 0 {
                continue;
            }

            if *next_room_id == 1 {
                num_paths += 1;
                continue;
            }

            let next_room = &rooms[*next_room_id];
            if !next_room.is_small_room || !current_path.contains(next_room_id) {
                let mut new_path = current_path.clone();
                new_path.push(*next_room_id);
                stack.push(new_path);
            }
        }
    }

    num_paths
}

fn has_small_cave_repeat(rooms: &[Room], path: &Vec<usize>) -> bool {
    let num_rooms = rooms.len();
    let mut seen: Vec<bool> = vec![false; num_rooms];
    for room_id in path {
        let room = &rooms[*room_id];
        if room.is_small_room {
            if seen[*room_id] {
                return true;
            }
            seen[*room_id] = true;
        }
    }

    false
}

fn part2(rooms: &[Room]) -> u64 {
    let mut stack: Vec<Vec<usize>> = Vec::new();
    let mut num_paths = 0;
    stack.push(vec![0]);
    while stack.len() > 0 {
        let current_path = stack.pop().unwrap();
        let current_room_id = current_path[current_path.len() - 1];
        let current_room = &rooms[current_room_id];
        for next_room_id in current_room.neighbors.iter() {
            if *next_room_id == 0 {
                continue;
            }

            if *next_room_id == 1 {
                num_paths += 1;
                continue;
            }

            let next_room = &rooms[*next_room_id];
            if !next_room.is_small_room
                || !current_path.contains(next_room_id)
                || !has_small_cave_repeat(rooms, &current_path)
            {
                let mut new_path = current_path.clone();
                new_path.push(*next_room_id);
                stack.push(new_path);
            }
        }
    }

    num_paths
}

fn is_small_room(room_name: &str) -> bool {
    room_name.chars().all(|c| c.is_lowercase())
}

#[derive(Debug)]
struct Room {
    neighbors: Vec<usize>,
    is_small_room: bool,
}

impl Room {
    pub fn new(name: &str) -> Room {
        Room {
            neighbors: Vec::new(),
            is_small_room: is_small_room(&name),
        }
    }
}

fn make_room_vec(input: &[&str]) -> Vec<Room> {
    let mut room_name_to_id: HashMap<&str, usize> = HashMap::new();
    room_name_to_id.insert("start", 0);
    room_name_to_id.insert("end", 1);
    let mut rooms = Vec::new();
    rooms.push(Room::new("start"));
    rooms.push(Room::new("end"));

    for line in input {
        let mut room_names = line.split('-');
        let name0 = room_names.next().unwrap();
        let name1 = room_names.next().unwrap();

        let id0 = if room_name_to_id.contains_key(name0) {
            room_name_to_id[name0]
        } else {
            let id = rooms.len();
            room_name_to_id.insert(name0, id);
            rooms.push(Room::new(name0));
            id
        };

        let id1 = if room_name_to_id.contains_key(name1) {
            room_name_to_id[name1]
        } else {
            let id = rooms.len();
            room_name_to_id.insert(name1, id);
            rooms.push(Room::new(name1));
            id
        };

        rooms[id0].neighbors.push(id1);
        rooms[id1].neighbors.push(id0);
    }

    rooms
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<&str> = input.lines().collect();
    let rooms = make_room_vec(&input);

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&rooms);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&rooms);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
