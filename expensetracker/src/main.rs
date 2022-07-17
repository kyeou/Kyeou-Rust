mod transaction;
use crate::transaction::TRANSACTION;
use std::io::Write;


fn main() {
    let mut file = std::fs::File::create("transactions.json").expect("create failed");
    let mut test = vec![TRANSACTION::new(),TRANSACTION::new()];
    test[0]._amount = 23.45;
    for item in &test {
        println!("{:?}", item);
    }


    let json_dump = serde_json::to_string_pretty(&test).unwrap();
    println!("{}", json_dump);
    file.write_all(json_dump.to_string().as_bytes()).expect("write failed");
}
