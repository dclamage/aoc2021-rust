use crate::line::Line;
use std::cmp;

#[derive(Debug, Clone)]
pub struct Grid {
    points: Vec<i32>,
    width: i32,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Grid {
        Grid {
            points: vec![0; (width * height) as usize],
            width,
        }
    }

    pub fn new_for(lines: &[Line]) -> Grid {
        let width = lines
            .iter()
            .map(|line| cmp::max(line.start.x, line.end.x))
            .max()
            .unwrap()
            + 1;
        let height = lines
            .iter()
            .map(|line| cmp::max(line.start.y, line.end.y))
            .max()
            .unwrap()
            + 1;
        Grid::new(width, height)
    }

    fn increment(&mut self, x: i32, y: i32) {
        self.points[(y * self.width + x) as usize] += 1;
    }

    fn calc_increment(start: i32, end: i32) -> i32 {
        match end - start {
            0 => 0,
            x if x > 0 => 1,
            _ => -1,
        }
    }

    pub fn increment_line(&mut self, line: &Line, allow_diagonals: bool) {
        if !allow_diagonals && line.start.x != line.end.x && line.start.y != line.end.y {
            return;
        }

        let mut x = line.start.x;
        let mut y = line.start.y;
        let x_incr = Grid::calc_increment(line.start.x, line.end.x);
        let y_incr = Grid::calc_increment(line.start.y, line.end.y);
        loop {
            self.increment(x, y);
            if x == line.end.x && y == line.end.y {
                break;
            }
            x += x_incr;
            y += y_incr;
        }
    }

    pub fn count_two_or_higher(&self) -> u32 {
        self.points.iter().filter(|&p| *p >= 2).count() as u32
    }
}
