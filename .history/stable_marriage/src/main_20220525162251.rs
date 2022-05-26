use std::fs::File;
use std::io::{BufRead, BufReader};



fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut _n: i32 = 0;
    let mut vals: Vec<String> = Vec::new();
  
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
let mut mens_desire = vec![vec![0; _n as usize]; _n as usize];
let mut womens_desire = vec![vec![0; _n as usize]; _n as usize];

let mut matchings: Vec<i32> = Vec::new();
for i in 0.._n {
    let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
    for j in 0.._n {
   mens_desire[i as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap(); 
}
}
println!("{:?}", mens_desire);
for i in _n..(_n*2) {
    let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
    for j in 0.._n {
   womens_desire[(i-_n) as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap(); 
}
}
println!("{:?}", womens_desire);

for i in _n*2.._n*3 {
    let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
    for j in 0.._n {
   womens_desire[(i-(_n*2)) as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap(); 
}
}



//println!("{}", _n);

//let mut test2d = vec![vec![2; _n as usize]; _n as usize];
//test2d[2][2] = 4;
//println!("{:?}", test2d);

//let teststring = "5 6 7";
//let vec = teststring.split(" ").collect::<Vec<&str>>();
//for i in 0.._n {
//    test2d[0][i as usize] = vec[i as usize].parse::<i32>().unwrap(); 
//}
//println!("{:?}", vec);
//println!("{:?}", test2d);
}
