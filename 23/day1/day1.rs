use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./day1.txt").unwrap();
    let array = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let string_nums: HashMap<&str, u32> = array.iter().cloned().collect();
    let blocks: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    // for string in blocks.iter() {
    //     let mut first = 0;
    //     let mut second = 0;
    //     for char in string.chars() {
    //         let number = char.to_digit(10);
    //         if let Some(num) = number {
    //             if first == 0 {
    //                 first = num;
    //             } else {
    //                 second = num;
    //             }
    //         }
    //     }
    //     if second == 0 {
    //         second = first;
    //     }
    //     sum += 10 * first + second;
    // }
    for (idx, string) in blocks.iter().enumerate() {
        let mut first = 0;
        let mut second = 0;
        let mut num_string = String::new();
        for char in string.chars() {
            let number = char.to_digit(10);
            if let Some(num) = number {
                num_string.clear();
                if first == 0 {
                    first = num;
                } else {
                    second = num;
                }
            } else {
                let mut exists = false;
                num_string.push(char);
                for (val, _) in array.iter() {
                    if val.starts_with(&num_string) {
                        exists = true;
                    }
                }
                if exists {
                    if let Some(&value) = string_nums.get(num_string.as_str()) {
                        num_string.clear();
                        // num_string.push(char);
                        if first == 0 {
                            first = value;
                        } else {
                            second = value;
                        }
                    }
                } else {
                    num_string.clear();
                    num_string.push(char);
                }
            }
        }
        if second == 0 {
            second = first;
        }
        sum += 10 * first + second;
        println!("{}. {} {}", idx+1, first, second,);
    }
    println!("{}", sum);
}
