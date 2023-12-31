use std::fs;

fn main() {
    let mut number = 0;
    let mut overlap = 0;
    let input = fs::read_to_string("./day4.txt").unwrap();
    let array: Vec<Vec<Vec<&str>>> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| range.split("-").collect())
                .collect()
        })
        .collect();
    for i in 0..array.len() {
        let a1 = array[i][0][0].parse::<u32>();
        let a2 = array[i][0][1].parse::<u32>();
        let b1 = array[i][1][0].parse::<u32>();
        let b2 = array[i][1][1].parse::<u32>();

        if let (Ok(a1), Ok(a2), Ok(b1), Ok(b2)) = (&a1, &a2, &b1, &b2) {
            if a1 >= b1 && a2 <= b2 {
                number += 1;
            } else if b1 >= a1 && b2 <= a2 {
                number += 1; 
            }
        }

        if let (Ok(a1), Ok(a2), Ok(b1), Ok(b2)) = (a1, a2, b1, b2) {
            if (a1 < b1 && a1 < b2) && (a2 < b1 && a2 < b2) {
                overlap += 1;
            } else if (b1 < a1 && b1 < a2) && (b2 < a1 && b2 < a2) {
                overlap += 1;
            }
        }
    }
    println!("{} {}", number.len(), array.len() - overlap.len());
}
