use std::fs::File;
use std::io::{BufRead, BufReader};




fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut sizes: Vec<&s> = Vec::new();
    let mut vals: Vec<String> = Vec::new();
    let mut puddles: i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
        if index == 0 {
            sizes = line.split(" ").collect::<Vec<&str>>();
        } else {
            vals.push(line);
        }
    }
    println!("{:#?}", sizes);
    println!("{:#?}", vals);





}
