use std::fs; 

fn from_left(trees: &Vec<Vec<u32>>, row: usize, i: usize) -> Option<&str> {
    for height in &trees[row][0..i] {
        if height >= &trees[row][i] {
            return None
        }
    }
    Some("left")
}

fn from_right(trees: &Vec<Vec<u32>>, row: usize, i: usize) -> Option<&str> {
    for height in &trees[row][i+1..5] {
        if height >= &trees[row][i] {
            return None
        }
    }
    Some("right")
}

fn from_top(trees: &Vec<Vec<u32>>, row: usize, i: usize) -> Option<&str> {
   for col in 0..row {
       if &trees[col][i] >= &trees[row][i] {
         return None
       }
   }
   Some("top")
}

fn from_bottom(trees: &Vec<Vec<u32>>, row: usize, i: usize) -> Option<&str> {
    for col in (row + 1)..trees.len() { 
        if &trees[col][i] >= &trees[row][i] {
            return None
        }
    }
    Some("bottom")
}


fn main() {
    let input = fs::read_to_string("./day8.txt").unwrap();
    let tree_arr: Vec<Vec<u32>> = input
                     .lines()
                     .map(|x| x.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
                     .collect();
    let rows = [1, 2, 3];
    for row in rows {
        for i in 1..4 {
            let left = from_left(&tree_arr, row, i);
            let right = from_right(&tree_arr, row, i);
            let top = from_top(&tree_arr, row, i);
            let bottom = from_bottom(&tree_arr, row, i);
            let any_some = [left, right, top, bottom];
            let unwrapped: Vec<&str> = any_some.iter().filter_map(|x| x.clone()).collect();
            println!("{} {:?}", tree_arr[row][i], unwrapped);            
        }
    }                          
}

