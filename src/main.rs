#[macro_use]

// clap for command line argument processing
extern crate clap;

use std::thread;
use std::env;
use clap::{Arg,App};




fn main() {
	fn validate_pholosophers(x: String) -> Result<(), String> {
		let val: u8;
		match x.trim().parse::<u8>() {
			Ok(n) => val = n,
			Err(_e) => return Err(format!("Expected a number between 2 and 255, not \"{}\".", x))
		}
		if val >= 2 && val <= std::u8::MAX { return Ok(()); }
		return Err(format!("Expected a number between 2 and 255, not \"{}\".", x));
	}
	// collect args
	let args = App::new("philosoph.rs")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Implementation of the Chandy/Misra solution to Dijkstra's dining philosphers problem.")
		.arg(Arg::with_name("PHILOSOPHERS")
			.help(&format!("The number of philosopher's at the table, between 2 and {}", std::u8::MAX))
			.default_value("5")
			.index(1)
			.validator(validate_pholosophers)
		).get_matches();
	let num = value_t!(args, "PHILOSOPHERS", u8).unwrap();
	// create philosophers
	let mut philosophers = Vec::new();
	for i in 0..num {
		let t = thread::spawn(move || {
			// TODO: think and eat
			println!("Philosopher #{}", i);
		});
		philosophers.push(t);
	}
	for p in philosophers{
		p.join().expect("thread failed");
	}
}