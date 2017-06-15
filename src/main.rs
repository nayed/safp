extern crate safp;

use std::env;
use std::io::prelude::*;
use std::process;

use safp::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(&mut stderr, "☹️ Problem parsing arguments: {}", err)
            .expect("Could not write to stderr");
        process::exit(1);
    });

    if let Err(e) = safp::run(config) {
        writeln!(&mut stderr, "🔥🔥🔥 Application error: {}", e)
            .expect("Could not write to stderr");
        process::exit(1);
    }
}
