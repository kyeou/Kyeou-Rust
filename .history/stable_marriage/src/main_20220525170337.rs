use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_unstable(curr_man_id: i32, curr_woman_id: i32, matchings: Vec<i32>, mens_desire: Vec<Vec<i32>>, womens_desire: Vec<Vec<i32>>) -> bool {

    let mut pref_of_his_match: i32 = 0;
    let mut pref_of_her_match: i32 = 0;
    let mut pref_of_curr_man: i32 = 0;
    let mut pref_of_curr_woman: i32 = 0;
    let mut herr_curr_match: i32 = 0;

    if  curr_woman_id == matchings[(curr_man_id-1) as usize] {
        return false;
    }
    //return true;

    for i in 0..matchings.len() {
        if matchings[(currManID - 1) as usize] == mensDesire[(currManID - 1 as usize)][i as usize] {
            pref_of_his_match = i;
        }
    }

    for i in 0..matchings.len() {
        if matchings[i as usize] == curr_woman_id {
            herr_curr_match = matchings[i as usize];
        }
    }

    for i in 0..matchings.len() {
        // System.out.println("her Desire " + womensDesire[currWomanID - 1][i]);
        if  herr_curr_match == womens_desire[(curr_woman_id - 1) as usize][i as usize] {
            pref_of_her_match = i;
        }
    }

    for i in 0..matchings.len()  {
        if curr_man_id ==  womens_desire[(curr_woman_id - 1) as usize][i as usize] {
            pref_of_curr_man = i;
        }
    }

    for i in 0..matchings.len()  {
        if (curr_woman_id == mens_desire[(curr_man_id - 1) as usize][i as usize]) {
            pref_of_curr_woman = i;
        }
    }
    return ((pref_of_curr_woman < pref_of_his_match) && (pref_of_curr_man < pref_of_her_match));
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut _n: i32 = 0;
    let mut vals: Vec<String> = Vec::new();
    let mut not_stable: i32 = 0;

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

println!("{:?}", check_unstable(1,3, matchings, mens_desire, womens_desire));

for i in 1..(matchings.len()+1) {
    for j in 1..(matchings.len()+1) {
        if checkUnstable(i, j) {
            not_stable = not_stable + 1
        }
    }
}

}
