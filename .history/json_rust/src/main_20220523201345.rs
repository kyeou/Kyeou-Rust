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
        let myobj = r#"
        {
        "name": "Amit vyas",
        "city": "Mumbai",
        "add": "mumbai street",
        "id" :001
        }
        "#;
        let demo: DemoObj = serde_json::from_str(myobj).unwrap();
        }