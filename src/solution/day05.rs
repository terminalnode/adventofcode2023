use crate::solution::Solution;
use rayon::prelude::*;

pub struct Day05 {
	file: String,
}

#[derive(Debug)]
struct ParsedInput {
	seeds: Vec<u64>,
	seed_to_soil: Vec<RangeMapping>,
	soil_to_fertilizer: Vec<RangeMapping>,
	fertilizer_to_water: Vec<RangeMapping>,
	water_to_light: Vec<RangeMapping>,
	light_to_temperature: Vec<RangeMapping>,
	temperature_to_humidity: Vec<RangeMapping>,
	humidity_to_location: Vec<RangeMapping>,
}

#[derive(Debug)]
struct RangeMapping {
	destination_start: u64,
	source_start: u64,
	length: u64,
}

impl RangeMapping {
	fn map(&self, value: u64) -> Option<u64> {
		if value < self.source_start { return None; }
		let diff = value - self.source_start;
		if diff >= self.length { return None; }
		Some(self.destination_start + diff)
	}
}

impl Day05 {
	fn parse(&self) -> Result<ParsedInput, String> {
		let file = self.read_file_as_string()?;
		let mut lines = file.lines();

		let seeds = match lines.next() {
			None => Err("failed to parse seeds".to_string()),
			Some(s) => Ok(self.extract_numbers(s)?)
		}?;
		lines.next(); lines.next(); // skip empty line and first descriptor

		let mut skip_descriptor = false;
		let mut ranges: Vec<Vec<RangeMapping>> = Vec::new();
		let mut current_range: Vec<RangeMapping> = Vec::new();
		for line in lines {
			if skip_descriptor { skip_descriptor = false; continue; }
			if line.is_empty() {
				ranges.push(current_range);
				current_range = Vec::new();
				skip_descriptor = true;
				continue;
			}

			let numbers = self.extract_numbers(line)?;
			current_range.push(RangeMapping {
				destination_start: numbers[0],
				source_start: numbers[1],
				length: numbers[2],
			});
		}

		Ok(ParsedInput {
			seeds,
			seed_to_soil: ranges.remove(0),
			soil_to_fertilizer: ranges.remove(0),
			fertilizer_to_water: ranges.remove(0),
			water_to_light: ranges.remove(0),
			light_to_temperature: ranges.remove(0),
			temperature_to_humidity: ranges.remove(0),
			humidity_to_location: current_range,
		})
	}

	fn extract_numbers(
		&self,
		list: &str,
	) -> Result<Vec<u64>, String> {
		let numbers = list.split_whitespace()
			.filter_map(|num| num.parse::<u64>().ok())
			.collect::<Vec<u64>>();
		Ok(numbers)
	}

	fn map_with_ranges(
		&self,
		mappings: &Vec<RangeMapping>,
		value: u64,
	) -> u64 {
		mappings.iter().find_map(|mapping| {
			mapping.map(value)
		}).unwrap_or(value)
	}

	fn seed_to_location(
		&self,
		input: &ParsedInput,
		seed: u64,
	) -> u64 {
		let soil = self.map_with_ranges(&input.seed_to_soil, seed);
		let fertilizer = self.map_with_ranges(&input.soil_to_fertilizer, soil);
		let water = self.map_with_ranges(&input.fertilizer_to_water, fertilizer);
		let light = self.map_with_ranges(&input.water_to_light, water);
		let temperature = self.map_with_ranges(&input.light_to_temperature, light);
		let humidity = self.map_with_ranges(&input.temperature_to_humidity, temperature);
		self.map_with_ranges(&input.humidity_to_location, humidity)
	}
}

impl Solution for Day05 {
	fn new(file: &str) -> Self { Day05 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let input = self.parse()?;
		let min: u64 = input.seeds.iter()
			.map(|seed| self.seed_to_location(&input, *seed))
			.min()
			.ok_or("missing seed values!".to_string())?;

		Ok(min.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let input = self.parse()?;

		let mut seeds = input.seeds.clone();
		let mut seed_ranges: Vec<(u64, u64)> = Vec::new();
		while seeds.len() > 0 {
			let s1 = seeds.remove(0);
			let s2 = seeds.remove(0);
			seed_ranges.push((s1, s2));
		}

		// Hope you have a lot of cores. :)
		let min = seed_ranges.par_iter().filter_map(|(start, size)| {
			(0..*size).into_par_iter().map(|offset| {
				self.seed_to_location(&input, start + offset)
			}).min()
		}).min().ok_or("missing seed values!".to_string())?;

		Ok(min.to_string())
	}
}
