use std::fs;

fn main() {
    let input = fs::read_to_string("./day1.txt").unwrap();
    let blocks: Vec<&str> = input
        .split("\n")
        .collect();
    let mut sum = 0;
    for string in blocks.iter() {
        println!("{:?}", string);
        let mut first = 0;
        let mut second = 0;
        for char in string.chars() {
            let number = char.to_digit(10);
            if let Some(num) = number {
                if first == 0 {
                    println!("{}", num);
                    first = num;
                } else {
                    println!("{}", num);
                    second = num;
                }
            }
        }
        if second == 0 {
            second = first;
        }
        sum += 10 * first + second;
    }
    println!("{}", sum)
}
