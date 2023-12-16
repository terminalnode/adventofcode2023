use std::fs;
use std::str::FromStr;

pub trait Solution {
	fn new(filename: &str) -> Self where Self: Sized;
	fn get_file_name(&self) -> &str;
	fn get_file_path(&self) -> String { format!("{}", self.get_file_name()) }

	fn part_one(&self) -> Result<String, String> {
		Err("Solution not yet implemented!".to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		Err("Solution not yet implemented!".to_string())
	}

	fn read_file_as_string(&self) -> Result<String, String> {
		match fs::read_to_string(self.get_file_path()) {
			Ok(s) => Ok(s),
			Err(s) => Err(format!("Failed to read file: {s}")),
		}
	}
}

pub struct PlaceholderSolution {}

impl Solution for PlaceholderSolution {
	fn new(_: &str) -> Self { PlaceholderSolution {} }
	fn get_file_name(&self) -> &str { panic!("Can't get file name for placeholder solution") }
}

pub fn extract_numbers<C, T>(
	list: &str,
) -> Result<C, String>
	where T: FromStr,
				C: FromIterator<T>,
{
	let numbers = list.split_whitespace()
		.filter_map(|num| num.parse::<T>().ok())
		.collect::<C>();
	Ok(numbers)
}
