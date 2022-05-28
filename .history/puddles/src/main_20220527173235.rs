use std::fs::File;
use std::io::{BufRead, BufReader};


fn check_unstable() -> bool {

}
fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut sizes: Vec<i32> = Vec::new();
    let mut vals: Vec<String> = Vec::new();
    let mut _puddles: i32 = 0;
   

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
       
            vals.push(line);
        
    }
    
    //println!("{:#?}", vals);
    

        let sz = vals[0 as usize].split(" ").collect::<Vec<&str>>();
        for j in 0..2 {
            sizes.push(sz[j as usize].parse::<i32>().unwrap());
        }
    
       // println!("{:#?}", sizes);
        let mut map = vec![vec![0; sizes[1] as usize]; sizes[0] as usize];
        let mut visited = vec![vec![false; sizes[1] as usize]; sizes[0] as usize];

        for i in 1..(sizes[0]+1) {
            let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
            for j in 0..sizes[1] {
                map[(i-1) as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap();
            }
        }
        //println!("{:?}", map);


        for i in 0..sizes[0] {
            for j in 0..sizes[1] {
                if map[i][j] == 0 && !(visited[i][j]) {
                    puddles(i, j, &m);
                    puddles += 1;
                }
            }
        }
}
