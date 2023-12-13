use std::cmp::min;

use crate::solution::Solution;

pub struct Day13 {
	file: String,
}

#[derive(Debug)]
struct Pattern {
	columns: Vec<String>,
	rows: Vec<String>,
}

impl Day13 {
	fn parse(&self) -> Result<Vec<Pattern>, String> {
		self.read_file_as_string()?
			.split("\n\n")
			.map(|it| self.parse_pattern(it))
			.collect::<Result<Vec<Pattern>, String>>()
	}

	fn parse_pattern(
		&self,
		pattern: &str,
	) -> Result<Pattern, String> {
		let rows = pattern.lines().map(|it| it.to_string()).collect::<Vec<String>>();
		let mut columns = vec![];
		for x in 0..rows[0].len() {
			let mut column = String::with_capacity(rows.len());
			for y in 0..rows.len() {
				column.push(rows[y].chars().nth(x).unwrap());
			}
			columns.push(column);
		}

		Ok(Pattern { columns, rows })
	}
}

fn find_reflection_line(lines: &Vec<String>) -> Option<usize> {
	for i1 in 0..(lines.len() - 1) {
		if lines.get(i1) == lines.get(i1+1) {
			let remaining = lines.len() - i1 - 1;
			let reflecting = min(i1+1, remaining);

			let mut success = true;
			for i2 in 1..reflecting {
				if lines.get(i1 - i2) != lines.get(i1 + i2 + 1) {
					success = false;
					break;
				}
			}
			if success { return Some(i1); }
		}
	}

	None
}

impl Solution for Day13 {
	fn new(file: &str) -> Self { Day13 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let patterns = self.parse()?;

		let mut left: usize = 0;
		let mut above: usize = 0;
		for pattern in &patterns {
			if let Some(row) = find_reflection_line(&pattern.rows) {
				above += row + 1
			};

			if let Some(col) = find_reflection_line(&pattern.columns) {
				left += col + 1
			};
		}

		Ok(format!("{above} * 100 + {left} = {}", above * 100 + left))
	}
}
