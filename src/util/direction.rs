#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
	North,
	South,
	East,
	West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeDirection {
	Right,
	Left,
}
