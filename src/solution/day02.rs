use crate::solution::Solution;

pub struct Day02 { file: String }
struct Game { id: usize, reveals: Vec<ColorCount> }
struct ColorCount { red: usize, green: usize, blue: usize }

impl Game {
	fn is_possible(
		&self,
		other: &ColorCount,
	) -> bool {
		self.reveals.iter().all(|reveal| reveal.is_possible(other))
	}
}

impl ColorCount {
	fn is_possible(
		&self,
		other: &ColorCount,
	) -> bool {
		self.blue <= other.blue
			&& self.red <= other.red
			&& self.green <= other.green
	}
}

impl Day02 {
	fn parse_input(&self) -> Result<Vec<Game>, String> {
		let games: Vec<Game> = self.read_file_as_string()?.lines()
			.filter_map(|line| self.parse_game(line).ok())
			.collect();
		Ok(games)
	}

	fn parse_game(&self, line: &str) -> Result<Game, String> {
		let mut split = line.split(": ");
		let id: usize = split.next()
			.ok_or("Failed to get game id".to_string())?
			.replace("Game ", "")
			.parse()
			.map_err(|_| "Failed to parse game id".to_string())?;

		let reveals = split.next()
			.ok_or("Failed to get reveals".to_string())?
			.split("; ")
			.into_iter()
			.filter_map(|reveal| self.parse_reveal(reveal).ok())
			.collect::<Vec<ColorCount>>();

		Ok(Game { id, reveals })
	}

	fn parse_reveal(&self, line: &str) -> Result<ColorCount, String> {
		let mut red = 0;
		let mut green = 0;
		let mut blue = 0;

		for part in line.split(", ") {
			let mut subs = part.split(" ");
			let count = subs.next()
				.ok_or(format!("Failed to get count from part {part}"))?
				.parse::<usize>()
				.or_else(|_| Err("Failed to parse count".to_string()))?;

			match subs.next() {
				Some("blue") => blue += count,
				Some("red") => red += count,
				Some("green") => green += count,
				_ => return Err(format!("Failed to parse color from part {part}")),
			};
		}

		Ok(ColorCount { blue, red, green })
	}
}

impl Solution for Day02 {
	fn new(file: &str) -> Self { Day02 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let cube_count = ColorCount { blue: 14, red: 12, green: 13 };
		let result = self.parse_input()?.iter()
			.filter(|game| game.is_possible(&cube_count))
			.map(|game| game.id)
			.reduce(|a, b| a + b)
			.ok_or("Failed to sum game ids".to_string())?;
		Ok(format!("Sum of possible game ids: {result}"))
	}

	fn part_two(&self) -> Result<String, String> {
		let result = self.parse_input()?.iter().map(|game| {
			let mut red = 0;
			let mut green = 0;
			let mut blue = 0;
			for reveal in &game.reveals {
				if reveal.red > red { red = reveal.red }
				if reveal.green > green { green = reveal.green }
				if reveal.blue > blue { blue = reveal.blue }
			}

			red * green * blue
		}).reduce(|a, b| a + b).ok_or("Failed to sum game powers".to_string())?;

		Ok(format!("Sum of game powers: {result}"))
	}
}
