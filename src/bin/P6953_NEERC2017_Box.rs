use std::io;

fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
fn main() {
    let line = read_numbers_from_input();
    let rect = vec![line[0], line[1], line[2]];
    let line = read_numbers_from_input();
    let paper = vec![line[0], line[1]];
    for i in 0..rect.len() {
        for j in 0..rect.len() {
            for k in 0..rect.len() {
                if i == j || j == k || k == i {
                    continue;
                }
                for t in 0..paper.len() {
                    let a = rect[i];
                    let b = rect[j];
                    let c = rect[k];
                    let h = paper[t ^ 1];
                    let w = paper[t];
                    if a + 2 * c <= w && 2 * b + 2 * c <= h {
                        println!("Yes");
                        return;
                    }
                    if a + b + c <= w && 2 * b + 2 * c <= h {
                        println!("Yes");
                        return;
                    }
                    if a + 2 * b <= w && 2 * b + 2 * c <= h {
                        println!("Yes");
                        return;
                    }
                    if a + b + c <= w && a + b + 2 * c <= h {
                        println!("Yes");
                        return;
                    }
                    if 2 * a + 2 * c <= w && b + 2 * c <= h {
                        println!("Yes");
                        return;
                    }
                    if 3 * a + b + c <= w && b + c <= h {
                        println!("Yes");
                        return;
                    }
                    if a + b + c <= w && 2 * a + 2 * c <= h {
                        println!("Yes");
                        return;
                    }
                    if 2 * a + b <= w && 2 * a + 2 * c <= h {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
