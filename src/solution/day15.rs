use crate::solution::Solution;

pub struct Day15 {
	file: String,
}

impl Day15 {
	fn parse(&self) -> Result<Vec<String>, String> {
		Ok(self.read_file_as_string()?
			.split(',')
			.map(|cs| {
				cs.to_string()
					.chars()
					.filter(|&c| c != '\n')
					.collect::<String>()
			})
			.collect())
	}
}

fn hash(s: &String) -> usize {
	let mut out = 0;
	for char in s.chars() {
		out += char as usize;
		out *= 17;
		out %= 256;
	}

	out
}

impl Solution for Day15 {
	fn new(file: &str) -> Self { Day15 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		Ok(self.parse()?
			.iter()
			.map(|s| hash(s))
			.sum::<usize>()
			.to_string())
	}
}
