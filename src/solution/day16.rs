use std::collections::{HashMap, HashSet, VecDeque};

use Direction::{East, North, South, West};

use crate::solution::Solution;
use crate::util::{Direction, Point2D, Point2DExt};

pub struct Day16 {
	file: String,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Ray {
	pos: Point2D,
	dir: Direction,
}

impl Ray {
	fn new(
		&self,
		dir: Option<Direction>,
		max_x: usize,
		max_y: usize,
	) -> Option<Self> {
		let dir = dir?;
		let pos = self.pos.move_dir(&dir).filter(|x| x.in_grid(max_x, max_y))?;
		Some(Ray { pos, dir })
	}
}

#[derive(Debug)]
struct Grid {
	map: HashMap<Point2D, char>,
	max_x: usize,
	max_y: usize,
}

impl Day16 {
	fn parse(&self) -> Result<Grid, String> {
		let file = self.read_file_as_string()?;
		let mut map = HashMap::new();
		let mut max_x = 0;
		let mut max_y = 0;
		for (y, line) in file.lines().enumerate() {
			max_y = y;
			for (x, c) in line.chars().enumerate() {
				max_x = x;
				if c == '.' { continue; }
				map.insert((x, y), c);
			}
		}

		Ok(Grid { map, max_x, max_y })
	}
}

impl Solution for Day16 {
	fn new(file: &str) -> Self { Day16 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let grid = self.parse()?;
		let start = Ray { pos: (0, 0), dir: East };
		Ok(solve(&grid, start)?.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let grid = self.parse()?;

		let mut max = 0;
		for x in 0..=grid.max_x {
			for y in 0..=grid.max_y {
				if x == 0 { max = max.max(solve(&grid, Ray { pos: (x, y), dir: East })?); }
				if x == grid.max_x { max = max.max(solve(&grid, Ray { pos: (x, y), dir: West })?); }
				if y == 0 { max = max.max(solve(&grid, Ray { pos: (x, y), dir: South })?); }
				if y == grid.max_y { max = max.max(solve(&grid, Ray { pos: (x, y), dir: North })?); }
			}
		}

		Ok(max.to_string())
	}
}

fn solve(
	grid: &Grid,
	start: Ray,
) -> Result<usize, String> {
	let mut rays = HashSet::new();
	rays.insert(start.clone());

	let mut queue = VecDeque::from(vec![start]);

	while let Some(ray) = queue.pop_front() {
		let (r1, r2) = move_ray(ray, &grid.map, grid.max_x, grid.max_y);

		if let Some(r1) = r1 {
			if !rays.contains(&r1) {
				rays.insert(r1.clone());
				queue.push_back(r1);
			}
		}

		if let Some(r2) = r2 {
			if !rays.contains(&r2) {
				rays.insert(r2.clone());
				queue.push_back(r2);
			}
		}
	}

	Ok(rays.iter().map(|p| p.pos).collect::<HashSet<Point2D>>().len())
}

fn move_ray(
	ray: Ray,
	map: &HashMap<Point2D, char>,
	max_x: usize,
	max_y: usize,
) -> (Option<Ray>, Option<Ray>) {
	let (dir1, dir2) = match map.get(&ray.pos) {
		None => (Some(ray.dir), None),
		Some('/') => match ray.dir {
			North => (Some(East), None),
			South => (Some(West), None),
			East => (Some(North), None),
			West => (Some(South), None),
		},
		Some('\\') => match ray.dir {
			North => (Some(West), None),
			South => (Some(East), None),
			East => (Some(South), None),
			West => (Some(North), None),
		},
		Some('-') => match ray.dir {
			North | South => (Some(East), Some(West)),
			dir => (Some(dir), None),
		},
		Some('|') => match ray.dir {
			East | West => (Some(North), Some(South)),
			dir => (Some(dir), None),
		}
		Some(char) => panic!("Unknown char: {}", char),
	};

	(ray.new(dir1, max_x, max_y), ray.new(dir2, max_x, max_y))
}
