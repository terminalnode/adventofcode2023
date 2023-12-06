use rayon::prelude::*;
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

	fn merge_time_distances(
		&self,
		tds: Vec<TimeDistance>,
	) -> Result<TimeDistance, String> {
		let time = tds.iter()
			.map(|td| td.time.to_string())
			.collect::<String>()
			.parse::<usize>()
			.map_err(|_| format!("failed to merge time"))?;
		let distance = tds.iter()
			.map(|td| td.distance.to_string())
			.collect::<String>()
			.parse::<usize>()
			.map_err(|_| format!("failed to merge distance"))?;

		Ok(TimeDistance { time, distance })
	}
}

impl Solution for Day06 {
	fn new(file: &str) -> Self { Day06 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let input = self.parse()?;
		let result = input.into_par_iter()
			.map(|td| {
				(1..td.time).filter_map(|time_held| {
					let travel = (td.time - time_held) * time_held;
					(travel > td.distance).then(|| (time_held, travel))
				}).count()
			}).product::<usize>();
		Ok(result.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let input = self.parse()?;
		let td = self.merge_time_distances(input)?;

		let result = (1..td.time).into_par_iter()
			.filter_map(|time_held| {
				let travel = (td.time - time_held) * time_held;
				(travel > td.distance).then(|| (time_held, travel))
			}).count();

		Ok(result.to_string())
	}
}
