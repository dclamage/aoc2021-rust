#[derive(Debug, Clone)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Point {
	pub fn new(x: i32, y: i32) -> Point {
		Point { x, y }
	}

	pub fn from_string(section: &str) -> Point {
		let mut split = section.split(",");
		let x = split.next().unwrap().trim().parse::<i32>().unwrap();
		let y = split.next().unwrap().trim().parse::<i32>().unwrap();
		Point::new(x, y)
	}
}
