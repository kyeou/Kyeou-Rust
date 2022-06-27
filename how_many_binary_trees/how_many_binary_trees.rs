//extern crate num_bigint;
//use num_bigint::{BigInt, Sign};



fn find_amount_of_trees(_n: i32) {
    let mut amt_of_trees: Vec<u128> = Vec::new();
   amt_of_trees.push(1);
    println!("Base Case: {}", amt_of_trees[0]);


    for _i in 1..(_n+1) {
        let mut _current_count: u128 = 0;
        //println!("{}", _i);
        if _i % 2 == 0 {
            let mut _m : i32 = (_i as i32) - 1;
            let mut _k : i32 =  0;


            for _a in 0..(_i as i32)/2 {
                _current_count += amt_of_trees[_m as usize] *  amt_of_trees[_k as usize];
                _m -= 1;
                _k += 1;
            }
            _current_count *= 2;
            //println!("{}", _current_count);

        
        } else {
            let mut _m : i32 = (_i as i32) - 1;
            let mut _k : i32 =  0;


            for _a in 0..((_i as i32)-1)/2 {
                _current_count += amt_of_trees[_m as usize] *  amt_of_trees[_k as usize];
                _m -= 1;
                _k += 1;
            }
            _current_count *= 2;
            _current_count += amt_of_trees[_m as usize] * amt_of_trees[_m as usize];
            
        }
        amt_of_trees.push(_current_count);
    }

println!("{}", amt_of_trees[(_n) as usize]);

}

fn main() {
    
    for _q in 1..100 {
        println!("{}", _q);
        find_amount_of_trees(_q);
        println!();
    }
}