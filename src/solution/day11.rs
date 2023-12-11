use std::collections::{HashSet, VecDeque};
use crate::solution::Solution;
use crate::util::{Point2D, Point2DExt};

pub struct Day11 {
	file: String,
}

type RawStarMap = Vec<Vec<char>>;
type GalaxyList = Vec<Point2D>;

impl Day11 {
	fn parse(&self) -> Result<RawStarMap, String> {
		let map: RawStarMap = self.read_file_as_string()?.lines()
			.map(|line| line.chars().collect())
			.collect();
		Ok(map)
	}

	fn parse_galaxies(&self) -> Result<GalaxyList, String> {
		let star_map = self.parse()?;
		let mut galaxies: GalaxyList = Vec::new();
		let y_len = star_map.len();
		let x_len = star_map[0].len();

		let mut empty_y = Vec::<usize>::new();
		for y in 0..y_len {
			let mut found = false;
			for x in 0..x_len {
				if star_map[y][x] == '#' {
					found = true;
					break;
				}
			}
			if !found {
				println!("Triggered ypansion on y={y}");
				empty_y.push(y + empty_y.len());
			}
		}
		println!("---");

		let mut found: bool;
		let mut xpansion: usize = 0;
		let mut ypansion: usize;
		for x in 0..x_len {
			found = false;
			ypansion = 0;

			for y in 0..star_map.len() {
				if empty_y.contains(&y) { ypansion += 1; }

				if star_map[y][x] == '#' {
					found = true;
					println!("Found galaxy at ({}, {})", x + xpansion, y + ypansion);
					galaxies.push((x + xpansion, y + ypansion));
				}
			}

			if !found {
				println!("Triggered xpansion on x={x} ({})", x+xpansion);
				xpansion += 1;
			}
		}

		Ok(galaxies)
	}
}

fn manhattan_distance(
	start: Point2D,
	end: Point2D,
) -> usize {
	let x_diff = start.x().abs_diff(end.x());
	let y_diff = start.y().abs_diff(end.y());
	x_diff + y_diff
}

impl Solution for Day11 {
	fn new(file: &str) -> Self { Day11 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let mut galaxies = VecDeque::from(self.parse_galaxies()?);

		let mut pairs = 0usize;
		let mut distances = 0usize;
		while let Some(galaxy) = galaxies.pop_front() {
			galaxies.iter().for_each(|other| {
				pairs += 1;
				distances += manhattan_distance(galaxy, *other);
			});
		};

		Err(distances.to_string())
	}
}
