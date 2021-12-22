use regex::Regex;
use std::fs;
use std::time;

fn create_tree(steps: &[Step]) -> GridNode {
    let mut bounds = steps[0].bounds;
    for step in steps {
        bounds = bounds.union(step.bounds);
    }

    let mut grid_root = GridNode::new(bounds, false);
    for step in steps {
        grid_root.run_step(&step);
    }

    grid_root
}

fn part1(grid_root: &GridNode) -> usize {
    grid_root.num_on_within(Bounds::new(Point::new_xyz(-50), Point::new_xyz(50)))
}

fn part2(grid_root: &GridNode) -> usize {
    grid_root.num_on()
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    fn new_xyz(xyz: i64) -> Point {
        Point::new(xyz, xyz, xyz)
    }

    fn min(&self, other: Point) -> Point {
        Point::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    fn max(&self, other: Point) -> Point {
        Point::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    min: Point,
    max: Point,
}

impl Bounds {
    fn new(min: Point, max: Point) -> Bounds {
        Bounds { min, max }
    }

    fn intersects(&self, other: Bounds) -> bool {
        self.min.x <= other.max.x
            && other.min.x <= self.max.x
            && self.min.y <= other.max.y
            && other.min.y <= self.max.y
            && self.min.z <= other.max.z
            && other.min.z <= self.max.z
    }

    fn contains(&self, other: Bounds) -> bool {
        self.min.x <= other.min.x
            && self.max.x >= other.max.x
            && self.min.y <= other.min.y
            && self.max.y >= other.max.y
            && self.min.z <= other.min.z
            && self.max.z >= other.max.z
    }

    fn volume(&self) -> i64 {
        (self.max.x - self.min.x + 1)
            * (self.max.y - self.min.y + 1)
            * (self.max.z - self.min.z + 1)
    }

    fn constrain_to(&self, other: Bounds) -> Bounds {
        Bounds::new(self.min.max(other.min), self.max.min(other.max))
    }

    fn union(&self, other: Bounds) -> Bounds {
        Bounds::new(self.min.min(other.min), self.max.max(other.max))
    }
}

#[derive(Debug, Clone, Copy)]
struct Step {
    on: bool,
    bounds: Bounds,
}

#[derive(Debug, Clone)]
struct GridNode {
    bounds: Bounds,
    is_on: bool,
    children: Option<Vec<Box<GridNode>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Component {
    X,
    Y,
    Z,
}

impl GridNode {
    fn new(bounds: Bounds, is_on: bool) -> GridNode {
        GridNode {
            bounds,
            is_on,
            children: None,
        }
    }

    fn create_children(&mut self, loc: i64, comp: Component) {
        let mut child0_bounds = self.bounds.clone();
        let mut child1_bounds = self.bounds.clone();
        match comp {
            Component::X => {
                child0_bounds.max.x = loc - 1;
                child1_bounds.min.x = loc;
            }
            Component::Y => {
                child0_bounds.max.y = loc - 1;
                child1_bounds.min.y = loc;
            }
            Component::Z => {
                child0_bounds.max.z = loc - 1;
                child1_bounds.min.z = loc;
            }
        }

        self.children = Some(vec![
            Box::new(GridNode::new(child0_bounds, self.is_on)),
            Box::new(GridNode::new(child1_bounds, self.is_on)),
        ]);
    }

    fn run_step(&mut self, step: &Step) {
        if !self.bounds.intersects(step.bounds) {
            return;
        }

        if self.children.is_none() {
            if self.is_on == step.on {
                return;
            }

            if self.bounds.min.x < step.bounds.min.x && self.bounds.max.x >= step.bounds.min.x {
                self.create_children(step.bounds.min.x, Component::X);
            } else if self.bounds.min.x <= step.bounds.max.x
                && self.bounds.max.x > step.bounds.max.x
            {
                self.create_children(step.bounds.max.x + 1, Component::X);
            } else if self.bounds.min.y < step.bounds.min.y
                && self.bounds.max.y >= step.bounds.min.y
            {
                self.create_children(step.bounds.min.y, Component::Y);
            } else if self.bounds.min.y <= step.bounds.max.y
                && self.bounds.max.y > step.bounds.max.y
            {
                self.create_children(step.bounds.max.y + 1, Component::Y);
            } else if self.bounds.min.z < step.bounds.min.z
                && self.bounds.max.z >= step.bounds.min.z
            {
                self.create_children(step.bounds.min.z, Component::Z);
            } else if self.bounds.min.z <= step.bounds.max.z
                && self.bounds.max.z > step.bounds.max.z
            {
                self.create_children(step.bounds.max.z + 1, Component::Z);
            } else {
                self.is_on = step.on;
                return;
            }
        } else if step.bounds.contains(self.bounds) {
            self.is_on = step.on;
            self.children = None;
            return;
        }

        if let Some(ref mut children) = self.children {
            for child in children {
                child.run_step(step);
            }
        }
    }

    fn num_on(&self) -> usize {
        if let Some(ref children) = self.children {
            children.iter().map(|child| child.num_on()).sum()
        } else if self.is_on {
            self.volume() as usize
        } else {
            0
        }
    }

    fn num_on_within(&self, bounds: Bounds) -> usize {
        if !self.bounds.intersects(bounds) {
            return 0;
        }

        if let Some(ref children) = self.children {
            children
                .iter()
                .map(|child| child.num_on_within(bounds))
                .sum()
        } else if self.is_on {
            let limited_bounds = self.bounds.constrain_to(bounds);
            limited_bounds.volume() as usize
        } else {
            0
        }
    }

    fn volume(&self) -> i64 {
        self.bounds.volume()
    }
}

fn parse_input(input: &str) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    let re =
        Regex::new(r"([onf]+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let on = &caps[1] == "on";
        let min_x = caps[2].parse::<i64>().unwrap();
        let max_x = caps[3].parse::<i64>().unwrap();
        let min_y = caps[4].parse::<i64>().unwrap();
        let max_y = caps[5].parse::<i64>().unwrap();
        let min_z = caps[6].parse::<i64>().unwrap();
        let max_z = caps[7].parse::<i64>().unwrap();
        let min = Point::new(min_x, min_y, min_z);
        let max = Point::new(max_x, max_y, max_z);
        steps.push(Step {
            on,
            bounds: Bounds::new(min, max),
        });
    }

    steps
}

fn main() {
    // Read all lines of input.txt
    let read_time_start = time::Instant::now();
    //let input = fs::read_to_string("simple_example.txt").expect("Unable to read file");
    //let input = fs::read_to_string("example.txt").expect("Unable to read file");
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let steps = parse_input(&input);

    let read_time = read_time_start.elapsed();

    println!("Part 1:");
    let part1_time_start = time::Instant::now();
    let grid_root = create_tree(&steps);
    let answer = part1(&grid_root);
    let part1_time = part1_time_start.elapsed();
    println!("Part 1: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Part 2:");
    let part2_time = time::Instant::now();
    let answer = part2(&grid_root);
    let part2_time = part2_time.elapsed();
    println!("Part 2: Answer: {}", answer);

    println!("");
    println!("----------------------------------");
    println!("");

    println!("Read time: {:?}", read_time);
    println!("Part 1 time: {:?}", part1_time);
    println!("Part 2 time: {:?}", part2_time);
}
