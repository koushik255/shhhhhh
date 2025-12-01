use std::time::Instant;

use crate::helper::{everything, greet};
mod helper;

fn main() {
    let start = Instant::now();
    println!("Hello, world!");
    greet();

    let path = "/home/koushikk/Downloads/SHOWS".to_string();
    let lookfor = "mkv".to_string();
    everything(path, lookfor);
    let duration = start.elapsed();
    println!("Time spent {:?}", duration);
}
