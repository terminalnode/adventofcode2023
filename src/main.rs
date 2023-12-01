use std::env;
use std::process::exit;
use getopts::Options;
use solution::{
	Day01, Day02, Solution,
};
use crate::solution::PlaceholderSolution;

mod solution;

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
		None => format!("day{:02}.txt", day),
		Some(file) => file,
	};

	let part_one = matches.opt_present("one");
	let part_two = matches.opt_present("two");
	let solution: Box<dyn Solution> = match day {
		1 => Box::new(Day01::new(&file)),
		2 => Box::new(Day02::new(&file)),
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
		Ok(result) => println!("{}", result),
		Err(error) => println!("Error: {}", error),
	}
}

fn print_usage(
	program: &str,
	opts: &Options,
) {
	let brief = format!("Usage: {} [options]", program);
	println!("{}", opts.usage(&brief))
}

fn print_error(
	error: &str,
	program: &str,
	opts: &Options,
) -> ! {
	println!("{}\n", error);
	print_usage(program, opts);
	exit(1)
}
