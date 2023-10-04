use std::collections::HashSet;
use std::fs;

fn get_val(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("./day3.txt").expect("Could not read file");
    let rucksacks: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for rucksack in rucksacks {
        let mut set = HashSet::new();
        let (first, last) = rucksack.split_at(rucksack.len() / 2);
        let mut common: Option<char> = None;
        for character1 in first.chars() {
            set.insert(character1);
        }
        for character2 in last.chars() {
            if set.contains(&character2) {
                common = Some(character2);
                break;
            }
        }

        match common {
            Some(common_char) => {
                let value = get_val(common_char);
                sum += value;
            }
            None => println!("no common chars!"),
        }
    }
    println!("{}", sum)
}
