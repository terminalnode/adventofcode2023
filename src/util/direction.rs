#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	North,
	South,
	East,
	West,
}

impl Direction {
	pub fn opposite(&self) -> Direction {
		match self {
			Direction::North => Direction::South,
			Direction::South => Direction::North,
			Direction::East => Direction::West,
			Direction::West => Direction::East,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeDirection {
	Right,
	Left,
}
