//#![allow(dead_code)]
//#![allow(unused_macros)]
//#![allow(unused_imports)]

using rand::Rng;

fn main() {
    let  sieve = [rand::thread_rng.gen(); 10];
    println!("{:?}", sieve);
}