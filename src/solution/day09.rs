use crate::solution::Solution;
use crate::solution::solution::extract_numbers;

pub struct Day09 {
	file: String,
}

type ISand = i32;
type History = Vec<ISand>;

impl Day09 {
	fn parse_input(&self) -> Result<Vec<History>, String> {
		let lines = self.read_file_as_string()?
			.lines()
			.map(|line| extract_numbers::<History, _>(line))
			.collect::<Result<Vec<History>, String>>()?;
		Ok(lines)
	}
}

fn extrapolate_history(
	history: History,
) -> Vec<History> {
	let mut new_history = Vec::with_capacity(history.len());
	let mut all_zero = false;

	for i in 0..history.len() - 1 {
		let new = history[i + 1] - history[i];
		all_zero = all_zero || new != 0;
		new_history.push(new);
	}

	if !all_zero {
		vec![new_history, history]
	} else {
		let mut hs = extrapolate_history(new_history);
		hs.push(history);
		hs
	}
}

fn add_predictions(
	histories: &mut Vec<History>,
) {
	for i in 0..histories.len() {
		if i == 0 {
			histories[i].push(0);
			continue;
		}

		let below = *histories[i - 1].last().unwrap();
		let line = &mut histories[i];
		let last = *line.last().unwrap();
		line.push(last + below)
	}
}

impl Solution for Day09 {
	fn new(file: &str) -> Self { Day09 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let result = self.parse_input()?.iter()
			.map(|history| {
				let mut new_history = extrapolate_history(history.clone());
				add_predictions(&mut new_history);
				*new_history.last()
					.map_or(None, |h| h.last())
					.unwrap_or(&0)
			}).sum::<ISand>();

		Ok(result.to_string())
	}
}
