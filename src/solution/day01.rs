use crate::solution::Solution;

pub struct Day01 { file: String }

impl Solution for Day01 {
	fn new(file: &str) -> Self { Day01 { file: file.to_string() } }

	fn part_one(&self) -> Result<String, String> {
		Ok("day 1 part 1".to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		Err("placeholder for day 1 / part 2".to_string())
	}
}
