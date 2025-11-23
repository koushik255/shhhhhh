use crate::helper::{everything, greet};
mod helper;

fn main() {
    println!("Hello, world!");
    greet();

    let path = "/home/".to_string();
    everything(path);
}
