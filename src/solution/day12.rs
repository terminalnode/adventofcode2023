use std::collections::VecDeque;
use rayon::prelude::*;
use crate::solution::Solution;

pub struct Day12 {
	file: String,
}

#[derive(Debug, Clone)]
struct Record {
	springs: Vec<char>,
	groups: Vec<usize>,
}

#[derive(Debug)]
struct RecordProgress {
	record: Record,
	broke_in_group: usize,
	group_index: usize,
}

impl Day12 {
	fn parse(
		&self,
		copies: usize,
	) -> Result<Vec<Record>, String> {
		let file = self.read_file_as_string()?;
		let lines = file.lines().collect::<Vec<&str>>();

		let records = lines.par_iter().enumerate().filter_map(|(i, line)| {
			let mut split = line.split(" ");
			let springs: Vec<char> = split.next()?.chars().collect();

			let group_split = split.next()?;
			let groups: Vec<usize> = group_split.split(",")
				.map(|s| s.parse::<usize>().unwrap())
				.collect::<Vec<usize>>();

			let x = Some(expand(&springs.repeat(copies), &groups.repeat(copies)));
			println!("Finished {i}");
			x
		}).flatten().collect::<Vec<Record>>();

		println!("Finished parsing");
		Ok(records)
	}
}

fn as_readable(rp: &RecordProgress) -> String {
	rp.record.springs.iter().collect::<String>()
}

fn expand(
	springs: &Vec<char>,
	groups: &Vec<usize>,
) -> Vec<Record> {
	let readable = springs.iter().collect::<String>();
	let group_count = groups.len();
	let mut progresses: Vec<RecordProgress> = vec![
		RecordProgress {
			record: Record {
				springs: vec![],
				groups: groups.clone(),
			},
			broke_in_group: 0,
			group_index: 0,
		},
	];

	for (i, char) in springs.iter().enumerate() {
		progresses = match char {
			'?' => {
				progresses.iter().flat_map(|p| {
					let next_group_count = groups.get(p.group_index);
					let mut new: Vec<RecordProgress> = vec![];

					if next_group_count != None && next_group_count > Some(&p.broke_in_group) {
						let mut record = p.record.clone();
						record.springs.push('#');
						new.push(RecordProgress {
							record,
							broke_in_group: p.broke_in_group + 1,
							group_index: p.group_index,
						});
					}

					// If not in a group, or group has right size, add dot
					if p.broke_in_group == 0 || next_group_count == Some(&p.broke_in_group) {
						let mut record = p.record.clone();
						record.springs.push('.');
						new.push(RecordProgress {
							record,
							broke_in_group: 0,
							group_index: if p.broke_in_group > 0 { p.group_index + 1 } else { p.group_index },
						});
					}

					new
				}).collect()
			}

			'.' => {
				progresses.retain_mut(|p| {
					let next_group_count = groups.get(p.group_index);
					if p.broke_in_group != 0 && next_group_count == None {
						return false;
					} else if p.broke_in_group != 0 && Some(&p.broke_in_group) != next_group_count {
						return false;
					}
					p.record.springs.push('.');

					if p.broke_in_group != 0 {
						p.broke_in_group = 0;
						p.group_index += 1;
					}

					true
				});
				progresses
			}

			'#' => {
				progresses.retain_mut(|p| {
					let next_group_count = groups.get(p.group_index);
					p.broke_in_group += 1;

					if p.broke_in_group == 0 && next_group_count == None {
						return false;
					} else if Some(&p.broke_in_group) > next_group_count {
						return false;
					}
					p.record.springs.push('#');

					true
				});
				progresses
			}

			c => panic!("Unknown char '{c}' in {springs:?}")
		};
	}

	progresses.retain(|rp| {
		validate(&rp.record, true)
	});

	progresses.iter().map(|rp| rp.record.clone()).collect()
}

fn validate(record: &Record, is_final: bool) -> bool {
	let mut groups = VecDeque::from(record.groups.clone());
	let mut current: usize = 0;
	let mut next = groups.pop_front();

	for (i, c) in record.springs.iter().enumerate() {
		match c {
			'.' => {
				if is_final && current != 0 && next != Some(current) {
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
					return false;
				} else {
					current += 1;
				}
			}

			x => panic!("Illegal char in record: '{}'", x),
		};
	}

	return if !is_final {
		true
	} else if current != 0 && Some(current) != next {
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
		Ok(self.parse(1)?
			.par_iter()
			.count()
			.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		Ok(self.parse(5)?
			.par_iter()
			.count()
			.to_string())
	}
}
