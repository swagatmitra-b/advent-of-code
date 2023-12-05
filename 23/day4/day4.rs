use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("./day4.txt").unwrap();
    let cards: Vec<Vec<&str>> = input
        .split("\n")
        .map(|card| card.split(":").collect::<Vec<&str>>())
        .collect();
    let mut total_points = 0;
    for (cardNo, card) in cards.iter().enumerate() {
        let mut points: Vec<u32> = Vec::new();
        let hands = card[1]
            .split("|")
            .map(|hand| hand.trim())
            .collect::<Vec<&str>>();
        let winning: Vec<u32> = hands[0]
            .split_whitespace()
            .map(|num| num.trim().parse::<u32>().ok().unwrap())
            .collect();
        let in_hand: Vec<u32> = hands[1]
            .split_whitespace()
            .map(|num| num.trim().parse::<u32>().ok().unwrap())
            .collect();
        for number in in_hand.iter() {
            if winning.contains(number) {
                points.push(*number);
            }
        }
        println!("{:?} {:?}", winning, in_hand);
        let n = points.len() as u32;
        if n != 0 {
            total_points += u32::pow(2, n - 1);
        }
    }
    println!("{}", total_points);
}
