use std::ops::Range;
use crate::util::{Point2D, Point2DExt};

pub type Matrix2D<T> = Vec<Vec<T>>;

pub trait Matrix2DExt<T> where T: PartialEq {
	fn get_xy(&self, x: usize, y: usize) -> Option<&T>;
	fn set_xy(&mut self, x: usize, y: usize, value: T);
	fn get_row(&self, y: usize) -> Option<&Vec<T>>;
	fn x_len(&self) -> usize;
	fn y_len(&self) -> usize;

	fn x_range(&self) -> Range<usize> { 0..self.x_len() }
	fn y_range(&self) -> Range<usize> { 0..self.y_len() }

	fn get_point(&self, point: Point2D) -> Option<&T> {
		self.get_xy(point.x(), point.y())
	}

	fn set_point(&mut self, point: Point2D, value: T) {
		self.set_xy(point.x(), point.y(), value);
	}

	fn find_all(&self, value: &T) -> Vec<Point2D> {
		let mut points = vec![];
		for y in self.y_range() {
			for x in self.x_range() {
				if self.get_xy(x, y) == Some(value) {
					points.push((x, y));
				}
			}
		}

		points
	}
}

impl<T> Matrix2DExt<T> for Matrix2D<T> where T: PartialEq {
	fn get_xy(&self, x: usize, y: usize) -> Option<&T> {
		if let Some(row) = self.get(y) {
			row.get(x)
		} else {
			None
		}
	}

	fn set_xy(&mut self, x: usize, y: usize, value: T) {
		if let Some(row) = self.get_mut(y) {
			if let Some(cell) = row.get_mut(x) {
				*cell = value;
			}
		}
	}

	fn get_row(&self, y: usize) -> Option<&Vec<T>> { self.get(y) }
	fn x_len(&self) -> usize { self[0].len() }

	fn y_len(&self) -> usize { self.len() }
}
