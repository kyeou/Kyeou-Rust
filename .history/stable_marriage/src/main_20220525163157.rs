use std::fs::File;
use std::io::{BufRead, BufReader};

fn checkStable() ->bool

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
    for i in _n..(_n * 2) {
        let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
        for j in 0.._n {
            womens_desire[(i - _n) as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap();
        }
    }
    println!("{:?}", womens_desire);

    for i in _n * 2.._n * 3 {
        let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();

        matchings.push(vec[1 as usize].parse::<i32>().unwrap());
    }
    println!("{:?}", matchings);


}
