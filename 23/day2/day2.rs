use std::fs;

const red: u32 = 12;
const green: u32 = 13;
const blue: u32 = 14;

fn main() {
    let input = fs::read_to_string("./day2.txt").unwrap();
    let a: Vec<Vec<Vec<&str>>> = input
        .split("\n")
        .map(|bag| {
            bag.split(":")
                .map(|balls| balls.split(";").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
        })
        .collect();
    let mut sum = 0;
    for (idx, game) in a.iter().enumerate() {
        let mut violate = false;
        'gameLoop: for bags in &game[1] {
            let colors: Vec<Vec<&str>> = bags
                .split(",")
                .map(|a| a.trim().split(" ").collect())
                .collect();
            for color in &colors {
                match color[1] {
                    "red" => {
                        if color[0].parse::<u32>().ok() > Some(red) {
                            violate = true;
                            break 'gameLoop;
                        }
                    }
                    "blue" => {
                        if color[0].parse::<u32>().ok() > Some(blue) {
                            violate = true;
                            break 'gameLoop;
                        }
                    }
                    "green" => {
                        if color[0].parse::<u32>().ok() > Some(green) {
                            violate = true;
                            break 'gameLoop;
                        }
                    }
                    (_) => {}
                }
            }
            println!("game id: {} {:?}", idx, colors);
        }
        if !violate {
            sum += idx + 1;
        }
    }
    println!("{}", sum);
}
