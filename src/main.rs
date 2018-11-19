extern crate rsh;

use rsh::status::Status;
use rsh::error::RshError;
use rsh::command::*;
use std::io::prelude::*;

fn main() {
    println!("Welcome to rsh, minimal shell implemented with Rust.");

    match run() {
        Ok(_)  => println!("Bye."),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<Status, RshError> {
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        let args = input.split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        match rsh_execute(args) {
            Ok(status) => match status {
                Status::Success => continue,
                Status::NoCommand => continue,
                Status::Exit => return Ok(Status::Exit),
            }
            Err(e) => return Err(e),
        }
    }

}
