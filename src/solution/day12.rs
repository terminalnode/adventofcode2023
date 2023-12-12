use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;

use crate::solution::Solution;

pub struct Day12 {
	file: String,
}

impl Day12 {
	fn solve(
		&self,
		copies: usize,
	) -> Result<usize, String> {
		let file = self.read_file_as_string()?;
		let lines = file.lines().collect::<Vec<&str>>();

		let done_count = AtomicUsize::new(0);
		let count = lines.par_iter().filter_map(|line| {
			let mut split = line.split(" ");
			let pre_springs: Vec<char> = split.next()?.chars().collect();
			let springs: Vec<char> = (0..copies)
				.flat_map(|i| {
					let mut result = pre_springs.clone();
					if i < copies - 1 { result.push('?'); }
					result
				}).collect();

			let group_split = split.next()?;
			let groups: Vec<usize> = group_split.split(",")
				.map(|s| s.parse::<usize>().unwrap())
				.collect::<Vec<usize>>();

			let x = Some(count(&springs, &groups.repeat(copies)));
			let done = done_count.fetch_add(1, Ordering::Relaxed);
			println!("\x1B[2K\x1B[1GDone: {done}\x1B[1A");
			x
		}).sum::<usize>();
		println!();

		Ok(count)
	}
}

fn count(
	springs: &Vec<char>,
	groups: &Vec<usize>,
) -> usize {
	let mut ok_count: usize = 0;
	let mut progresses: VecDeque<(usize, RecordProgress)> = VecDeque::from(vec![
		(0, RecordProgress {
			new_group: true,
			broke_in_group: 0,
			group_index: 0,
		}),
	]);

	while let Some((mut pos, mut rp)) = progresses.pop_back() {
		loop {
			if pos == springs.len() {
				if rp.group_index == groups.len() && Some(&rp.broke_in_group) == groups.last() {
					ok_count += 1;
				}
				break;
			}

			let should_continue = match springs.get(pos) {
				Some('.') => rp.move_dot(groups),
				Some('#') => rp.move_hash(groups),
				Some('?') => {
					// Add . to queue
					let mut dot_clone = rp.clone();
					if dot_clone.move_dot(groups) {
						progresses.push_back((pos + 1, dot_clone));
					}

					// Then walk down #
					rp.move_hash(groups)
				}

				None => panic!("We got None! How?"),
				Some(x) => panic!("Illegal char in springs: '{}'", x),
			};

			pos += 1;
			if !should_continue { break; }
		}
	};

	ok_count
}

#[derive(Debug, Clone)]
struct RecordProgress {
	broke_in_group: usize,
	group_index: usize,
	new_group: bool,
}

impl RecordProgress {
	fn move_dot(
		&mut self,
		groups: &Vec<usize>,
	) -> bool {
		if self.broke_in_group > 0 {
			self.new_group = true;
			if self.group_index > 0 && Some(&self.broke_in_group) != groups.get(self.group_index - 1) {
				return false;
			}
		}
		true
	}

	fn move_hash(
		&mut self,
		groups: &Vec<usize>,
	) -> bool {
		self.broke_in_group += 1;

		// New group?
		if self.new_group {
			self.group_index += 1;
			self.broke_in_group = 1;
			self.new_group = false;
		}

		// Group too big?
		if Some(&self.broke_in_group) > groups.get(self.group_index - 1) {
			return false;
		}

		true
	}
}

impl Solution for Day12 {
	fn new(file: &str) -> Self { Day12 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		Ok(self.solve(1)?.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		// It works and isn't super memory hungry, but you need a pretty beefy CPU
		// to run it in a... still pretty unreasonable time.
		Ok(self.solve(5)?.to_string())
	}
}
