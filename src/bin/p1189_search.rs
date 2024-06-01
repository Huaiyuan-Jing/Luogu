use std::io;

fn main() {
    let numbers = read_numbers_from_input();
    let r = numbers[0] as usize;
    let c = numbers[1] as usize;
    let mut grid = vec![vec![0; c]; r];
    let mut ans = vec![vec![0; c]; r];
    let mut start_x = 0;
    let mut start_y = 0;
    for i in 0..r {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("读取输入失败");
        for j in 0..c {
            match line.chars().nth(j) {
                Some('.') => grid[i][j] = 0,
                Some('X') => grid[i][j] = -1,
                Some('*') => {start_x = i as i32; start_y = j as i32; grid[i][j] = 1;},
                _ => panic!("非法输入"),
            }
            ans[i][j] = grid[i][j];
        }
    }
    let numbers = read_numbers_from_input();
    let num_of_directions = numbers[0] as usize;
    let mut directions = vec![0; num_of_directions];
    for i in 0..num_of_directions {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("读取输入失败");
        match line.trim() {
            "NORTH" => directions[i] = 0,
            "EAST" => directions[i] = 1,
            "SOUTH" => directions[i] = 2,
            "WEST" => directions[i] = 3,
            _ => panic!("非法输入"),
        }
    }
    dfs(Node { x: start_x, y: start_y, step: 0 }, &grid, &directions, &mut ans);
    for i in 0..r {
        for j in 0..c {
            print!("{}", match ans[i][j] {
                0 => '.',
                1 => '*',
                _ => 'X',
            });
        }
        println!("");
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
struct Node {
    x: i32,
    y: i32,
    step: i32,
}
fn dfs(node: Node, grid: &Vec<Vec<i32>>, directions: &Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    let mask = vec![0, 1, 0, -1, 0];
    let mut x = node.x;
    let mut y = node.y;
    let step = node.step;
    if step == directions.len() as i32 {
        ans[x as usize][y as usize] = 1;
        return;
    }
    loop {
        x += mask[directions[step as usize] as usize];
        y += mask[directions[step as usize] as usize + 1];
        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 || grid[x as usize][y as usize] == -1 {
            break;
        }
        dfs(Node { x, y, step: step + 1 }, grid, directions, ans);
    }
}
