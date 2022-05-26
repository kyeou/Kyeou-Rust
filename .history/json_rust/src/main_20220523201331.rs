use serde::{Deserialize, Serialize};
// mandatory lines to use json in rust
#[derive(Debug, Deserialize, Serialize)]

fn main() {
    println!("Hello, world!");
}
