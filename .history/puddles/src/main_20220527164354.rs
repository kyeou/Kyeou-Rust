use std::fs::File;
use std::io::{BufRead, BufReader};




fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut sizes: Vec<String> = Vec::new();
    let mut vals: Vec<String> = Vec::new();
    let mut puddles: i32 = 0;
   

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
       
            vals.push(line);
        
    }
    
    println!("{:#?}", vals);
    let mut map = vec![vec![0; _n as usize]; _n as usize];

        let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
        for j in 0..1 {
            sizes[i as usize]= vec[j as usize].parse::<i32>().unwrap();
        }
    



}
