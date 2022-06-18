use std::fs::File;
use std::io::{BufRead, BufReader};


struct ProposeDispose {
    _num_of_owners: i32,
    _owners_desire: Vec<Vec<i32>>,
    _dogs_desire: Vec<Vec<i32>>,
    _owners_tenative: Vec<Vec<i32>>,
    _dog_tentative: Vec<i32>
    _
}

fn read_input(  pd: &mut ProposeDispose, vals: &Vec<String>) {

     for i in 0..pd._num_of_owners {
        let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
         for j in 0..(pd._num_of_owners*2) {
             pd._owners_desire[i as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap();
         }
     }

  for i in pd._num_of_owners..(pd._num_of_owners + (pd._num_of_owners * 2)) {
         let vec = vals[i as usize].split(" ").collect::<Vec<&str>>();
         for j in 0..pd._num_of_owners {
             pd._dogs_desire[(i - pd._num_of_owners) as usize][j as usize] = vec[j as usize].parse::<i32>().unwrap();
         }
     }

}

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

let mut _pd = ProposeDispose {
    _num_of_owners: _n,
    _owners_desire: vec![vec![0;( _n*2) as usize]; (_n) as usize],
    _dogs_desire:  vec![vec![0; (_n) as usize]; (_n*2) as usize],
    _owners_tenative: vec![vec![-1; (2) as usize]; (_n) as usize],
    _dog_tentative: vec![-1; (_n*2) as usize]
};

    
     read_input(&mut _pd, &vals) ;


     println!("\nOwners List:\n\n {:?}", &_pd._owners_desire);
     println!("\nDogs List:\n\n {:?}", &_pd._dogs_desire);

   
     println!("\nOwners Engagements:\n\n {:?}", &_pd._owners_tenative);
     println!("\nDogs Engagements:\n\n {:?}", &_pd._dog_tentative);


}
