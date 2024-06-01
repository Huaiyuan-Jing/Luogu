use std::io;

fn main() {
    let numbers = read_numbers_from_input();
    let r = numbers[0] as usize;
    let c = numbers[1] as usize;
    let mut grid = vec![vec![0; c]; r];
    for i in 0..r {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("读取输入失败");
        for j in 0..c {
            match line.chars().nth(j) {
                Some('.') => grid[i][j] = 0,
                Some('X') => grid[i][j] = -1,
                Some('*') => grid[i][j] = 1,
                _ => panic!("非法输入"),
            }
        }
    }
    let numbers = read_numbers_from_input();
    let num_of_directions = numbers[0] as usize;
    let mut directions = vec![0; num_of_directions];
    for i in 0..num_of_directions {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("读取输入失败");
        match line.as_str() {
            "North" => directions[i] = 0,
            "East" => directions[i] = 1,
            "South" => directions[i] = 2,
            "West" => directions[i] = 3,
            _ => panic!("非法输入"),
        }
    }
    
}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
fn dfs(x: i32, y: i32, step: i32, grid: &Vec<Vec<i32>>, directions: &Vec<i32>) {
    if step == directions.len() as i32 - 1 {

    }
}