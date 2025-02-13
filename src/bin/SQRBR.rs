fn main() {
    for _ in 0..read_i32() {
        test_case();
    }
}
fn test_case() {
    let n = read_i32();
    let k = read_i32();
    let mut mark = vec![false; n as usize * 2 + 1];
    for _ in 0..k {
        mark[read_i32() as usize] = true;
    }
    let mut dp = vec![vec![0; n as usize * 2 + 1]; n as usize * 2 + 1];
    dp[0][0] = 1;
    for i in 1..=n as usize * 2 {
        for j in 0..=i {
            if i - j > j {
                continue;
            }
            if mark[i] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
            }
        }
    }
    println!("{}", dp[n as usize * 2][n as usize]);
}
fn read_i32() -> i32 {
    use std::io::{self, Read, Write};
    let mut number = String::new();
    io::stdout().flush().unwrap();
    let mut stdin = io::stdin();
    let mut buffer = [0; 1];
    let mut started = false;
    while let Ok(()) = stdin.read_exact(buffer.as_mut()) {
        let c = buffer[0] as char;
        if !started {
            if c == '-' || c.is_numeric() {
                started = true;
                number.push(c);
            }
        } else {
            if c.is_numeric() {
                number.push(c);
            } else {
                break;
            }
        }
    }
    number.parse::<i32>().unwrap_or(0)
}
