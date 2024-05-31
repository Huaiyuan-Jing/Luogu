use std::{f64::consts::PI, io};

fn main() {
    let line = read_numbers_from_input();
    let n = line[0];
    let m = line[1];
    let mut f = vec![0.0; n as usize + 1];
    let mut ans = 0.0;
    for i in 1..=n as usize {
        let mut s = 0.0;
        for k in 1..=m - 1 {
            s += (i as f64 * 2.0).min(k as f64 * i as f64 * PI / m as f64);
        }
        s += 2.0 * i as f64;
        f[i] = f[i - 1] + 1.0 + s;
        ans += (f[i - 1] + 1.0) * m as f64 * 2.0 + s * m as f64;
        println!("{:.6}", ans);
    }
    println!("{:.6}", ans);
}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
