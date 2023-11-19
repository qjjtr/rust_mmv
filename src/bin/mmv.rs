use std::process::exit;

use clap::Parser;
use copier::{run, Arguments};

fn main() {
    let arguments = Arguments::parse();

    let result_lines = match run(&arguments) {
        Ok(result_lines) => result_lines,
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    };

    for output_line in result_lines {
        println!("{}", output_line);
    }
}
