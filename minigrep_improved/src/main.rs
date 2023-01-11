use std::env;

use std::process;

use minigrep;

fn main() {
    let args = env::args();

    let config = minigrep::Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprint!("Application error: {}", e);
        process::exit(1);
    }
}
