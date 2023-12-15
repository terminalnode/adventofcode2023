use crate::solution::Solution;

pub struct Day15 {
	file: String,
}

struct Step {
	raw: String,
	label: String,
	operation: Operation,
}

#[derive(Debug, PartialEq)]
enum Operation {
	Remove,
	Set(usize),
}

impl Day15 {
	fn parse(&self) -> Result<Vec<Step>, String> {
		Ok(self.read_file_as_string()?
			.split(',')
			.map(|cs| parse_step(cs))
			.collect::<Result<Vec<Step>, String>>()?)
	}
}

fn parse_step(step: &str) -> Result<Step, String> {
	let mut i = 0usize;
	let chars = step.chars().collect::<Vec<char>>();
	let mut label = String::new();
	let raw = step.trim().to_string();

	while chars[i] != '=' && chars[i] != '-' {
		if !chars[i].is_whitespace() { label.push(chars[i]); }
		i += 1;
	}

	let operation = match chars[i] {
		'=' => Operation::Set(chars[i+1].to_string().parse().unwrap()),
		'-' => Operation::Remove,
		c => return Err(format!("Invalid operation: {c}")),
	};

	Ok(Step { raw, label, operation })
}

fn hash(s: &String) -> usize {
	let mut out = 0;
	for char in s.chars() {
		out += char as usize;
		out *= 17;
		out %= 256;
	}

	out
}

impl Solution for Day15 {
	fn new(file: &str) -> Self { Day15 { file: file.to_string() } }
	fn get_file_name(&self) -> &str { &self.file }

	fn part_one(&self) -> Result<String, String> {
		Ok(self.parse()?
			.iter()
			.map(|s| hash(&s.raw))
			.sum::<usize>()
			.to_string())
	}

	fn part_two(&self) -> Result<String, String> {
		let steps = self.parse()?;

		// &String is the label, usize is the lens
		let mut boxes: Vec<Vec<(String, usize)>> = (0..256).map(|_| Vec::new()).collect();

		for step in steps {
			let step_box = boxes.get_mut(hash(&step.label))
				.ok_or(format!("Invalid hash for {} ({})", &step.label, hash(&step.label)))?;

			match step.operation {
				Operation::Remove => step_box.retain(|(label, _)| label != &step.label),
				Operation::Set(lens) => {
					let mut replaced = false;
					for i in 0..step_box.len() {
						if step_box[i].0 == step.label {
							step_box[i].1 = lens;
							replaced = true;
							break;
						}
					}

					if !replaced { step_box.push((step.label, lens)); }
				},
			};
		}

		let mut out = 0usize;
		for (ibox, cbox) in boxes.iter().enumerate() {
			for (islot, (_, lens)) in cbox.iter().enumerate() {
				out += (ibox+1) * (islot+1) * (lens);
			}
		}

		Ok(out.to_string())
	}
}
