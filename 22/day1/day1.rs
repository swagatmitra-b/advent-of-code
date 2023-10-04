use std::fs;

struct Elf {
    index: usize,
    max_cal: u32,
}

fn main() {
    let mut result = Elf {
        index: 0,
        max_cal: 0,
    };
    let input = fs::read_to_string("./day1.txt").unwrap();
    let calorie_array: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    for (idx, calorie_bunch) in calorie_array.iter().enumerate() {
        let mut count = 0;
        let calories: Vec<u32> = calorie_bunch
            .split("\n")
            .filter(|x| !x.trim().is_empty()) 
            .map(|x| x.parse().unwrap())
            .collect();
        for calorie in calories {
            count += calorie;
        }
        if count > result.max_cal {
            result.max_cal = count;
            result.index = idx + 1
        }
    }
    println!("{} {}", result.max_cal, result.index);
}
