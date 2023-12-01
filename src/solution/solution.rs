pub trait Solution {
	fn new(filename: &str) -> Self where Self: Sized;

	fn part_one(&self) -> Result<String, String> {
		Err("Solution not yet implemented!".to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		Err("Solution not yet implemented!".to_string())
	}
}

pub struct PlaceholderSolution {}

impl Solution for PlaceholderSolution {
	fn new(_: &str) -> Self { PlaceholderSolution {} }
}
