use std::env;
use std::process::exit;
use getopts::Options;
use solution::*;

mod solution;
mod util;

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.reqopt("d", "day", "specify which day to run", "[1..25]");
	opts.optopt("f", "file", "override which file to use", "FILE");
	opts.optflag("", "one", "toggle running part 1");
	opts.optflag("", "two", "toggle running part 2");
	opts.optflag("h", "help", "print this help menu");

	let matches = opts.parse(&args[1..])
		.unwrap_or_else(|_| {
			print_usage(&program, &opts);
			exit(1);
		});

	if matches.opt_present("h") {
		print_usage(&program, &opts);
		exit(0);
	}

	let day = match matches.opt_str("d") {
		None => print_error("Day must be specified!", &program, &opts),
		Some(x) => x.parse::<i32>()
			.unwrap_or_else(|_| print_error("Day must be a number!", &program, &opts)),
	};

	let file = match matches.opt_str("f") {
		None => format!("day{day:02}.txt"),
		Some(file) => file,
	};

	let part_one = matches.opt_present("one");
	let part_two = matches.opt_present("two");
	let solution: Box<dyn Solution> = match day {
		1 => Box::new(Day01::new(&file)),
		2 => Box::new(Day02::new(&file)),
		3 => Box::new(Day03::new(&file)),
		4 => Box::new(Day04::new(&file)),
		5 => Box::new(Day05::new(&file)),
		6 => Box::new(Day06::new(&file)),
		7 => Box::new(Day07::new(&file)),
		8 => Box::new(Day08::new(&file)),
		9 => Box::new(Day09::new(&file)),
		10 => Box::new(Day10::new(&file)),
		11 => Box::new(Day11::new(&file)),
		12 => Box::new(Day12::new(&file)),
		13 => Box::new(Day13::new(&file)),
		_ => Box::new(PlaceholderSolution::new(&file)),
	};

	if !part_one && !part_two {
		print_error("Must specify at least one part to run!", &program, &opts);
	} else if part_one && part_two {
		run(1, || solution.part_one());
		println!();
		run(2, || solution.part_two());
	} else if part_one {
		run(1, || solution.part_one());
	} else if part_two {
		run(2, || solution.part_two());
	}
}

fn run<F>(
	part: i32,
	function: F,
) where F: Fn() -> Result<String, String> {
	println!("Running part {}...", part);
	match function() {
		Ok(result) => println!("{result}"),
		Err(error) => println!("Error: {error}"),
	}
}

fn print_usage(
	program: &str,
	opts: &Options,
) {
	let brief = format!("Usage: {program} [options]");
	println!("{}", opts.usage(&brief))
}

fn print_error(
	error: &str,
	program: &str,
	opts: &Options,
) -> ! {
	println!("{error}\n");
	print_usage(program, opts);
	exit(1)
}
