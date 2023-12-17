use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use Direction::{East, North, South, West};

use crate::solution::Solution;
use crate::util::{Direction, Matrix2D, Matrix2DExt, Point2D, Point2DExt};

pub struct Day17 {
	file: String,
}

type Grid = Matrix2D<usize>;

#[derive(Debug, Eq)]
struct Walker {
	pos: Point2D,
	dir: Direction,
	steps: usize,
	cost: usize,
	suboptimal_steps: usize,
}

impl PartialEq<Self> for Walker {
	fn eq(&self, other: &Self) -> bool { self.cost.eq(&other.cost) }
}

impl PartialOrd<Self> for Walker {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Walker {
	fn cmp(&self, other: &Self) -> Ordering { other.cost.cmp(&self.cost) }
}

impl Walker {
	fn walk(
		&self,
		dir: Direction,
		grid: &Grid,
		min_steps: usize,
		max_steps: usize,
	) -> Option<Walker> {
		if dir == self.dir && self.steps == max_steps {
			return None;
		} else if dir != self.dir && self.steps < min_steps {
			return None;
		}

		match self.pos.move_dir(&dir) {
			Some(pos) if pos.in_matrix(&grid) => {
				let cost = self.cost + *grid.get_point(pos).unwrap();
				let steps = if dir == self.dir { self.steps + 1 } else { 1 };
				Some(Walker { pos, dir, steps, cost, suboptimal_steps: self.suboptimal_steps })
			}
			_ => None,
		}
	}
}

impl Day17 {
	fn parse(&self) -> Result<Grid, String> {
		Ok(self.read_file_as_string()?.lines().into_iter()
			.map(|line| {
				line.chars()
					.filter_map(|c| c.to_digit(10))
					.map(|d| d as usize)
					.collect::<Vec<usize>>()
			}).collect())
	}
}

fn cheapest_path(
	grid: &Grid,
	min_steps: usize,
	max_steps: usize,
) -> usize {
	let goal: Point2D = (grid.x_len() - 1, grid.y_len() - 1);
	let all_dir = vec![North, South, East, West];

	let mut cost_map = HashMap::new();
	cost_map.insert((0,0), 0);

	let w1 = Walker { pos: (0, 1), dir: South, steps: 1, cost: grid[1][0], suboptimal_steps: 0 };
	let w2 = Walker { pos: (1, 0), dir: East, steps: 1, cost: grid[0][1], suboptimal_steps: 0 };
	let mut walkers = BinaryHeap::from(vec![w1, w2]);

	let mut best_goal = usize::MAX;

	while let Some(walker) = walkers.pop() {
		let new_walkers = all_dir.iter()
			.filter(|dir| dir != &&walker.dir.opposite())
			.filter_map(|&dir| walker.walk(dir, &grid, min_steps, max_steps));

		for mut new_walker in new_walkers {
			if new_walker.cost >= best_goal { continue; }

			if let Some(cost) = cost_map.get(&new_walker.pos) {
				if new_walker.cost >= *cost {
					if new_walker.suboptimal_steps >= 4 {
						continue;
					} else {
						new_walker.suboptimal_steps += 1;
					}
				} else {
					new_walker.suboptimal_steps = 0;
					cost_map.insert(new_walker.pos, new_walker.cost);
				}
			} else {
				new_walker.suboptimal_steps = 0;
				cost_map.insert(new_walker.pos, new_walker.cost);
			}

			if new_walker.pos == goal {
				if new_walker.cost < best_goal { best_goal = new_walker.cost; }
				continue;
			} else {
				walkers.push(new_walker);
			}
		}
	}

	cost_map[&goal]
}

impl Solution for Day17 {
	fn new(file: &str) -> Self { Day17 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let grid = self.parse()?;
		let cheapest = cheapest_path(&grid, 1, 3);
		Ok(cheapest.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let grid = self.parse()?;
		let cheapest = cheapest_path(&grid, 4, 10);
		Ok(cheapest.to_string())
	}
}
