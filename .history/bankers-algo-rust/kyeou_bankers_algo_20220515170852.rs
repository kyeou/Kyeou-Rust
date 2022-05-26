#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use std::io::{self, BufRead}; 
use std::io::Write;
macro_rules! read_input {
    ( $result:ident, $text: literal) => {
        print!("{}",$text);
        std::io::stdout().flush().unwrap();
        let mut dump = String::new();
        std::io::stdin().read_line(&mut dump).unwrap();
        let $result = dump.trim().parse::<i32>().unwrap_or_else(|_| {
            eprintln!("- Entered input is not Integer!");
            drop(dump);
            std::process::exit(1);
        });
    };
}

macro_rules! read_line_input {
    ($vec: expr,  $text: literal) => {
        $vec.clear();
        print!("{}",$text);
        std::io::stdout().flush().unwrap();
        //let mut line = String::new();
        let reader = io::stdin();
        let numbers: Vec<i32> = 
            reader.lock()                           // (0)
                  .lines().next().unwrap().unwrap() // (1)
                  .split(' ').map(|s| s.trim())     // (2)
                  .filter(|s| !s.is_empty())        // (3)
                  .map(|s| s.parse().unwrap())      // (4)
                  .collect();                       // (5)
                  for a in numbers {
                      $vec.push(a);
                  }
    };
}


macro_rules! if_then {
    ($x:expr, $y:stmt) => {
        if $x {
            $y
        }
    }
}

 


#[derive(Debug)]
struct _Process {
        seq: bool,
        not_safe: bool,
        claim: Vec<i32>,
        max_amt: Vec<i32>,
        potential: Vec<i32>,
}

#[derive(Debug)]
struct _Resource {
    ramt: i32,
    aval: i32,
}


fn run_bankers() {
  
let mut _res_amt: i32;
let mut _proc_amt: i32;
let mut _res_vec: Vec<_Resource> = Vec::new();
let mut _proc_vec: Vec<_Process> = Vec::new();
let mut _final_sequence : Vec<i32>;




    read_input!(_proc_amt, "Enter number of processes: ");
    println!("# of Processes: {}", _proc_amt);
    read_input!(_res_amt, "Enter number of resources: ");
    println!("# of Resources: {}", _res_amt);
       
    for _i in 0.._proc_amt {
        _proc_vec.push(_Process { seq: false, not_safe: false, claim: Vec::new(), max_amt: Vec::new(), potential: Vec::new() });
    }

    //println!("{:?}", _proc_vec);
    for _i in 0.._res_amt {
        _res_vec.push(_Resource { ramt: 0, aval: 0 });
    }
    let mut temp_vec: Vec<i32> = Vec::new();
    read_line_input!(temp_vec, "Enter number of units for each resource: ");
    let mut counter: i32 = 0;
    for i in  &mut _res_vec{
       i.ramt = temp_vec[counter as usize] as i32;
       counter+=1;
    } 
    //println!("{:?}", _res_vec);
    counter = 0;
    for i in &mut _proc_vec {
        print!("Enter maximum number of units process p{} will request from each resource (r0 to r{}): ",counter, (_res_amt-1));
        std::io::stdout().flush().unwrap();
        read_line_input!(temp_vec, "");
        for j in &mut temp_vec {
        i.max_amt.push(*j);
        }
        counter+=1;
    }

    counter = 0;
    for i in &mut _proc_vec {
        print!("Enter number of units of each resource (r0 to r{}) allocated to process p{}: ",(_res_amt-1), counter);
        std::io::stdout().flush().unwrap();
        read_line_input!(temp_vec, "");
        for j in &mut temp_vec {
        i.claim.push(*j);
        //println!("{:#?}", i.claim);
        }
        counter+=1;
    }

    println!("{}", _proc_amt[1]);
    println!("{:#?}", _proc_vec);







}//end function

fn prt() {

}


fn main() {
   
    let _choice: u8;
    loop {
        println!("\nBatch Scheduling");
        println!("--------------------------------");
        println!("1) Run bankers");
        println!("2) Quit program and free memory\n");
        read_input!(_choice, "Make a selection: ");
        match _choice {
            1=>{run_bankers()},
            2=>{println!("Quitting"); break;},
            _=>println!("Not a choice. Try again."),
        }
    }
  //let mut vec_of_nums : Vec<i32> =  Vec::new();
//read_line_input!(vec_of_nums , "Enter some shit:");
  // println!("{:?}", vec_of_nums);
}

