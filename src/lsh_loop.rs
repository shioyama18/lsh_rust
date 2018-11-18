use status::Status;
use error::LshError;
use command::*;
use std::io::{self, Write};


pub struct LshLoop;

impl LshLoop {
    pub fn new() -> Self {
        LshLoop
    }

    pub fn start(&self) -> Result<Status, LshError> {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            let scan = io::stdin();
            let _ = scan.read_line(&mut input);
            
            let args = input.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            match execute(args) {
                Ok(status) => match status {
                    Status::Success => continue,
                    Status::NoCommand => continue,
                    Status::Exit => return Ok(Status::Exit),
                }
                Err(e) => return Err(e),
            }
        }
    }
}
