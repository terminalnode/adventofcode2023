use regex::{Match, Regex};
use crate::solution::Solution;

pub struct Day01 {
	file: String,
}

impl Solution for Day01 {
	fn new(file: &str) -> Self { Day01 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let result = self.read_file_as_string()?.lines()
			.filter_map(|line| {
				let digits = line.chars()
					.filter(|c| c.is_digit(10))
					.collect::<Vec<char>>();

				digits.first()
					.zip(digits.last())
					.map(|(first, last)| format!("{first}{last}"))
					.and_then(|s| s.parse::<u32>().ok())
			}).sum::<u32>();

		Ok(result.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let regex = match Regex::new(r"^(\d|one|two|three|four|five|six|seven|eight|nine)") {
			Ok(r) => Ok(r),
			Err(_) => Err("Failed to create regex"),
		}?;

		let result = self.read_file_as_string()?.lines()
			.map(|s| s.to_lowercase()).filter_map(|line| {
			let numbers = (0..line.len()).filter_map(|start| {
				let slice = line.chars().skip(start).collect::<String>();
				parse_line(regex.find(&slice)?)
			}).collect::<Vec<i32>>();

			numbers.first()
				.zip(numbers.last())
				.map(|(first, last)| format!("{first}{last}"))
				.and_then(|s| s.parse::<u32>().ok())
		}).sum::<u32>();

		Ok(result.to_string())
	}
}

fn parse_line(
	m: Match,
) -> Option<i32> {
	match m.as_str() {
		"1" | "one" => Ok(1),
		"2" | "two" => Ok(2),
		"3" | "three" => Ok(3),
		"4" | "four" => Ok(4),
		"5" | "five" => Ok(5),
		"6" | "six" => Ok(6),
		"7" | "seven" => Ok(7),
		"8" | "eight" => Ok(8),
		"9" | "nine" => Ok(9),
		c => Err(format!("Invalid number: {c}")),
	}.ok()
}
