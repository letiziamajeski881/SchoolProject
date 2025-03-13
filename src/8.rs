
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "Alice");
    map.insert(2, "Bob");
    println!("{}", map[&2]);
}