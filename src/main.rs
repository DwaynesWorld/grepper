extern crate grepper;

use grepper::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = grepper::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}
