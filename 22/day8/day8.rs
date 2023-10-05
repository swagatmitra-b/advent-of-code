use std::fs;

fn from_left(trees: &Vec<Vec<u32>>, row: usize, i: usize) -> Option<&str> {
    for (idx, height) in (&trees[row][0..i]).iter().enumerate() {
        if height >= &trees[row][i] {
            println!("{}", idx);
            return None;
        }
        println!("{}", idx);
    }
    Some("left")
}

fn from_right(trees: &Vec<Vec<u32>>, row: usize, i: usize) -> Option<&str> {
    let length: usize = trees[row].len();
    for height in &trees[row][i + 1..length] {
        if height >= &trees[row][i] {
            return None;
        }
    }
    Some("right")
}

fn from_top(trees: &Vec<Vec<u32>>, row: usize, i: usize) -> Option<&str> {
    for col in 0..row {
        if &trees[col][i] >= &trees[row][i] {
            return None;
        }
    }
    Some("top")
}

fn from_bottom(trees: &Vec<Vec<u32>>, row: usize, i: usize) -> Option<&str> {
    for col in (row + 1)..trees.len() {
        if &trees[col][i] >= &trees[row][i] {
            return None;
        }
    }
    Some("bottom")
}

fn main() {
    let mut visible_trees = 0;
    let input = fs::read_to_string("./day8.txt").expect("Failed to read file");
    let mut tree_arr: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        for character in line.chars() {
            if let Some(number) = character.to_digit(10) {
                row.push(number);
            }
        }
        tree_arr.push(row);
    }
    let rows = 1..tree_arr.len() - 1;
    for row in rows {
        for i in 1..tree_arr[row].len() - 1 {
            let left = from_left(&tree_arr, row, i);
            let right = from_right(&tree_arr, row, i);
            let top = from_top(&tree_arr, row, i);
            let bottom = from_bottom(&tree_arr, row, i);
            let unwrapped: Vec<&str> = [left, right, top, bottom]
                .iter()
                .filter_map(|x| x.clone())
                .collect();
            if !unwrapped.is_empty() {
                visible_trees += 1;
            }
            println!("{} {:?}", tree_arr[row][i], unwrapped);
        }
    }
    let rows = tree_arr.len();
    let cols = tree_arr[0].len();
    let edge_visible = (rows * cols) - ((rows - 2) * (cols - 2));
    println!("{}", visible_trees + edge_visible);
}
