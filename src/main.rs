#[macro_use]

// clap for command line argument processing
extern crate clap;
extern crate rand;

use std::{env, thread, time};
use rand::{thread_rng, Rng};
use clap::{Arg,App};


fn philosopher(id: u8, cycles: u8, delay: u8) {
	println!("id: {}, name: {}, cycles: {}", id, thread::current().name().unwrap(), cycles);
	let mut rng = thread_rng();

	let mut sentinel = cycles + 1;
	while sentinel > 0 {
		// request forks
		let eat_time: u8 = rng.gen_range(0..delay);
		println!("{}: eating for {} seconds", id, eat_time);
		thread::sleep(time::Duration::from_secs(eat_time.into()));

		// mark forks as dirty
		let think_time: u8 = rng.gen_range(0..delay);
		println!("{}: thinking for {} seconds", id, think_time);
		thread::sleep(time::Duration::from_secs(think_time.into()));

		if cycles != 0 {	sentinel -= 1; }
	}
}


fn main() {
	fn valid_u8(x: String, min: Option<u8>) -> Result<(), String> {
		let val: u8;
		let m = min.unwrap_or(0);
		match x.trim().parse::<u8>() {
			Ok(n) => val = n,
			Err(_e) => return Err(format!("Expected a number between {} and 255, not \"{}\".", m, x))
		}
		if val >= m && val <= std::u8::MAX { return Ok(()); }
		return Err(format!("Expected a number between {} and 255, not \"{}\".", m, x));
	}

	// collect args
	let args = App::new("philosoph.rs")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Implementation of the Chandy/Misra solution to Dijkstra's dining philosphers problem.")
		.arg(Arg::with_name("PHILOSOPHERS")
			.help(&format!("The number of philosopher's at the table, between 2 and {}", std::u8::MAX))
			.default_value("5")
			.index(1)
			.validator(|x| valid_u8(x, Some(2)))
		)
		.arg(Arg::with_name("cycles")
			.help("The number of cycles of thinking and eating a philosopher will do before leaving (0..255), 0 is cycle infinitely")
			.default_value("0")
			.validator(|x| valid_u8(x, None))
		)
		.arg(Arg::with_name("max_delay")
			.help("The maximum think and eat time in seconds of each philosopher (0..255)")
			.default_value("10")
			.validator(|x| valid_u8(x, None))
		).get_matches();
	let total_philosophers = value_t!(args, "PHILOSOPHERS", u8).unwrap();
	let cycles = value_t!(args, "cycles", u8).unwrap();
	let delay = value_t!(args, "max_delay", u8).unwrap();

	// create philosophers
	let mut philosophers = Vec::new();
	for i in 0..total_philosophers {
		let builder = thread::Builder::new().name(format!("Philosopher #{}", i));
		let t = builder.spawn(move || philosopher(i, cycles, delay)).unwrap();
		philosophers.push(t);
	}
	for p in philosophers {
		p.join().expect("thread failed");
	}
}