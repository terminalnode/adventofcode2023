use std::collections::{HashMap, HashSet, VecDeque};
use crate::solution::Solution;
use crate::util::{Point2D, Point2DExt};

pub struct Day10 {
	file: String,
}

type Map = HashMap<Point2D, HashSet<Point2D>>;

fn multi_connect(
	map: &mut Map,
	origin: Point2D,
	p1: Option<Point2D>,
	p2: Option<Point2D>,
	max_x: usize,
	max_y: usize,
) {
	connect(map, origin, p1, max_x, max_y);
	connect(map, origin, p2, max_x, max_y);
}

fn connect(
	map: &mut Map,
	origin: Point2D,
	target: Option<Point2D>,
	max_x: usize,
	max_y: usize,
) {
	if let Some(target) = target.filter(|p| p.in_grid(max_x, max_y)) {
		map.entry(origin).or_insert(HashSet::new()).insert(target);
		map.entry(target).or_insert(HashSet::new()).insert(origin);
	}
}

impl Day10 {
	fn parse(&self) -> Result<(Point2D, Map), String> {
		let mut option_start = None;
		let mut map = HashMap::new();
		let raw_map: Vec<Vec<char>> = self.read_file_as_string()?.lines()
			.map(|line| line.chars().collect())
			.collect();
		let max_x = raw_map[0].len() - 1;
		let max_y = raw_map.len() - 1;

		for y in 0..=max_y {
			for x in 0..=max_x {
				let here: Point2D = (x, y);
				match raw_map[y][x] {
					'|' => multi_connect(&mut map, here, here.north(), here.south(), max_x, max_y),
					'-' => multi_connect(&mut map, here, here.east(), here.west(), max_x, max_y),
					'L' => multi_connect(&mut map, here, here.north(), here.east(), max_x, max_y),
					'J' => multi_connect(&mut map, here, here.north(), here.west(), max_x, max_y),
					'7' => multi_connect(&mut map, here, here.south(), here.west(), max_x, max_y),
					'F' => multi_connect(&mut map, here, here.south(), here.east(), max_x, max_y),
					'S' => { option_start = Some(here) }
					'.' => (/* ignore ground */),
					c => println!("Unknown char: {c}"),
				}
			}
		}

		let start = option_start.ok_or(format!("Could not find starting position"))?;
		Ok((start, map))
	}
}

impl Solution for Day10 {
	fn new(file: &str) -> Self { Day10 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let (start, map) = self.parse()?;
		let mut max = 0;
		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();
		queue.push_back((0usize, start));
		visited.insert(start);

		while let Some((step, point)) = queue.pop_front() {
			let connections = match map.get(&point) {
				None => continue,
				Some(x) => x,
			};

			let next_step = step + 1;
			for connection in connections {
				if !visited.contains(&connection) {
					max = max.max(next_step);
					queue.push_back((next_step, *connection));
					visited.insert(*connection);
				}
			}
		}

		Ok(max.to_string())
	}
}
