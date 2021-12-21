#[allow(unused)]
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::time;

fn part1(player1_start: usize, player2_start: usize) -> usize {
    let mut game = GameState::new(player1_start, player2_start);
    let mut die_value = 1;
    let mut num_die_rolls = 0;
    while game.highest_score() < 1000 {
        let die_roll = die_value * 3 + 3;
        die_value = ((die_value + 2) % 100) + 1;
        num_die_rolls += 3;
        game = game.next_state(die_roll);
    }

    num_die_rolls * game.lowest_score()
}

fn part2(player1_start: usize, player2_start: usize) -> usize {
    let mut stack: Vec<GameState> = vec![GameState::new(player1_start, player2_start)];
    let mut player1_wins = 0;
    let mut player2_wins = 0;

    while stack.len() > 0 {
        let state = stack.pop().unwrap();
        for die_roll in 3..=9 {
            let next_state = state.next_state(die_roll);
            if next_state.player1.score >= 21 || next_state.player2.score >= 21 {
                if next_state.player1.score > next_state.player2.score {
                    player1_wins += next_state.num_universes;
                } else {
                    player2_wins += next_state.num_universes;
                }
            } else {
                stack.push(next_state);
            }
        }
    }

    if player1_wins > player2_wins {
        player1_wins
    } else {
        player2_wins
    }
}

#[derive(Debug, Clone, Copy)]
struct PlayerState {
    score: usize,
    loc: usize,
}

impl PlayerState {
    fn new(loc: usize) -> PlayerState {
        PlayerState { score: 0, loc }
    }

    fn next_state(&mut self, die_roll: usize) {
        let new_loc = ((self.loc + die_roll - 1) % 10) + 1;
        let new_score = self.score + new_loc;
        self.loc = new_loc;
        self.score = new_score;
    }
}

#[derive(Debug, Clone, Copy)]
struct GameState {
    player1: PlayerState,
    player2: PlayerState,
    player1_turn: bool,
    num_universes: usize,
}

static UNIVERSES_PER_ROLL: &'static [usize] = &[0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

impl GameState {
    fn new(player1_start: usize, player2_start: usize) -> GameState {
        GameState {
            player1: PlayerState::new(player1_start),
            player2: PlayerState::new(player2_start),
            player1_turn: true,
            num_universes: 1,
        }
    }

    fn highest_score(&self) -> usize {
        self.player1.score.max(self.player2.score)
    }

    fn lowest_score(&self) -> usize {
        self.player1.score.min(self.player2.score)
    }

    fn next_state(&self, die_roll: usize) -> GameState {
        let mut new_state = self.clone();
        if new_state.player1_turn {
            new_state.player1.next_state(die_roll);
        } else {
            new_state.player2.next_state(die_roll);
        }
        new_state.player1_turn = !new_state.player1_turn;
        if die_roll < UNIVERSES_PER_ROLL.len() {
            new_state.num_universes *= UNIVERSES_PER_ROLL[die_roll];
        }

        new_state
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    let mut lines = input.lines();

    let re = Regex::new(r"Player \d starting position: (\d)").unwrap();
    let cap1 = re.captures(lines.next().unwrap().trim()).unwrap();
    let cap2 = re.captures(lines.next().unwrap().trim()).unwrap();
    (
        cap1[1].parse::<usize>().unwrap(),
        cap2[1].parse::<usize>().unwrap(),
    )
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let (player1_start, player2_start) = parse_input(&input);
    println!("Player 1 start: {}", player1_start);
    println!("Player 2 start: {}", player2_start);

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(player1_start, player2_start);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(player1_start, player2_start);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
