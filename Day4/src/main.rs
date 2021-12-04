use std::fs;
use std::time;

fn part1(balls: &Vec<u32>, boards: &mut Vec<Board>) -> (u32, u32) {
    for cur_ball in balls {
        for board in boards.iter_mut() {
            if board.mark_ball(*cur_ball) {
                if board.is_solved() {
                    return (*cur_ball, board.sum_unset());
                }
            }
        }
    }

    (0, 0)
}

fn part2(balls: &Vec<u32>, boards: &mut Vec<Board>) -> (u32, u32) {
    let total_boards = boards.len();
    let mut board_finished: Vec<bool> = vec![false; boards.len()];
    let mut num_finished = 0;

    for cur_ball in balls {
        for (i, board) in boards.iter_mut().enumerate() {
            if board_finished[i] {
                continue;
            }

            if board.mark_ball(*cur_ball) {
                if board.is_solved() {
                    board_finished[i] = true;
                    num_finished += 1;
                    if num_finished == total_boards {
                        return (*cur_ball, board.sum_unset());
                    }
                }
            }
        }
    }

    (0, 0)
}

fn parse_first_line(line: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for s in line.split(",") {
        result.push(s.parse::<u32>().unwrap());
    }
    result
}

fn parse_board_line(line: &str, list: &mut Vec<u32>) {
    for s in line.split(" ") {
        let s = s.trim();
        if s.len() > 0 {
            list.push(s.parse::<u32>().unwrap());
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    numbers: Vec<u32>,
    is_called: Vec<bool>,
    num_marked_balls: u32,
}

impl Board {
    fn new(numbers: Vec<u32>) -> Board {
        Board {
            is_called: vec![false; numbers.len()],
            numbers: numbers,
            num_marked_balls: 0,
        }
    }

    fn resetted(&self) -> Board {
        Board {
            is_called: vec![false; self.numbers.len()],
            numbers: self.numbers.clone(),
            num_marked_balls: 0,
        }
    }

    fn ball_index(&self, ball: u32) -> usize {
        match self.numbers.iter().position(|&b| b == ball) {
            Some(i) => i,
            None => self.numbers.len(),
        }
    }

    fn mark_ball(&mut self, ball: u32) -> bool {
        match self.ball_index(ball) {
            i if i < self.numbers.len() => {
                self.is_called[i] = true;
                self.num_marked_balls += 1;
                true
            }
            _ => false,
        }
    }

    fn is_solved(&self) -> bool {
        if self.num_marked_balls < 5 {
            return false;
        }

        let width = (self.numbers.len() as f64).sqrt() as usize;
        let height = self.numbers.len() / width;

        // Check each row
        for i in 0..height {
            let mut solved_count = 0;
            for j in 0..width {
                if self.is_called[i * width + j] {
                    solved_count += 1;
                }
            }
            if solved_count == width {
                return true;
            }
        }

        // Check each column
        for j in 0..width {
            let mut solved_count = 0;
            for i in 0..height {
                if self.is_called[i * width + j] {
                    solved_count += 1;
                }
            }
            if solved_count == height {
                return true;
            }
        }

        false
    }

    fn sum_unset(&self) -> u32 {
        self.is_called
            .iter()
            .enumerate()
            .fold(0, |acc, (i, is_called)| {
                if !*is_called {
                    acc + self.numbers[i]
                } else {
                    acc
                }
            })
    }
}

fn reset_boards(boards: &Vec<Board>) -> Vec<Board> {
    let mut result: Vec<Board> = Vec::new();
    for board in boards {
        result.push(board.resetted());
    }

    result
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<&str> = input.lines().collect();

    let balls = parse_first_line(&lines[0]);

    let mut boards: Vec<Board> = Vec::new();
    let mut current_board_values: Vec<u32> = Vec::new();
    for line in &lines[1..] {
        if line.trim().len() == 0 {
            continue;
        }
        parse_board_line(line, &mut current_board_values);
        if current_board_values.len() >= 25 {
            boards.push(Board::new(current_board_values));
            current_board_values = Vec::new();
        }
    }
    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let (ball, num_unset) = part1(&balls, &mut reset_boards(&boards));
    let part1_time = part1_time_start.elapsed();

    println!("Part 1: Ball: {}, Unset: {}", ball, num_unset);
    let part1_answer = ball * num_unset;
    println!("Part 1: Answer: {}", part1_answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let (ball, num_unset) = part2(&balls, &mut reset_boards(&boards));
    let part2_time = part2_time.elapsed();

    println!("Part 2: Ball: {}, Unset: {}", ball, num_unset);
    let part2_answer = ball * num_unset;
    println!("Part 2: Answer: {}", part2_answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
