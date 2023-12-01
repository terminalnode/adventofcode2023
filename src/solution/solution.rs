use std::fs;

pub trait Solution {
	fn new(filename: &str) -> Self where Self: Sized;
	fn get_file_name(&self) -> &str;
	fn get_file_path(&self) -> String { format!("data/{}", self.get_file_name()) }

	fn part_one(&self) -> Result<String, String> {
		Err("Solution not yet implemented!".to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		Err("Solution not yet implemented!".to_string())
	}

	fn read_file_as_lines(&self) -> Result<Vec<String>, String> {
		match fs::read_to_string(self.get_file_path()) {
			Ok(s) => {
				let split = s.split("\n")
					.map(|s| s.to_string())
					.collect::<Vec<String>>();
				Ok(split)
			}
			Err(s) => Err(format!("Failed to read file: {}", s)),
		}
	}
}

pub struct PlaceholderSolution {}

impl Solution for PlaceholderSolution {
	fn new(_: &str) -> Self { PlaceholderSolution {} }
	fn get_file_name(&self) -> &str { panic!("Can't get file name for placeholder solution") }
}
