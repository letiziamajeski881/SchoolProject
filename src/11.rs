use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("hello_world.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
}
