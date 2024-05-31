use std::io;

fn main() {
    let in_line = read_numbers_from_input();
    let n = in_line[0];
    let k = in_line[1];
    let mut val = vec![0; k as usize];
    val[0] = 1;

}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
fn dfs(val: &mut Vec<i32>, pos: usize, n: i32, k: i32) {
    if pos == k as usize {
        
    }
}
