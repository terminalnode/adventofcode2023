use std::collections::{HashSet, VecDeque};
use Direction::{East, North, South, West};
use crate::solution::Solution;
use crate::util::{Direction, Matrix2DExt, Point2D, Point2DExt};

pub struct Day21 {
	file: String,
}

impl Day21 {
	fn parse(&self) -> Result<(Point2D, HashSet<Point2D>), String> {
		let lines = self.read_file_as_string()?.lines()
			.map(|line| line.chars().collect::<Vec<char>>())
			.collect::<Vec<Vec<char>>>();
		let mut start = None;
		let mut rocks = HashSet::new();

		for x in lines.x_range() {
			for y in lines.y_range() {
				let char = lines.get_xy(x, y).ok_or("Failed to get char")?;
				if char == &'S' {
					start = Some((x, y));
				} else if char == &'#' {
					rocks.insert((x, y));
				}
			}
		}

		let start = start.ok_or("No start found")?;
		Ok((start, rocks))
	}
}

impl Solution for Day21 {
	fn new(file: &str) -> Self { Day21 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let (start, rocks) = self.parse()?;
		let directions = vec![North, South, East, West];
		let mut queue = VecDeque::from(vec![(0usize, start)]);
		let mut done = HashSet::new();
		let done_steps = 64usize;

		while let Some((steps, pos)) = queue.pop_front() {
			let new_positions = directions.iter()
				.filter_map(|dir| pos.move_dir(dir));

			let new_steps = steps + 1;
			for new_pos in new_positions {
				if rocks.contains(&new_pos) {
					continue;
				} else if new_steps == done_steps {
					done.insert(new_pos);
					continue;
				}

				if new_steps % 2 == 0 {
					if done.contains(&new_pos) { continue; }
					else { done.insert(new_pos); }
				}

				queue.push_back((new_steps, new_pos));
			}
		}

		Ok(done.len().to_string())
	}
}
