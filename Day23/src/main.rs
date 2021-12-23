#[allow(unused)]
use itertools::Itertools;
#[allow(unused)]
use regex::Regex;
use std::fs;
use std::time;

struct StepInfo {
    map: Map,
    moves: Vec<Move>,
    cost: i64,
}

fn part1(input_map: &Map) -> i64 {
    input_map.lowest_cost()
}

fn part2(input_map: &Map) -> i64 {
    let unfolded_map = input_map.unfolded();
    unfolded_map.print();

    unfolded_map.lowest_cost()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    from_location: usize,
    from_index: usize,
    to_location: usize,
    to_index: usize,
    cost: i64,
}

impl Move {
    fn new(
        from_location: usize,
        from_index: usize,
        to_location: usize,
        to_index: usize,
        piece: usize,
    ) -> Move {
        let mut m = Move {
            from_location,
            from_index,
            to_location,
            to_index,
            cost: 0,
        };

        m.cost = m.cost(piece);
        m
    }

    fn dist(&self) -> i64 {
        let mut dist: i64 = 0;
        if self.from_location < OPEN_SPACE_INDEX {
            dist += 1;
            if self.from_index == 1 {
                dist += 1;
            }
        }
        if self.to_location < OPEN_SPACE_INDEX {
            dist += 1;
            if self.to_index == 1 {
                dist += 1;
            }
        }

        let from_open_index = if self.from_location < OPEN_SPACE_INDEX {
            ROOM_ENTRANCES[self.from_location]
        } else {
            self.from_index
        };
        let to_open_index = if self.to_location < OPEN_SPACE_INDEX {
            ROOM_ENTRANCES[self.to_location]
        } else {
            self.to_index
        };

        dist += (to_open_index as i64 - from_open_index as i64).abs();

        dist
    }

    fn cost(&self, piece: usize) -> i64 {
        self.dist() * ENERGY_COSTS[piece - 1]
    }
}

#[derive(Debug, Clone)]
struct Map {
    open_space: [usize; 11],
    rooms: Vec<Vec<usize>>,
}

impl Map {
    fn new() -> Map {
        Map {
            open_space: [0; 11],
            rooms: vec![vec![0; 2]; 4],
        }
    }

    fn parse(input: &str) -> Map {
        let mut map = Map::new();

        let line = input.lines().nth(2).unwrap();
        let re = Regex::new(r"#+([A-D])#([A-D])#([A-D])#([A-D])#+").unwrap();
        let caps = re.captures(line).unwrap();
        map.rooms[0][0] = caps[1].parse::<char>().unwrap() as usize - 'A' as usize + 1;
        map.rooms[1][0] = caps[2].parse::<char>().unwrap() as usize - 'A' as usize + 1;
        map.rooms[2][0] = caps[3].parse::<char>().unwrap() as usize - 'A' as usize + 1;
        map.rooms[3][0] = caps[4].parse::<char>().unwrap() as usize - 'A' as usize + 1;

        let line = input.lines().nth(3).unwrap();
        let caps = re.captures(line).unwrap();
        map.rooms[0][1] = caps[1].parse::<char>().unwrap() as usize - 'A' as usize + 1;
        map.rooms[1][1] = caps[2].parse::<char>().unwrap() as usize - 'A' as usize + 1;
        map.rooms[2][1] = caps[3].parse::<char>().unwrap() as usize - 'A' as usize + 1;
        map.rooms[3][1] = caps[4].parse::<char>().unwrap() as usize - 'A' as usize + 1;

        map
    }

    // #D#C#B#A#
    // #D#B#A#C#
    fn unfolded(&self) -> Map {
        let mut map = Map::new();
        map.open_space = self.open_space.clone();
        map.rooms = Vec::new();
        map.rooms.push(vec![self.rooms[0][0], 4, 4, self.rooms[0][1]]);
        map.rooms.push(vec![self.rooms[1][0], 3, 2, self.rooms[1][1]]);
        map.rooms.push(vec![self.rooms[2][0], 2, 1, self.rooms[2][1]]);
        map.rooms.push(vec![self.rooms[3][0], 1, 3, self.rooms[3][1]]);

        map
    }

    fn lowest_cost(&self) -> i64 {
        let mut stack: Vec<StepInfo> = Vec::new();
        stack.push(StepInfo {
            map: self.clone(),
            moves: self.get_moves(),
            cost: 0,
        });

        let mut lowest_cost = std::i64::MAX;
        while stack.len() > 0 {
            let top_index = stack.len() - 1;
            let step = &mut stack[top_index];
            if step.cost > lowest_cost || step.moves.len() == 0 {
                stack.pop();
                continue;
            }

            let next_move = step.moves.pop().unwrap();
            let next_cost = step.cost + next_move.cost;
            if next_cost > lowest_cost {
                continue;
            }
            let next_map = step.map.execute_move(&next_move);
            if next_map.is_complete() {
                if lowest_cost > next_cost {
                    lowest_cost = next_cost;
                }
                continue;
            }

            let next_moves = next_map.get_moves();
            if next_moves.len() > 0 {
                stack.push(StepInfo {
                    map: next_map,
                    moves: next_moves,
                    cost: next_cost,
                });
            }
        }

        lowest_cost
    }

    fn get_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for i in 0..11 {
            let piece: usize = self.open_space[i];
            if piece == 0 {
                continue;
            }
            let home_index = piece - 1;
            if self.rooms[home_index][0] == 0 && self.rooms[home_index][1] == 0 {
                let m = Move::new(OPEN_SPACE_INDEX, i, home_index, 1, piece);
                if self.is_move_valid(&m) {
                    moves.push(m);
                    return moves;
                }
            }

            if self.rooms[home_index][0] == 0 && self.rooms[home_index][1] == piece {
                let m = Move::new(OPEN_SPACE_INDEX, i, home_index, 0, piece);
                if self.is_move_valid(&m) {
                    moves.push(m);
                    return moves;
                }
            }
        }

        for room_index in 0..self.rooms.len() {
            let piece0 = self.rooms[room_index][0];
            let piece1 = self.rooms[room_index][1];
            let correct_piece = room_index + 1;
            if (piece0 == 0 || piece0 == correct_piece) && (piece1 == 0 || piece1 == correct_piece)
            {
                continue;
            }
            if piece0 != 0 && self.can_go_home(piece0) {
                let home_index = piece0 - 1;
                let home_slot = if self.rooms[home_index][1] == 0 { 1 } else { 0 };
                let m = Move::new(room_index, 0, home_index, home_slot, piece0);
                if self.is_move_valid(&m) {
                    moves.push(m);
                    return moves;
                }
            }
            if piece0 == 0 && piece1 != 0 && self.can_go_home(piece1) {
                let home_index = piece1 - 1;
                let home_slot = if self.rooms[home_index][1] == 0 { 1 } else { 0 };
                let m = Move::new(room_index, 1, home_index, home_slot, piece1);
                if self.is_move_valid(&m) {
                    moves.push(m);
                    return moves;
                }
            }
        }

        for room_index in 0..self.rooms.len() {
            let piece0 = self.rooms[room_index][0];
            let piece1 = self.rooms[room_index][1];
            let correct_piece = room_index + 1;
            if piece0 == 0 && piece1 == 0
                || piece0 == 0 && piece1 == correct_piece
                || piece0 == correct_piece && piece1 == correct_piece
            {
                continue;
            }

            let from_index = if piece0 != 0 { 0 } else { 1 };
            let piece = if piece0 != 0 { piece0 } else { piece1 };
            for i in 0..11 {
                if i == 2 || i == 4 || i == 6 || i == 8 {
                    continue;
                }

                let m = Move::new(room_index, from_index, OPEN_SPACE_INDEX, i, piece);
                if self.is_move_valid(&m) {
                    moves.push(m);
                }
            }
        }
        moves
    }

    fn can_go_home(&self, piece: usize) -> bool {
        let home_index = piece - 1;
        if self.rooms[home_index][0] == 0 && self.rooms[home_index][1] == 0 {
            return true;
        }
        if self.rooms[home_index][0] == 0 && self.rooms[home_index][1] == piece {
            return true;
        }

        false
    }

    fn is_move_valid(&self, m: &Move) -> bool {
        let (start, dest) = if m.from_location == OPEN_SPACE_INDEX {
            (m.from_index, ROOM_ENTRANCES[m.to_location])
        } else if m.to_location == OPEN_SPACE_INDEX {
            (ROOM_ENTRANCES[m.from_location], m.to_index)
        } else {
            (
                ROOM_ENTRANCES[m.from_location],
                ROOM_ENTRANCES[m.to_location],
            )
        };

        if start < dest {
            for i in start + 1..=dest {
                if self.open_space[i] != 0 {
                    return false;
                }
            }
        } else {
            for i in dest..start {
                if self.open_space[i] != 0 {
                    return false;
                }
            }
        }

        true
    }

    fn execute_move(&self, m: &Move) -> Map {
        let mut map = self.clone();
        let piece;
        if m.from_location < OPEN_SPACE_INDEX {
            piece = map.rooms[m.from_location][m.from_index];
            map.rooms[m.from_location][m.from_index] = 0;
        } else {
            piece = map.open_space[m.from_index];
            map.open_space[m.from_index] = 0;
        }

        if m.to_location < OPEN_SPACE_INDEX {
            map.rooms[m.to_location][m.to_index] = piece;
        } else {
            map.open_space[m.to_index] = piece;
        }

        map
    }

    fn is_complete(&self) -> bool {
        for room_index in 0..self.rooms.len() {
            let correct_piece = room_index + 1;
            if self.rooms[room_index][0] != correct_piece
                || self.rooms[room_index][1] != correct_piece
            {
                return false;
            }
        }
        true
    }

    fn piece_name(piece: usize) -> char {
        if piece == 0 {
            return '.';
        }
        ('A' as u8 + (piece - 1) as u8) as char
    }

    fn print(&self) {
        println!("#############");
        print!("#");
        for i in 0..11 {
            print!("{}", Self::piece_name(self.open_space[i]));
        }
        println!("#");
        println!(
            "###{}#{}#{}#{}###",
            Self::piece_name(self.rooms[0][0]),
            Self::piece_name(self.rooms[1][0]),
            Self::piece_name(self.rooms[2][0]),
            Self::piece_name(self.rooms[3][0])
        );
        for i in 1..self.rooms[0].len() {
            println!(
                "  #{}#{}#{}#{}#",
                Self::piece_name(self.rooms[0][i]),
                Self::piece_name(self.rooms[1][i]),
                Self::piece_name(self.rooms[2][i]),
                Self::piece_name(self.rooms[3][i])
            );
        }
        println!("  #########");
    }
}

const ENERGY_COSTS: [i64; 4] = [1, 10, 100, 1000];
const ROOM_ENTRANCES: [usize; 4] = [2, 4, 6, 8];
const OPEN_SPACE_INDEX: usize = 4;

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    let input = fs::read_to_string("example.txt").expect("Unable to read file");
    //let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let map = Map::parse(&input);
    map.print();

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&map);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&map);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
