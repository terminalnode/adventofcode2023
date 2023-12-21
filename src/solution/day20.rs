use std::collections::{HashMap, VecDeque};
use ModuleType::{Broadcaster, Conjunction, FlipFlop};
use crate::solution::Solution;

pub struct Day20 {
	file: String,
}

#[derive(Debug, PartialEq)]
enum ModuleType {
	Broadcaster,
	FlipFlop(bool),
	Conjunction,
}

#[derive(Debug, PartialEq, Clone)]
enum Pulse { High, Low }

#[derive(Debug)]
struct Module {
	name: String,
	module_type: ModuleType,
	targets: Vec<String>,
}

impl Day20 {
	fn parse(&self) -> Result<HashMap<String, Module>, String> {
		self.read_file_as_string()?.lines()
			.map(|line| {
				let mut parts = line.split(" -> ");
				let name = parts.next().ok_or("No name")?.to_string();
				let targets: Vec<String> = parts.next().ok_or("No targets")?
					.split(", ")
					.map(|s| s.to_string())
					.collect::<Vec<String>>();

				let (name, module_type) = match name.chars().next() {
					Some('b') => { Ok((name, Broadcaster)) }
					Some('%') => { Ok((name[1..].to_string(), FlipFlop(false))) }
					Some('&') => { Ok((name[1..].to_string(), Conjunction)) }
					c => { Err(format!("Failed to parse module name: {line}, char was {c:?}")) }
				}?;

				let module = Module { name: name.clone(), module_type, targets };
				Ok((name, module))
			}).collect::<Result<HashMap<String, Module>, String>>()
	}
}

impl Solution for Day20 {
	fn new(file: &str) -> Self { Day20 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let mut modules = self.parse()?;
		let mut highs = 0usize;
		let mut lows = 0usize;

		let broadcaster = modules["broadcaster"].targets.iter()
			.map(|target| ("broadcaster".to_string(), Pulse::Low, target.clone()))
			.collect::<Vec<(String, Pulse, String)>>();
		let mut queue = VecDeque::from(broadcaster);

		while let Some((source, pulse, target)) = queue.pop_front() {
			println!("{source} -{:?}-> {:?}", &pulse, &target);
			if target == "output" { continue; }

			if pulse == Pulse::High { highs += 1; } else { lows += 1; }
			let module = modules.get_mut(&target).ok_or(format!("No module named {target}"))?;

			let send = match &module.module_type {
				Broadcaster => Err("Broadcaster cannot be a target".to_string())?,

				FlipFlop(is_on) => {
					if pulse == Pulse::Low {
						let pulse = if is_on == &true { Pulse::Low } else { Pulse::High };
						module.module_type = FlipFlop(!is_on);
						Some(pulse)
					} else {
						None
					}
				}

				Conjunction => {
					// let send_high = module.targets.iter()
					// 	.all(|x| &modules.get(x).unwrap().module_type == &FlipFlop(true));
					// if !send_high { Some(Pulse::High) } else { Some(Pulse::Low) }
					None
				}
			};

			if let Some(send_pulse) = send {
				for target in &module.targets {
					queue.push_back((
														module.name.clone(),
														send_pulse.clone(),
														target.clone()),
					);
				}
			}
		}

		println!("highs: {highs}");
		println!("lows: {lows}");

		Err("Not implemented".to_string())
	}
}
