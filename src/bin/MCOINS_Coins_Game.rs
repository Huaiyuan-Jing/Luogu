fn main() {
    let k = read_i32();
    let l = read_i32();
    let m = read_i32();
    let mut max_n = -i32::MAX;
    let mut cases = vec![];
    for _ in 0..m {
        let n = read_i32();
        cases.push(n);
        max_n = max_n.max(cases[cases.len() - 1]);
    }
    let opt = can_A_win(k, l, max_n);
    // println!("{:?}", cases);
    for i in 0..m as usize {
        if opt[cases[i] as usize] {
            print!("A");
        } else {
            print!("B");
        }
    }
    println!();
}
fn can_A_win(k: i32, l: i32, n: i32) -> Vec<bool> {
    let mut opt = vec![false; n as usize + 1];
    for i in 1..=n as usize {
        if i as i32 - k >= 0 && !opt[i - k as usize] {
            opt[i as usize] = true;
            continue;
        }
        if i as i32 - l >= 0 && !opt[i - l as usize] {
            opt[i as usize] = true;
            continue;
        }
        if !opt[i - 1 as usize] {
            opt[i as usize] = true;
            continue;
        }
    }
    opt
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
