use std::collections::HashMap;
use std::fs;

fn main() {
    let mut hashmap = HashMap::new();
    let input = fs::read_to_string("./day6.txt").unwrap();
    let mut last_idx = 0;
    for (idx, character) in input.chars().enumerate() {
        if let Some(value) = hashmap.get(&character) {
            if *value >= last_idx {
                last_idx = *value + 1;
            } 
            hashmap.insert(character, idx);
        } else {
            hashmap.insert(character, idx);
            if idx - last_idx >= 4 {
                println!("{}", idx);
                return
            }
        }
    }
    println!("{}", input.len())
}
