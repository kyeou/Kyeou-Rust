use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let  sieve = [rng.gen(); 10];
    println!("{:?}", sieve);
}