use std::collections::HashMap;

use regex::Regex;

use crate::solution::Solution;

pub struct Day19 {
	file: String,
}

struct Input {
	workflows: HashMap<String, Vec<Rule>>,
	parts: Vec<Part>,
}

#[derive(Debug)]
struct Part {
	x: usize,
	m: usize,
	a: usize,
	s: usize,
}

#[derive(Debug)]
struct Rule {
	condition: Condition,
	then: String,
}

#[derive(Debug)]
enum Condition {
	Auto,
	Between(char, usize, usize),
}

impl Day19 {
	fn parse(&self) -> Result<Input, String> {
		let file = self.read_file_as_string()?;
		let mut file_iter = file.split("\n\n").into_iter();
		let workflows = file_iter.next().ok_or("No workflows")?.lines()
			.map(|line| self.parse_workflow(line))
			.collect::<Result<HashMap<String, Vec<Rule>>, String>>()?;

		let parts = file_iter.next().ok_or("No parts")?.lines()
			.map(|p| self.parse_part(p))
			.collect::<Result<Vec<Part>, String>>()?;

		Ok(Input { parts, workflows })
	}

	fn parse_part(&self, line: &str) -> Result<Part, String> {
		let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").map_err(|_| "Failed to create regex")?;
		if let Some(caps) = re.captures(line) {
			let x = caps.get(1).ok_or("No x")?.as_str().parse::<usize>().map_err(|_| "Failed to parse x")?;
			let m = caps.get(2).ok_or("No m")?.as_str().parse::<usize>().map_err(|_| "Failed to parse m")?;
			let a = caps.get(3).ok_or("No a")?.as_str().parse::<usize>().map_err(|_| "Failed to parse a")?;
			let s = caps.get(4).ok_or("No s")?.as_str().parse::<usize>().map_err(|_| "Failed to parse s")?;
			Ok(Part { x, m, a, s })
		} else {
			Err(format!("Failed to parse part {line}"))
		}
	}

	fn parse_workflow(
		&self,
		line: &str,
	) -> Result<(String, Vec<Rule>), String> {
		let re = Regex::new(r"(\w+)\{([^}]+)}").map_err(|_| "Failed to create regex")?;
		if let Some(caps) = re.captures(line) {
			let name = caps.get(1).ok_or("No name")?.as_str().to_string();

			let rules = caps.get(2).ok_or("No rules")?.as_str()
				.split(",")
				.map(|cond| self.parse_rule(cond))
				.collect::<Result<Vec<Rule>, String>>()?;

			Ok((name, rules))
		} else {
			Err("No match".to_string())
		}
	}

	fn parse_rule(
		&self,
		s: &str,
	) -> Result<Rule, String> {
		let split = s.split(":").collect::<Vec<&str>>();
		let mut iter = split.into_iter().rev();

		let then = iter.next().ok_or("No target for rule")?.to_string();
		let condition = match iter.next() {
			Some(s) => {
				let mut chars = s.chars();
				let target = chars.next().ok_or(format!("No target for rule {s}"))?;
				let num = s[2..].parse::<usize>().map_err(|_| format!("Failed to parse number in {s}"))?;

				match chars.next() {
					Some('>') => Condition::Between(target, num + 1, 4000),
					Some('<') => Condition::Between(target, 0, num - 1),
					_ => return Err(format!("Invalid target in {s}")),
				}
			}
			None => Condition::Auto,
		};

		Ok(Rule { condition, then })
	}
}

impl Solution for Day19 {
	fn new(file: &str) -> Self { Day19 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		let input = self.parse()?;
		let mut score = 0usize;

		for part in input.parts {
			let mut next = "in".to_string();
			while next != "A" && next != "R" {
				let workflow = input.workflows.get(&next).ok_or(format!("No workflow for {next}"))?;
				let evaluation = evaluate(&part, workflow)?.ok_or(format!("No evaluation for {part:?}"))?;
				next = evaluation;
			}

			if next == "A" { score += part.x + part.a + part.m + part.s; }
		}

		Ok(score.to_string())
	}
}

fn evaluate(part: &Part, rules: &Vec<Rule>) -> Result<Option<String>, String> {
	for rule in rules {
		let out = match rule.condition {
			Condition::Auto => Some(&rule.then),
			Condition::Between(c, lower, upper) => {
				let value = match c {
					'x' => part.x,
					'm' => part.m,
					'a' => part.a,
					's' => part.s,
					_ => return Err(format!("Invalid target {c}")),
				};
				if value >= lower && value <= upper { Some(&rule.then) } else { None }
			}
		};

		if let Some(x) = out {
			return Ok(Some(x.to_string()));
		}
	}

	Ok(None)
}
