//#![allow(dead_code)]
//#![allow(unused_macros)]
//#![allow(unused_imports)]

use rand::Rng;

fn main() {
    let  sieve = [rand::thread_rng; 10];
println!("{:?}", sieve);
}