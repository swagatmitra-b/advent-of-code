use std::fs;

fn get_result(a: &str, b: &str) -> (u32, u32) {
    match (a, b) {
        ("A", "X") => (3, 1),
        ("A", "Y") => (6, 2),
        ("A", "Z") => (0, 3),
        ("B", "X") => (0, 1),
        ("B", "Y") => (3, 2),
        ("B", "Z") => (6, 3),
        ("C", "X") => (6, 1),
        ("C", "Y") => (0, 2),
        ("C", "Z") => (3, 3),
        _ => (0, 0), 
    }
}

fn get_result_2(a: &str, b: &str) -> (u32, u32) {
    match (a, b) {
        ("A", "X") => (0, 3),  
        ("A", "Y") => (3, 1),
        ("A", "Z") => (6, 2),
        ("B", "X") => (0, 1),
        ("B", "Y") => (3, 2),
        ("B", "Z") => (6, 3),
        ("C", "X") => (0, 2),
        ("C", "Y") => (3, 3),
        ("C", "Z") => (6, 1),
        _ => (0, 0), 
    }
}

fn main() {
    let input = fs::read_to_string("./day2.txt").expect("Could not read file");
    let rounds: Vec<&str> = input.lines().collect();
    let mut score1 = 0;
    let mut score2 = 0;

    for round in rounds.iter() {
        let moves: Vec<&str> = round.split_whitespace().collect();
        let (outcome1, move_price1) = get_result(moves[0], moves[1]);
        let (outcome2, move_price2) = get_result_2(moves[0], moves[1]);
        score1 += outcome1 + move_price1;
        score2 += outcome2 + move_price2;
    }

    println!("{} {}", score1, score2);
}
