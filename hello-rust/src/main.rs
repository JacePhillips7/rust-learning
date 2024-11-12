use std::io::{stdout, BufWriter};

use ferris_says::say;
fn main() {
    println!("Hello, world!");
    let message = "Hello world!";
    let writer = BufWriter::new(stdout().lock());
    let _ = say(&message, message.len(), writer);
}
