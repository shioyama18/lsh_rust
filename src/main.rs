extern crate rsh;
use rsh::rsh_loop::*;

fn main() {
    let rsh_loop = RshLoop::new();

    match rsh_loop.start() {
        Ok(_) => println!("Exited successfully."),
        Err(e) => eprintln!("{}", e),
    }
}
