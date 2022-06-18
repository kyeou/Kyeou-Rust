use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input.text").unwrap());
    let m_n: i32 = 0;
    let mut vals: Vec<String> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        
        if index == 0 {
            _n = line.unwrap().to_string().parse::<i32>().unwrap();
        } else {
            vals.push(line.unwrap().to_string());
        }
    }
    println!("{:#?}", vals);
}
