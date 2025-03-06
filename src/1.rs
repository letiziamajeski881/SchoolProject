use std::fs;

fn main() {
    let path = "/path/to/file.txt";
    let mut file = fs::File::create(path).unwrap();
    file.write_all(b"Hello, world!").unwrap();
}
