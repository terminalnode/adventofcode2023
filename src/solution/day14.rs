use crate::solution::Solution;
use crate::util::{Matrix2D, Matrix2DExt};

pub struct Day14 {
	file: String,
}

type Matrix = Matrix2D<char>;

impl Day14 {
	fn parse(&self) -> Result<Matrix, String> {
		let file = self.read_file_as_string()?;
		Ok(file.lines()
			.map(|s| s.chars().collect::<Vec<char>>())
			.collect::<Matrix>())
	}
}

impl Solution for Day14 {
	fn new(file: &str) -> Self { Day14 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let mut matrix = self.parse()?;
		for y in matrix.y_range() {
			for x in matrix.x_range() {
				roll_north(&mut matrix, x, y);
			}
		}

		Ok(count_load(&matrix).to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let mut matrix = self.parse()?;
		let mut history = vec![];

		loop {
			for y in matrix.y_range() {
				for x in matrix.x_range() {
					roll_north(&mut matrix, x, y);
				}
			}

			for y in matrix.y_range() {
				for x in matrix.x_range() {
					roll_west(&mut matrix, x, y);
				}
			}

			for y in matrix.y_range().rev() {
				for x in matrix.x_range() {
					roll_south(&mut matrix, x, y);
				}
			}

			for y in matrix.y_range() {
				for x in matrix.x_range().rev() {
					roll_east(&mut matrix, x, y);
				}
			}

			if history.contains(&matrix) { break; }
			history.push(matrix.clone());
		}

		let offset = history.iter().position(|m| m == &matrix).unwrap();
		let trim_len = history.len() - offset;
		let idx = (1_000_000_000 - offset - 1) % trim_len;
		let final_m = history.iter().skip(offset).nth(idx).unwrap();

		Ok(count_load(&final_m).to_string())
	}
}

fn roll_north(
	matrix: &mut Matrix,
	x: usize,
	y: usize,
) {
	if y == 0 { return; }
	let here = matrix.get_xy(x, y);
	let there = matrix.get_xy(x, y - 1);

	if here == Some(&'O') && there == Some(&'.') {
		matrix.set_xy(x, y, '.');
		matrix.set_xy(x, y - 1, 'O');
		roll_north(matrix, x, y - 1);
	}
}

fn roll_south(
	matrix: &mut Matrix,
	x: usize,
	y: usize,
) {
	if y == matrix.y_len() - 1 { return; }
	let here = matrix.get_xy(x, y);
	let there = matrix.get_xy(x, y + 1);

	if here == Some(&'O') && there == Some(&'.') {
		matrix.set_xy(x, y, '.');
		matrix.set_xy(x, y + 1, 'O');
		roll_south(matrix, x, y + 1);
	}
}

fn roll_west(
	matrix: &mut Matrix,
	x: usize,
	y: usize,
) {
	if x == 0 { return; }
	let here = matrix.get_xy(x, y);
	let there = matrix.get_xy(x - 1, y);

	if here == Some(&'O') && there == Some(&'.') {
		matrix.set_xy(x, y, '.');
		matrix.set_xy(x - 1, y, 'O');
		roll_west(matrix, x - 1, y);
	}
}

fn roll_east(
	matrix: &mut Matrix,
	x: usize,
	y: usize,
) {
	if x == matrix.x_len() - 1 { return; }
	let here = matrix.get_xy(x, y);
	let there = matrix.get_xy(x + 1, y);

	if here == Some(&'O') && there == Some(&'.') {
		matrix.set_xy(x, y, '.');
		matrix.set_xy(x + 1, y, 'O');
		roll_east(matrix, x + 1, y);
	}
}

fn count_load(
	matrix: &Matrix,
) -> usize {
	matrix.iter()
		.map(|x| x.iter().filter(|&&c| c == 'O').count())
		.rev()
		.enumerate()
		.map(|(idx, value)|  value * (idx+1))
		.sum()
}
