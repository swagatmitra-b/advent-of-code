use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day6.txt").unwrap();
    let buffer_arr: Vec<&str> = input.lines().collect();
    for stream in buffer_arr {
        let mut last_idx = 0;
        let mut hashmap = HashMap::new();
        for (idx, character) in stream.chars().enumerate() {
            if let Some(value) = hashmap.get(&character) {
                if *value >= last_idx {
                    last_idx = *value + 1;
                }
                hashmap.insert(character, idx);
            } else {
                hashmap.insert(character, idx);
                if idx - last_idx >= 4 {
                    println!("{}", idx);
                    break
                }
            }
        }
    }
}
