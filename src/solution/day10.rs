use std::collections::{HashMap, HashSet, VecDeque};
use std::thread::current;
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
	}
}

struct MapInfo {
	map: Map,
	grid: Vec<Vec<char>>,
	start: Point2D,
	max_x: usize,
	max_y: usize,
}

#[derive(Debug, PartialEq)]
enum Direction { North, South, East, West }

impl MapInfo {
	fn get_char(&self, point: Option<Point2D>) -> Option<char> {
		if let Some(p) = point {
			if p.in_grid(self.max_x, self.max_y) {
				Some(self.grid[p.y()][p.x()])
			} else {
				None
			}
		} else {
			None
		}
	}
}

impl Day10 {
	fn parse(&self) -> Result<MapInfo, String> {
		let mut option_start = None;
		let mut unfiltered_map = HashMap::new();
		let grid: Vec<Vec<char>> = self.read_file_as_string()?.lines()
			.map(|line| line.chars().collect())
			.collect();
		let max_x = grid[0].len() - 1;
		let max_y = grid.len() - 1;

		for y in 0..=max_y {
			for x in 0..=max_x {
				let here: Point2D = (x, y);
				match grid[y][x] {
					'|' => multi_connect(&mut unfiltered_map, here, here.north(), here.south(), max_x, max_y),
					'-' => multi_connect(&mut unfiltered_map, here, here.east(), here.west(), max_x, max_y),
					'L' => multi_connect(&mut unfiltered_map, here, here.north(), here.east(), max_x, max_y),
					'J' => multi_connect(&mut unfiltered_map, here, here.north(), here.west(), max_x, max_y),
					'7' => multi_connect(&mut unfiltered_map, here, here.south(), here.west(), max_x, max_y),
					'F' => multi_connect(&mut unfiltered_map, here, here.south(), here.east(), max_x, max_y),
					'S' => {
						option_start = Some(here);
						multi_connect(&mut unfiltered_map, here, here.south(), here.north(), max_x, max_y);
						multi_connect(&mut unfiltered_map, here, here.east(), here.west(), max_x, max_y);
					}
					'.' => (/* ignore ground */),
					c => println!("Unknown char: {c}"),
				}
			}
		}

		// Remove connections that are not bidirectional
		// Very annoying to do in-place with Rust, and parsing is only done once #yolo
		let mut map = HashMap::new();
		for key in unfiltered_map.keys() {
			let connections = unfiltered_map.get(key).unwrap();
			let new_connections = connections.iter().copied().filter(|connection| {
				match unfiltered_map.get(connection) {
					Some(cs) => cs.contains(key),
					None => false,
				}
			}).collect();
			map.insert(*key, new_connections);
		}

		let start = option_start.ok_or(format!("Could not find starting position"))?;

		Ok(MapInfo { map, grid, start, max_x, max_y })
	}
}

impl Solution for Day10 {
	fn new(file: &str) -> Self { Day10 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let info = self.parse()?;
		let start = info.start;
		let map = info.map;

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

	fn part_two(&self) -> Result<String, String> {
		let info2 = self.parse()?;
		let info = self.parse()?;
		let start = info.start;
		let map = info.map;

		let mut visited = HashSet::new();
		let mut queue = VecDeque::new();
		queue.push_back(start);
		visited.insert(start);

		while let Some(point) = queue.pop_front() {
			let connections = match map.get(&point) {
				None => continue,
				Some(x) => x,
			};

			for connection in connections {
				if !visited.contains(&connection) {
					queue.push_back(*connection);
					visited.insert(*connection);
				}
			}
		}

		let mut revisited = HashSet::new();
		let mut side1 = HashSet::new();
		let mut side2 = HashSet::new();
		let mut direction = Direction::North;
		let mut current = visited.clone().into_iter()
			.find(|(x, y)| info2.get_char(Some((*x, *y))) == Some('|'))
			.ok_or(format!("Could not find starting position"))?;

		println!("Start: {current:?}");
		while !revisited.contains(&current) {
			let curr_char = info2.get_char(Some(current)).unwrap();
			println!("Coming from {direction:?} into {curr_char} at {current:?}");

			let (new_side1, new_side2) = match curr_char {
				'|' => match direction {
					Direction::North => (vec![current.west()], vec![current.east()]),
					Direction::South => (vec![current.east()], vec![current.west()]),
					x => return Err(format!("impossible | ({x:?})")),
				}
				'-' => match direction {
					Direction::East => (vec![current.north()], vec![current.south()]),
					Direction::West => (vec![current.south()], vec![current.north()]),
					x => return Err(format!("impossible - ({x:?})")),
				}
				'F' => match direction {
					Direction::West => (vec![current.west(), current.north()], vec![]),
					Direction::North => (vec![], vec![current.west(), current.north()]),
					x => return Err(format!("impossible F ({x:?})")),
				}
				'J' => match direction {
					Direction::South => (vec![], vec![current.east(), current.south()]),
					Direction::East => (vec![current.east(), current.south()], vec![]),
					x => return Err(format!("impossible J ({x:?})")),
				}
				'L' => match direction {
					Direction::South => (vec![current.south(), current.west()], vec![]),
					Direction::West => (vec![], vec![current.south(), current.west()]),
					x => return Err(format!("impossible L ({x:?})")),
				}
				'7' => match direction {
					Direction::North => (vec![], vec![current.north(), current.east()]),
					Direction::East => (vec![current.north(), current.east()], vec![]),
					x => return Err(format!("impossible 7 ({x:?})")),
				}
				'S' => (vec![], vec![]),
				c => return Err(format!("impossible current pos ({c})")),
			};
			side1.extend(new_side1.iter().filter_map(|x| *x));
			side2.extend(new_side2.iter().filter_map(|x| *x));

			direction = match curr_char {
				// TODO infer S, until then... manually edit how S is handled
				'S' => match direction {
					Direction::West => Direction::South,
					Direction::North => Direction::East,
					x => return Err(format!("Can not go into S from {x:?} at {current:?}")),
				}

				'J' => match direction {
					Direction::South => Direction::West,
					Direction::East => Direction::North,
					x => return Err(format!("Can not go into J from {x:?} at {current:?}")),
				}

				'F' => match direction {
					Direction::North => Direction::East,
					Direction::West => Direction::South,
					x => return Err(format!("Can not go into F from {x:?} at {current:?}")),
				}

				'L' => match direction {
					Direction::South => Direction::East,
					Direction::West => Direction::North,
					x => return Err(format!("Can not go into L from {x:?} at {current:?}")),
				}

				'7' =>
					match direction {
						Direction::North => Direction::West,
						Direction::East => Direction::South,
						x => return Err(format!("Can not go into 7 from {x:?} at {current:?}")),
					}

				'-' | '|' => direction,
				x => return Err(format!("Went {direction:?}, hit {x} and died")),
			};

			revisited.insert(current);
			current = match direction {
				Direction::North => current.north(),
				Direction::South => current.south(),
				Direction::East => current.east(),
				Direction::West => current.west(),
			}.unwrap();
		}
		let s1: usize = side1.iter()
			.filter(|&&x| info2.get_char(Some(x)) == Some('.'))
			.count();
		let s2: usize = side2.iter()
			.filter(|&&x| info2.get_char(Some(x)) == Some('.'))
			.count();

		Ok(format!("One of these (probably the smaller): {} / {}", s1, s2))
	}
}
