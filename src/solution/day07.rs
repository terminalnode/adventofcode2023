use std::cmp::Ordering;
use std::collections::HashMap;
use crate::solution::day07::HandType::*;
use crate::solution::Solution;

pub struct Day07 {
	file: String,
}

impl Day07 {}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Copy, Clone)]
enum Card { X2, X3, X4, X5, X6, X7, X8, X9, T, J, Q, K, A }

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType { HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind }

fn char_to_card(c: char) -> Result<Card, String> {
	match c {
		'2' => Ok(Card::X2),
		'3' => Ok(Card::X3),
		'4' => Ok(Card::X4),
		'5' => Ok(Card::X5),
		'6' => Ok(Card::X6),
		'7' => Ok(Card::X7),
		'8' => Ok(Card::X8),
		'9' => Ok(Card::X9),
		'T' => Ok(Card::T),
		'J' => Ok(Card::J),
		'Q' => Ok(Card::Q),
		'K' => Ok(Card::K),
		'A' => Ok(Card::A),
		_ => Err(format!("Invalid card: {c}")),
	}
}

#[derive(Debug)]
struct Hand {
	cards: Vec<Card>,
	hand_type: HandType,
	bid: usize,
}

fn cards_to_type(cards: &Vec<Card>) -> Result<HandType, String> {
	let mut map = HashMap::<Card, usize>::new();
	cards.iter().for_each(|&card| {
		*map.entry(card).or_insert(0) += 1;
	});

	let mut groupings = map.values().collect::<Vec<&usize>>();
	groupings.sort();

	match groupings.as_slice() {
		[5] => Ok(FiveOfAKind),
		[1, 4] => Ok(FourOfAKind),
		[2, 3] => Ok(FullHouse),
		[1, 1, 3] => Ok(ThreeOfAKind),
		[1, 2, 2] => Ok(TwoPair),
		[1, 1, 1, 2] => Ok(OnePair),
		[1, 1, 1, 1, 1] => Ok(HighCard),
		_ => Err(format!("Invalid hand: {cards:?}")),
	}
}

impl Day07 {
	fn parse(&self) -> Result<Vec<Hand>, String> {
		let hand = self.read_file_as_string()?.lines().filter_map(|line| {
			let mut line_split = line.split(" ");
			let cards = line_split.next()?.chars()
				.map(|c| char_to_card(c))
				.collect::<Result<Vec<Card>, String>>()
				.ok()?;
			let bid = line_split.next()?.parse::<usize>().ok()?;
			let hand_type = cards_to_type(&cards).ok()?;

			Some(Hand { cards, hand_type, bid })
		}).collect::<Vec<Hand>>();
		Ok(hand)
	}
}

impl Solution for Day07 {
	fn new(file: &str) -> Self { Day07 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let mut hands = self.parse()?;

		hands.sort_by(|a, b| {
			if a.hand_type > b.hand_type {
				Ordering::Greater
			} else if a.hand_type < b.hand_type {
				Ordering::Less
			} else {
				a.cards.iter().zip(&b.cards).filter_map(|(a, b)| {
					if a > b { Some(Ordering::Greater) }
					else if a < b { Some(Ordering::Less) }
					else { None }
				}).next().unwrap_or(Ordering::Equal)
			}
		});

		let mut out = 0;
		for (i, hand) in hands.iter().enumerate() {
			out += hand.bid * (i + 1);
		}
		Ok(out.to_string())
	}
}
