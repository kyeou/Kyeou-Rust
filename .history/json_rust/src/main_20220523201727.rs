exter
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)] 

struct DemoObjectStruct {
rolno:usize,
name: String,
city: String,
salary : usize,
}
fn main() {
println!("demo to show json in rust !!");
println!("creating json object below ::");
let demoobj = r#"
{
"rolno": 001,
"name": "dummy name here",
"city": "mumbai",
"salary" : 1000
}
"#;
let jobject: DemoObjectStruct = serde_json::from_str(demoobj).unwrap();
println!("prinitng value og jsn objetc :: ");
println!("value inside the json objetc is :: {:?}", jobject);
}