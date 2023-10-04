use std::fs;

fn main() {
    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();
    let mut stack3 = Vec::new();
    let instructions = [
        [1, 2, 1],
        [3, 1, 3],
        [2, 2, 1], 
        [1, 1, 2]
    ];
    let input = fs::read_to_string("./day5.txt").unwrap();
    let array: Vec<Vec<&str>> = input
        .lines()
        .map(|x| x.split(",").map(|x| x.trim()).collect())
        .collect();
    for stack in 0..array.len() {
        match stack {
            0 => {
                for crates in &array[stack] {
                    stack1.push(crates);
                }
            }
            1 => {
                for crates in &array[stack] {
                    stack2.push(crates);
                }
            }
            2 => {
                for crates in &array[stack] {
                    stack3.push(crates);
                }
            }
            _ => println!("error"),
        }
    }
    for command in instructions {
        let [times, from, to] = command;
        match from {
            1 => {
                for _ in 0..times {
                    let popped = stack1.pop();
                    match to {
                        1 => stack1.push(popped.unwrap()),
                        2 => stack2.push(popped.unwrap()),
                        3 => stack3.push(popped.unwrap()),
                        _ => println!("err")
                    }
                }
            }
            2 => {
                for _ in 0..times {
                    let popped = stack2.pop();
                    match to {
                        1 => stack1.push(popped.unwrap()),
                        2 => stack2.push(popped.unwrap()),
                        3 => stack3.push(popped.unwrap()),
                        _ => println!("err")
                    }
                }

            }
            3 => {
                for _ in 0..times {
                    let popped = stack3.pop();
                    match to {
                        1 => stack1.push(popped.unwrap()),
                        2 => stack2.push(popped.unwrap()),
                        3 => stack3.push(popped.unwrap()),
                        _ => println!("err")
                    }
                }
            }
            _ => println!("err")
        }
    }
    println!("{}{}{}", stack1[stack1.len() -1], stack2[stack2.len() -1], stack3[stack3.len() -1]);
}
