use std::fs;

fn main() {
    let mut max_cal = 0;
    let mut sec_max = 0;
    let mut third_max = 0;
    let input = fs::read_to_string("./day1.txt").unwrap();
    let blocks: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|block| {
            block
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect();
    for calorie_blocks in blocks {
        let mut sum = 0;
        for calorie in calorie_blocks {
            sum += calorie;
        }
        if sum > max_cal {
            third_max = sec_max;
            sec_max = max_cal;
            max_cal = sum;
        } else if sum > sec_max && sum < max_cal {
            third_max = sec_max;
            sec_max = sum;
        } else if sum > third_max && sum < sec_max {
            third_max = sum;
        }
    }
    println!("{} {}", max_cal, max_cal+sec_max+third_max);
}
