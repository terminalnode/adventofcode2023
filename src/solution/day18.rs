use std::collections::HashSet;

use Direction::{East, North, South, West};

use crate::solution::Solution;
use crate::util::{Direction, Point2D, Point2DExt};

pub struct Day18 {
	file: String,
}

struct InputRow {
	direction: Direction,
	meters: usize,
}

impl Day18 {
	fn parse(&self) -> Result<Vec<InputRow>, String> {
		self.read_file_as_string()?
			.lines().into_iter()
			.map(|line| {
				let mut parts = line.split_whitespace();
				let direction = match parts.next() {
					Some("R") => East,
					Some("L") => West,
					Some("U") => North,
					Some("D") => South,
					c => return Err(format!("Invalid direction: {:?}", c)),
				};

				let meters = match parts.next() {
					Some(m) => m.parse::<usize>().map_err(|e| format!("Invalid meters: {:?}", e))?,
					None => return Err("Missing meters".to_string()),
				};

				Ok(InputRow { direction, meters })
			}).collect()
	}
}

impl Solution for Day18 {
	fn new(file: &str) -> Self { Day18 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let inputs = self.parse()?;
		let mut current_pos = (0i32, 0i32);

		let mut points = HashSet::new();
		points.insert(current_pos);

		// Build the loop
		let mut min_x = 0;
		let mut min_y = 0;
		let mut max_x = 0;
		let mut max_y = 0;
		for input in inputs {
			for _ in 0..input.meters {
				let (x, y) = current_pos;
				current_pos = match input.direction {
					North => (x, y - 1),
					South => (x, y + 1),
					East => (x + 1, y),
					West => (x - 1, y),
				};

				let (x, y) = current_pos;
				min_x = min_x.min(x);
				min_y = min_y.min(y);
				max_x = max_x.max(x);
				max_y = max_y.max(y);
				points.insert(current_pos);
			}
		}

		// Normalize the grid, easier to reason about non-negative coordinates
		// plus I can use my Point2D extensions if I want to.
		let max_y = (max_y - min_y) as usize;
		let mut points = points.iter().map(|(x, y)| {
			let x = x - min_x;
			let y= y - min_y;
			(x as usize, y as usize)
		}).collect::<HashSet<Point2D>>();

		// Find a starting point
		let mut start = None;
		for y in 0..max_y {
			let starts_with_hash = points.contains(&(0, y));
			let then_no_hash = !points.contains(&(1, y));
			if starts_with_hash && then_no_hash {
				start = Some((1, y));
				break;
			}
		}
		let start = start.ok_or("Could not find a starting point")?;

		// Flood-fill
		let mut queue = vec![start];
		let directions = vec![North, South, East, West];
		while let Some(point) = queue.pop() {
			let new_points = directions.iter()
				.filter_map(|dir| point.move_dir(dir))
				.filter(|p| !points.contains(p))
				.collect::<Vec<Point2D>>();

			for &point in new_points.iter() {
				points.insert(point);
				queue.push(point);
			}
		}

		Ok(points.len().to_string())
	}
}
