extern crate safp;

use std::env;
use std::io::prelude::*;
use std::process;

use safp::Config;

fn main() {
    let mut stderr = std::io::stderr();

    let config = Config::new(env::args()).unwrap();

    if let Err(e) = safp::run(config) {
        writeln!(&mut stderr, "🔥🔥🔥 Application error: {}", e)
            .expect("Could not write to stderr");
        process::exit(1);
    }
}
