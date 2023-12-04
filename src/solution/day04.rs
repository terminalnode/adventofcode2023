use std::collections::HashSet;
use crate::solution::Solution;

pub struct Day04 {
	file: String,
}

struct Card {
	id: usize,
	winners: HashSet<usize>,
	tickets: HashSet<usize>,
}

impl Day04 {
	fn parse(&self) -> Result<Vec<Card>, String> {
		let cards: Vec<Card> = self.read_file_as_string()?.lines().filter_map(|line| {
			let mut card_split = line.split(": ");
			let card_num = card_split.next()?[5..].trim_start().parse::<usize>().ok()?;

			let mut list_split = card_split.next()?.split("|");
			let winners_str = list_split.next()?;
			let tickets_str = list_split.next()?;

			Some(Card {
				id: card_num,
				winners: self.extract_numbers(winners_str).ok()?,
				tickets: self.extract_numbers(tickets_str).ok()?,
			})
		}).collect();

		Ok(cards)
	}

	fn extract_numbers(
		&self,
		list: &str,
	) -> Result<HashSet<usize>, String> {
		let numbers = list.split_whitespace()
			.filter_map(|num| num.parse::<usize>().ok())
			.collect::<HashSet<usize>>();
		Ok(numbers)
	}
}

impl Solution for Day04 {
	fn new(file: &str) -> Self { Day04 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let result = self.parse()?.iter().filter_map(|card| {
			let wins = card.tickets.intersection(&card.winners).count() as u32;
			if wins > 0 { Some(2usize.pow(wins - 1)) } else { None }
		}).sum::<usize>();
		Ok(result.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let cards = self.parse()?;
		let mut card_count = (0..cards.len()).map(|_| 1).collect::<Vec<usize>>();
		let mut result = 0;
		cards.iter().for_each(|card| {
			let wins = card.tickets.intersection(&card.winners).count();
			let count = card_count[card.id - 1];
			result += count;
			(0..wins).for_each(|i| card_count[card.id + i] += count);
		});

		Ok(result.to_string())
	}
}
