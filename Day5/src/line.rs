use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    pub fn from_string(line: &str) -> Line {
        let sections = line.split("->").map(|s| s.trim()).collect::<Vec<&str>>();
        if sections.len() != 2 {
            panic!("Invalid line: {}", line);
        }
        let start = Point::from_string(sections[0]);
        let end = Point::from_string(sections[1]);
        Line::new(start, end)
    }
}
