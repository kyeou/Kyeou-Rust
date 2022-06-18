use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut _n: i32 = 0;
    let mut vals: Vec<String> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        
        if index == 0 {
            _n = line.unwrap().to_string().parse::<i32>().unwrap();
        } else {
            vals.push(line.unwrap().to_string());
        }
    }
    println!("{:#?}", vals);


    let mut owners_desire = vec![vec![0; _n as usize]; (_n*2) as usize];
    let mut dogs_desire = vec![vec![0; (_n*2) as usize]; _n as usize];
    let mut matchings: Vec<i32> = Vec::new();

    for i in 0.._n {
        let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
        for j in 0..(_n*2) {
            owners_desire[i as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap();
        }
    }
    println!("{:?}", mens_desire);
    for i in _n..(_Nn_n * 2) {
        let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
        for j in 0.._n {
            womens_desire[(i - _n) as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap();
        }
    }
}
