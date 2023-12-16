use crate::util::Direction;

pub type Point2D = (usize, usize);

pub trait Point2DExt {
	fn x(&self) -> usize;
	fn y(&self) -> usize;

	fn move_dir(
		&self,
		direction: &Direction,
	) -> Option<Point2D> {
		match direction {
			Direction::North => self.north(),
			Direction::South => self.south(),
			Direction::East => self.east(),
			Direction::West => self.west(),
		}
	}

	fn move_opt_dir(
		&self,
		direction: Option<Direction>,
	) -> Option<Point2D> {
		match direction {
			Some(x) => self.move_dir(&x),
			None => None,
		}
	}

	fn north(&self) -> Option<Point2D> {
		if self.y() > 0 {
			Some((self.x(), self.y() - 1))
		} else {
			None
		}
	}

	fn west(&self) -> Option<Point2D> {
		if self.x() > 0 {
			Some((self.x() - 1, self.y()))
		} else {
			None
		}
	}

	fn manhattan_distance(
		&self,
		end: Point2D,
	) -> usize {
		let x_diff = self.x().abs_diff(end.x());
		let y_diff = self.y().abs_diff(end.y());
		x_diff + y_diff
	}

	fn south(&self) -> Option<Point2D> { Some((self.x(), self.y() + 1)) }
	fn east(&self) -> Option<Point2D> { Some((self.x() + 1, self.y())) }

	fn up(&self) -> Option<Point2D> { self.north() }
	fn down(&self) -> Option<Point2D> { self.south() }
	fn left(&self) -> Option<Point2D> { self.west() }
	fn right(&self) -> Option<Point2D> { self.east() }

	fn in_grid(
		&self,
		max_x: usize,
		max_y: usize,
	) -> bool {
		self.x() <= max_x && self.y() <= max_y
	}
}

impl Point2DExt for Point2D {
	fn x(&self) -> usize { self.0 }
	fn y(&self) -> usize { self.1 }
}
