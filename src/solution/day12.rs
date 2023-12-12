use std::collections::VecDeque;
use crate::solution::Solution;

pub struct Day12 {
	file: String,
}

#[derive(Debug)]
struct Record {
	original_row: usize,
	springs: Vec<char>,
	groups: Vec<usize>,
}

impl Day12 {
	fn parse(&self) -> Result<Vec<Record>, String> {
		let file = self.read_file_as_string()?;
		let mut records = Vec::new();
		for (i, line) in file.lines().enumerate() {
			let mut split = line.split(" ");
			let springs: Vec<char> = split.next()
				.ok_or(format!("Invalid line: {}", line))?
				.chars()
				.collect();

			let group_split = split.next().ok_or(format!("Invalid line: {}", line))?;
			let groups: Vec<usize> = group_split.split(",").map(|s| s.parse::<usize>().unwrap()).collect();

			records.extend(expand(i, &springs, &groups));
		}

		Ok(records)
	}
}

fn expand(
	original_row: usize,
	springs: &Vec<char>,
	groups: &Vec<usize>,
) -> Vec<Record> {
	let mut expanded_springs: Vec<Vec<char>> = vec![vec![]];

	for (i, c) in springs.iter().enumerate() {
		match c {
			'.' | '#' => {
				for ex_spring in &mut expanded_springs {
					ex_spring.push(*c)
				}
			}

			'?' => {
				let new_springs: Vec<Vec<char>> = expanded_springs.iter()
					.map(|s| {
						let mut clone = s.clone();
						clone.push('#');
						clone
					}).collect();
				for ex_spring in &mut expanded_springs {
					ex_spring.push('.')
				}
				expanded_springs.extend(new_springs);
			}
			_ => panic!("whaaaat")
		}
	}

	expanded_springs.iter().map(|springs| {
		Record {
			original_row,
			springs: springs.clone(),
			groups: groups.clone(),
		}
	}).collect()
}

fn validate(record: &Record) -> bool {
	let mut groups = VecDeque::from(record.groups.clone());
	let mut current: usize = 0;
	let mut next = groups.pop_front();

	for (i, c) in record.springs.iter().enumerate() {
		match c {
			'.' => {
				if current != 0 && next != Some(current) {
					return false;
				} else if current != 0 {
					next = groups.pop_front();
				}
				current = 0;
			}

			'#' => {
				if current == 0 && next == None {
					return false;
				} else if next <= Some(current) {
					// println!("next({next:?}) <= Some(current) @ {i}");
					return false;
				} else {
					current += 1;
				}
			}

			x => panic!("Illegal char in record: '{}'", x),
		};
	}

	return if current != 0 && Some(current) != next {
		false
	} else if next > Some(current) {
		false
	} else if !groups.is_empty() {
		false
	} else {
		true
	};
}

impl Solution for Day12 {
	fn new(file: &str) -> Self { Day12 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		Ok(self.parse()?
			.iter()
			.filter(|&r| validate(r))
			.count()
			.to_string())
	}
}
