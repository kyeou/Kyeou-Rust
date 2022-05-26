use std::fs::File;
use std::io::{BufRead, BufReader};



fn main() {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

   
    let mut _n: i32 = 0;
    let mut vals: Vec<&str> = Vec::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        //let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
        if index == 0 {
            _n = line.unwrap().parse::<i32>().unwrap();
            vals = vec![""; _n* as usize];
        } else {
            
           //vals[(index-1) as usize] = &line.unwrap();
           println!("{:?}", vals);
        }
        

       
    }

println!("{}", _n);

let mut test2d = vec![vec![2; _n as usize]; _n as usize];
test2d[2][2] = 4;
println!("{:?}", test2d);

let teststring = "5 6 7";
let vec = teststring.split(" ").collect::<Vec<&str>>();
for i in 0.._n {
    test2d[0][i as usize] = vec[i as usize].parse::<i32>().unwrap(); 
}
println!("{:?}", vec);
println!("{:?}", test2d);
}
