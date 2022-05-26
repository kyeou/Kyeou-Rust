use serde::{Deserialize, Serialize};
// mandatory lines to use json in rust
#[derive(Debug, Deserialize, Serialize)]

struct DemoObj {
    name: String,
    city: String,
    add: String,
    id: usize,
    }

fn main() {
    println!("Hello, world!");
}
