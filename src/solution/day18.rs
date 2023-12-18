use Direction::{East, North, South, West};

use crate::solution::Solution;
use crate::util::Direction;

pub struct Day18 {
	file: String,
}

#[derive(Debug)]
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

	fn parse_fixed(&self) -> Result<Vec<InputRow>, String> {
		self.read_file_as_string()?
			.lines().into_iter()
			.map(|line| {
				let mut parts = line.split_whitespace().into_iter().skip(2);
				let hex = parts.next().ok_or("Missing hex")?;

				let meters = usize::from_str_radix(&hex[2..7], 16)
					.map_err(|e| format!("Invalid hex: {:?}", e))?;

				let direction = match hex.chars().nth(7) {
					Some('0') => East,
					Some('1') => South,
					Some('2') => West,
					Some('3') => North,
					x => return Err(format!("Invalid direction: {:?}", x)),
				};

				Ok(InputRow { direction, meters })
			}).collect()
	}
}

fn solve(inputs: Vec<InputRow>) -> Result<String, String> {
	let mut current_pos = (0i64, 0i64);
	let mut points = Vec::new();
	let mut boundary = 0;

	for input in inputs {
		let dist = input.meters as i64;
		boundary += dist;
		let (x, y) = current_pos;

		current_pos = match input.direction {
			North => (x, y - dist),
			South => (x, y + dist),
			East => (x + dist, y),
			West => (x - dist, y),
		};
		points.push(current_pos);

	}

	// Shoelace theorem
	// Sum of (x1y2 - x2y1) / 2, where x1y1 is here and x2y2 is the next coordinate
	let mut sum = 0i64;
	for i in 0..points.len() - 1 {
		let (x_1, y_1) = points[i];
		let (x_2, y_2) = points[i + 1];
		let p1 = x_1 * y_2;
		let p2 = x_2 * y_1;
		sum += p1 - p2;
	}
	sum = sum / 2;

	// Pick's theorem says -1, not +1, but I consistently get 2 too little.
	// I don't know why, possibly it has something to do with overlapping points in my set,
	// but this works and runs very fast, so I'm happy.
	let picks = sum.clone() + (boundary / 2) + 1;
	Ok(picks.to_string())
}

impl Solution for Day18 {
	fn new(file: &str) -> Self { Day18 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		solve(self.parse()?)
	}

	fn part_two(&self) -> Result<String, String> {
		solve(self.parse_fixed()?)
	}
}
