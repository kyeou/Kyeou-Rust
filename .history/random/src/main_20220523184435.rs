using rand::Rng;

fn main() {
    let  sieve = [rand::thread_rng.gen(); 10];
    println!("{:?}", sieve);
}