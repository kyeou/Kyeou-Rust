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

 fn main() {
     
 }