use std::collections::{HashMap, HashSet};
use RelativeDirection::{Left, Right};
use crate::solution::Solution;
use crate::util::RelativeDirection;

pub struct Day08 {
	file: String,
}

#[derive(Debug)]
struct InputData {
	directions: Vec<RelativeDirection>,
	map: HashMap<String, (String, String)>,
	last_chars: HashMap<String, char>,
}

impl Day08 {
	fn parse(&self) -> Result<InputData, String> {
		let input = self.read_file_as_string()?;
		let mut lines = input.lines();
		let mut directions = Vec::new();
		let mut map = HashMap::new();
		let mut last_chars = HashMap::new();

		let rl_chars = lines.next().ok_or("failed to extract right/left chars")?.chars();
		for c in rl_chars {
			match c {
				'R' => directions.push(Right),
				'L' => directions.push(Left),
				_ => return Err(format!("invalid direction: {c}")),
			}
		}
		lines.next(); // drop empty line

		for line in lines {
			let mut eq_split = line.split(" = ");
			let position = eq_split.next()
				.ok_or(format!("failed to extract position split from {line}"))?
				.to_string();

			let lr_part = eq_split.next()
				.ok_or(format!("failed to extract right/left split from {line}"))?;
			let lr_split = lr_part.split(", ").collect::<Vec<&str>>();

			let left = lr_split[0][1..].to_string();
			let right = lr_split[1][0..3].to_string();

			map.insert(position, (left, right));
		}

		for position in map.keys() {
			let last_char = position.chars().last()
				.ok_or(format!("failed to extract last char from {position}"))?;
			last_chars.insert(position.to_string(), last_char);
		}

		Ok(InputData { directions, map, last_chars })
	}
}

impl Solution for Day08 {
	fn new(file: &str) -> Self { Day08 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let input = self.parse()?;
		let mut steps = 0;
		let mut position = "AAA";
		for direction in input.directions.iter().cycle() {
			steps += 1;
			let (left, right) = &input.map[position];
			match direction {
				Right => position = &right,
				Left => position = &left,
			}

			if position == "ZZZ" { break; }
		}

		Ok(steps.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let input = self.parse()?;
		let positions = input.last_chars.iter()
			.filter(|(_, &last_char)| last_char == 'A')
			.map(|(position, _)| position.clone())
			.collect::<Vec<String>>();

		let paths = positions.iter().map(|start| {
			let mut steps = 0;
			let mut path = Vec::new();
			let mut end_positions = HashSet::new();
			let mut winners = Vec::new();
			let mut curr = start.clone();

			loop {
				for direction in input.directions.iter() {
					steps += 1;
					let (left, right) = &input.map[&curr];
					curr = match direction {
						Right => right.clone(),
						Left => left.clone(),
					};

					if input.last_chars[&curr] == 'Z' { winners.push(steps) }
					path.push(curr.clone());
				}

				if end_positions.contains(&curr) { break }
				end_positions.insert(curr.clone());
			}

			let first_occ = path.iter().position(|p| p == &curr).unwrap();
			winners.iter()
				.filter(|&w| *w > first_occ)
				.map(|&x| x)
				.collect::<Vec<usize>>()
		});

		// doesn't work for test input, but works for real input lol
		let lcm_result = paths.clone().flatten()
			.into_iter()
			.reduce(|a, b| lcm(a, b))
			.unwrap();

		Ok(lcm_result.to_string())
	}
}

fn gcd(a: usize, b: usize) -> usize {
	if b == 0 {
		a
	} else {
		gcd(b, a % b)
	}
}

fn lcm(a: usize, b: usize) -> usize {
	a * b / gcd(a, b)
}
