extern crate lsh_rust;

use lsh_rust::lsh_loop::*;

fn main() {
    let lsh_loop = LshLoop::new();

    match lsh_loop.start() {
        Ok(_) => println!("Exited successfully."),
        Err(e) => eprintln!("{}", e),
    }
}
