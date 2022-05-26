use std::fs::File;
use std::io::{BufRead, BufReader};



fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut _n: i32 = 0;
    let mut vals: Vec<String> = Vec::new();
    let mut
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string(); 
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
        if index == 0 {
            _n = line.parse::<i32>().unwrap();
        } else {   
           vals.push(line); 
        }
        

       
    }
println!("{:#?}", vals);

}
