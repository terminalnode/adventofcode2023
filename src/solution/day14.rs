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
				roll_up(&mut matrix, x, y);
			}
		}

		Ok(count_load(&matrix).to_string())
	}
}

fn roll_up(
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
		roll_up(matrix, x, y - 1);
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
