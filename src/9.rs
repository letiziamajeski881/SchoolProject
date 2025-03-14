// This is an example of how you can use Rust to create a random number between 1 and 10
fn main() {
    let mut rng = rand::thread_rng();
    println!("Random number: {}", rng.gen_range(1, 11));
}
