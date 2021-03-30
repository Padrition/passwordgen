mod password;
mod time_rng;

use password::Config;
use std::env;
use std::process;

fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {}", err);
        process::exit(1);
    });

    println!("{}", password::rand_pass(conf));
}
