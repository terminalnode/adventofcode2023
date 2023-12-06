use crate::solution::Solution;
use crate::solution::solution::extract_numbers;

pub struct Day06 {
	file: String,
}

#[derive(Debug)]
struct TimeDistance {
	time: usize,
	distance: usize,
}

impl Day06 {
	fn parse(&self) -> Result<Vec<TimeDistance>, String> {
		let file = self.read_file_as_string()?;
		let mut lines = file.lines();
		let time = extract_numbers::<Vec<usize>, _>(lines.next().ok_or("failed to extract time")?)?;
		let distance = extract_numbers::<Vec<usize>, _>(lines.next().ok_or("failed to extract time")?)?;

		let tds = time.iter()
			.zip(distance)
			.map(|(&time, distance)| TimeDistance { time, distance })
			.collect::<Vec<TimeDistance>>();
		Ok(tds)
	}
}

impl Solution for Day06 {
	fn new(file: &str) -> Self { Day06 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let input = self.parse()?;
		let result = input.iter()
			.map(|td| {
				(1..td.time).filter_map(|time_held| {
					let travel = (td.time - time_held) * time_held;
					if travel > td.distance {
						Some((time_held, travel))
					} else {
						None
					}
				}).count()
			}).product::<usize>();
		Ok(result.to_string())
	}
}
