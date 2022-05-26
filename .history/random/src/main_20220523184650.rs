use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num8: u8   = rng.gen();
  println!("{}", rng.gen());
}