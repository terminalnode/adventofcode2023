use crate::solution::Solution;

pub struct Day01 {
	file: String,
}

impl Solution for Day01 {
	fn new(file: &str) -> Self { Day01 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let lines = self.read_file_as_lines()?;
		let result = lines.iter().filter_map(|line| {
			let digits = line.chars()
				.filter(|c| c.is_digit(10))
				.collect::<Vec<char>>();

			digits.first()
				.zip(digits.last())
				.map(|(first, last)| format!("{}{}", first, last))
				.and_then(|s| s.parse::<u32>().ok())
		}).sum::<u32>();

		Ok(format!("{}", result))
	}

	fn part_two(&self) -> Result<String, String> {
		Err("placeholder for day 1 / part 2".to_string())
	}
}
