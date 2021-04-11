#[macro_use]

// clap for command line argument processing
extern crate clap;

use std::thread;
use std::env;
use clap::{Arg,App};



fn main() {
	let args = App::new("philosoph.rs")
		.version(env!("CARGO_PKG_VERSION"))
		.about("Implementation of the Chandy/Misra solution to Dijkstra's dining philosphers problem.")
		.arg(Arg::with_name("PHILOSOPHERS")
			.help("The number of philosopher's at the table")
			.default_value("5")
			.index(1)
			.validator(|x| match x.parse::<u32>() {
				Ok(_n) => Ok(()),
				Err(_e) => Err(String::from(format!("Expected an unsigned integer, not \"{}\".", x)))
			})
		).get_matches();
	let num = value_t!(args, "PHILOSOPHERS", u32).unwrap();

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