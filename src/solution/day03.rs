use std::collections::HashMap;
use crate::solution::Solution;

pub struct Day03 {
	file: String,
}

type CharMap = HashMap<(usize, usize), char>;
type Point = (usize, usize);

impl Day03 {
	fn parse_input(&self) -> Result<(CharMap, CharMap), String> {
		let mut char_map = HashMap::new();
		let mut num_map = HashMap::new();
		let mut x: usize = 0;
		let mut y: usize = 0;

		self.read_file_as_string()?.lines()
			.for_each(|line| {
				line.chars().for_each(|char| {
					if char.is_digit(10) {
						num_map.insert((x, y), char);
					} else if char != '.' {
						char_map.insert((x, y), char);
					}
					x += 1;
				});
				y += 1;
				x = 0;
			});

		Ok((char_map, num_map))
	}

	fn get_horizontal_number(
		&self,
		point: &Point,
		num_map: &CharMap,
	) -> Option<(Point, usize)> {
		let (mut x, y) = point;
		let y = y + 0; // copy y
		if !num_map.contains_key(&(x, y)) { return None; }

		// Find start.. lol
		loop {
			if x == 0 { break; }
			x -= 1;
			if !num_map.contains_key(&(x, y)) {
				x += 1;
				break;
			}
		};

		// Find digits
		let number = (x..)
			.map_while(|dx| num_map.get(&(dx, y)).map(|x| x.to_string()))
			.collect::<String>()
			.parse::<usize>()
			.ok()?;

		Some(((x, y), number))
	}
}

impl Solution for Day03 {
	fn new(file: &str) -> Self { Day03 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let (char_map, num_map) = self.parse_input()?;
		let mut result_map = HashMap::new();

		char_map.keys().for_each(|(x, y)| {
			let x = *x;
			let y = *y;
			vec![
				(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
				(x - 1, y), (x + 1, y),
				(x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
			].iter().for_each(|point| {
				self.get_horizontal_number(point, &num_map)
					.and_then(|(p, v)| result_map.insert(p, v))
				;
			});
		});

		Ok(result_map.values().sum::<usize>().to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let (char_map, num_map) = self.parse_input()?;
		let mut sum = 0;

		char_map.iter()
			.filter(|(_, &c)| { c == '*' })
			.for_each(|((x, y), _)| {
				let x = *x;
				let y = *y;
				let mut result_map = HashMap::new();
				vec![
					(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
					(x - 1, y), (x + 1, y),
					(x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
				].iter().for_each(|point| {
					self.get_horizontal_number(point, &num_map)
						.and_then(|(p, v)| result_map.insert(p, v));
				});

				if result_map.len() == 2 {
					sum += result_map.values().product::<usize>();
				}
			});

		Ok(sum.to_string())
	}
}
