use std::fs;
use std::time;
use itertools::Itertools;

fn part1(number_trees: &[NumberTree]) -> i64 {
    let mut accum_tree: NumberTree = number_trees[0].clone();
    for tree in &number_trees[1..] {
        accum_tree = accum_tree.add(tree);
    }

    accum_tree.magnitude()
}

fn part2(number_trees: &[NumberTree]) -> i64 {
    let mut highest_magnitude = 0;
    for tree_pairs in (0..number_trees.len()).permutations(2) {
        let tree = number_trees[tree_pairs[0]].add(&number_trees[tree_pairs[1]]);
        highest_magnitude = i64::max(highest_magnitude, tree.magnitude());
    }

    highest_magnitude
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Literal(i64),
    Pair(NumberPair),
}

#[derive(Debug, Clone, Copy)]
struct NumberPair {
    x: usize,
    y: usize,
}

impl NumberPair {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn parse(reader: &mut StringReader, value_array: &mut Vec<Value>) -> usize {
        let open_bracket = reader.read_char().unwrap();
        if open_bracket != '[' {
            panic!("Expected '[' but got '{}'", open_bracket);
        }

        let x = Self::read_value(reader, value_array);
        let comma = reader.read_char().unwrap();
        if comma != ',' {
            panic!("Expected ',' but got '{}'", comma);
        }

        let y = Self::read_value(reader, value_array);

        let close_bracket = reader.read_char().unwrap();
        if close_bracket != ']' {
            panic!("Expected ']' but got '{}'", close_bracket);
        }

        let parent_index = value_array.len();
        let parent: Value = Value::Pair(NumberPair::new(x, y));
        value_array.push(parent);

        parent_index
    }

    fn read_value(reader: &mut StringReader, value_array: &mut Vec<Value>) -> usize {
        let next_char = reader.peek_char().unwrap();
        if next_char == '[' {
            NumberPair::parse(reader, value_array)
        } else {
            value_array.push(Value::Literal(reader.read_number().unwrap()));
            value_array.len() - 1
        }
    }
}

#[derive(Debug, Clone)]
struct NumberTree {
    values: Vec<Value>,
    root_index: usize,
}

impl NumberTree {
    fn new(input: &str) -> Self {
        let mut reader = StringReader::new(input);
        let mut values = Vec::new();
        let root_index = NumberPair::parse(&mut reader, &mut values);
        NumberTree { values, root_index }
    }

    fn create_literal(&mut self, value: i64) -> usize {
        self.values.push(Value::Literal(value));
        self.values.len() - 1
    }

    fn add(&self, other: &NumberTree) -> NumberTree {
        let mut values = Vec::with_capacity(self.values.len() + other.values.len() + 1);
        values.extend(&self.values);

        let other_start_index = values.len();
        values.extend(&other.values);
        for i in other_start_index..values.len() {
            if let Value::Pair(pair) = &mut values[i] {
                pair.x += other_start_index;
                pair.y += other_start_index;
            }
        }

        let root_index = values.len();
        values.push(Value::Pair(NumberPair::new(
            self.root_index,
            other_start_index + other.root_index,
        )));
        let mut result_tree = NumberTree { values, root_index };
        result_tree.reduce();
        result_tree
    }

    fn get_literal_parent_at(&self, at_index: usize) -> Option<(usize, bool)> {
        let root = &self.values[self.root_index];
        if let Value::Pair(_) = root {
            let mut stack: Vec<(usize, bool)> = Vec::new();
            stack.push((self.root_index, false));
            stack.push((self.root_index, true));

            let mut index = 0;
            while stack.len() > 0 {
                let (cur_pair, is_left) = stack.pop().unwrap();
                if let Value::Pair(parent_pair) = self.values[cur_pair] {
                    let cur_value = if is_left {
                        parent_pair.x
                    } else {
                        parent_pair.y
                    };
                    match self.values[cur_value] {
                        Value::Pair(_) => {
                            stack.push((cur_value, false));
                            stack.push((cur_value, true));
                        }
                        Value::Literal(_) => {
                            if index == at_index {
                                return Some((cur_pair, is_left));
                            }
                            index += 1;
                        }
                    }
                }
            }

            None
        } else if at_index == 0 {
            Some((self.root_index, false))
        } else {
            None
        }
    }

    fn add_to_literal(&mut self, index: usize, value: i64) {
        let parent_opt = self.get_literal_parent_at(index);
        if parent_opt.is_none() {
            return;
        }
        
        let (parent_index, is_left) = parent_opt.unwrap();
        if let Value::Pair(pair) = self.values[parent_index] {
            if is_left {
                if let Value::Literal(literal) = self.values[pair.x] {
                    self.values[pair.x] = Value::Literal(literal + value);
                } else {
                    panic!("Expected literal but got pair");
                }
            } else {
                if let Value::Literal(literal) = self.values[pair.y] {
                    self.values[pair.y] = Value::Literal(literal + value);
                } else {
                    panic!("Expected literal but got pair");
                }
            }
        } else {
            panic!("Expected parent to be a pair");
        }
    }

    fn explode(&mut self) -> bool {
        if let Value::Literal(_) = &self.values[self.root_index] {
            return false;
        }

        let mut stack: Vec<(usize, bool, usize)> = Vec::new();
        stack.push((self.root_index, false, 1));
        stack.push((self.root_index, true, 1));

        let mut literal_index = 0;
        while stack.len() > 0 {
            let (cur_pair, is_left, depth) = stack.pop().unwrap();
            if let Value::Pair(parent_pair) = self.values[cur_pair] {
                let cur_value = if is_left {
                    parent_pair.x
                } else {
                    parent_pair.y
                };
                match self.values[cur_value] {
                    Value::Pair(cur_pair) => {
                        if depth < 4 {
                            stack.push((cur_value, false, depth + 1));
                            stack.push((cur_value, true, depth + 1));
                        } else {
                            if let Value::Literal(literal) = self.values[cur_pair.x] {
                                if literal_index > 0 {
                                    self.add_to_literal(literal_index - 1, literal);
                                }
                                literal_index += 1;
                            } else {
                                panic!("Expected literal but got pair");
                            }

                            if let Value::Literal(literal) = self.values[cur_pair.y] {
                                self.add_to_literal(literal_index + 1, literal);
                            } else {
                                panic!("Expected literal but got pair");
                            }

                            self.values[cur_value] = Value::Literal(0);
                            return true;
                        }
                    }
                    Value::Literal(_) => {
                        literal_index += 1;
                    }
                }
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        if let Value::Literal(literal) = self.values[self.root_index] {
            // Special case for if the root is a literal
            if literal > 9 {
                let x = self.create_literal(literal / 2);
                let y = self.create_literal(literal - literal / 2);
                self.values[self.root_index] = Value::Pair(NumberPair::new(x, y));
                return true;
            }
            return false;
        }

        let mut stack: Vec<(usize, bool)> = Vec::new();
        stack.push((self.root_index, false));
        stack.push((self.root_index, true));

        while stack.len() > 0 {
            let (cur_pair, is_left) = stack.pop().unwrap();
            if let Value::Pair(parent_pair) = self.values[cur_pair] {
                let cur_value = if is_left {
                    parent_pair.x
                } else {
                    parent_pair.y
                };
                match self.values[cur_value] {
                    Value::Pair(_) => {
                        stack.push((cur_value, false));
                        stack.push((cur_value, true));
                    }
                    Value::Literal(literal) => {
                        if literal > 9 {
                            let x = self.create_literal(literal / 2);
                            let y = self.create_literal(literal - literal / 2);
                            self.values[cur_value] = Value::Pair(NumberPair::new(x, y));
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    fn magnitude(&self) -> i64 {
        self.magnitude_helper(self.root_index)
    }

    fn magnitude_helper(&self, index: usize) -> i64 {
        let mut mag: i64 = 0;
        match &self.values[index] {
            Value::Literal(literal) => {
                mag += *literal;
            }
            Value::Pair(pair) => {
                mag += 3 * self.magnitude_helper(pair.x);
                mag += 2 * self.magnitude_helper(pair.y);
            }
        }

        mag
    }
}

struct StringReader {
    input: String,
    pos: usize,
}

impl StringReader {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        if self.pos >= self.input.len() {
            return None;
        }

        let c = self.input.chars().nth(self.pos).unwrap();
        self.pos += 1;
        Some(c)
    }

    fn peek_char(&self) -> Option<char> {
        if self.pos >= self.input.len() {
            return None;
        }

        let c = self.input.chars().nth(self.pos).unwrap();
        Some(c)
    }

    fn read_number(&mut self) -> Option<i64> {
        let mut result = 0;
        let mut is_negative = false;

        if let Some(c) = self.read_char() {
            if c == '-' {
                is_negative = true;
            } else {
                self.pos -= 1;
            }
        }

        while let Some(c) = self.read_char() {
            if c.is_digit(10) {
                result = result * 10 + (c as i64 - '0' as i64);
            } else {
                self.pos -= 1;
                break;
            }
        }

        if is_negative {
            Some(-result)
        } else {
            Some(result)
        }
    }
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let number_trees: Vec<NumberTree> = input.lines().map(|line| NumberTree::new(line)).collect();

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let answer = part1(&number_trees);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&number_trees);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
