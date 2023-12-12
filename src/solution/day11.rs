use std::collections::VecDeque;
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

	fn parse_galaxies(
		&self,
		expansion: usize,
	) -> Result<GalaxyList, String> {
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

			if !found { empty_y.push(y); }
		}

		let mut found: bool = true;
		let mut xpansion: usize = 0;
		for x in 0..x_len {
			if !found { xpansion += expansion; }
			found = false;

			for y in 0..star_map.len() {
				let ypansion = empty_y.iter()
					.filter(|other| other < &&y)
					.count() * expansion;

				if star_map[y][x] == '#' {
					found = true;
					galaxies.push((x + xpansion, y + ypansion));
				}
			}
		}

		Ok(galaxies)
	}

	fn solve(
		&self,
		expansion: usize,
	) -> Result<String, String> {
		let mut galaxies = VecDeque::from(self.parse_galaxies(expansion)?);

		let mut distances = 0usize;
		while let Some(galaxy) = galaxies.pop_front() {
			galaxies.iter().for_each(|other| {
				distances += galaxy.manhattan_distance(*other);
			});
		};

		Ok(distances.to_string())
	}
}

impl Solution for Day11 {
	fn new(file: &str) -> Self { Day11 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		self.solve(1)
	}

	fn part_two(&self) -> Result<String, String> {
		// Don't understand why, but for distance of 1 you input 1.
		// For higher distances, you subtract one from the distance.
		// This goes for the example data with distance of 10 as well (10 too high, 9 correct).
		self.solve(999_999)
	}
}
