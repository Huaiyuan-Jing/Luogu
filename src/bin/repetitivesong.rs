use std::collections::HashMap;

fn main() {
    let n = read_i32() as usize;
    let mut arr: Vec<String> = vec![];
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        arr.push(s.trim().to_string());
    }
    let mut previous_pos = HashMap::new();
    let mut ans = i32::MAX;
    for i in 0..n {
        let s = &arr[i];
        if let Some(pos) = previous_pos.get(s) {
            ans = ans.min((i - *pos) as i32);
        }
        previous_pos.insert(s, i);
    }
    if ans == i32::MAX {
        println!("0");
    } else {
        println!("{}", n as i32 - ans);
    }
}
fn read_i32() -> i32 {
    use std::io::{self, Read, Write};
    let mut number = String::new();
    io::stdout().flush().unwrap();
    let mut stdin = io::stdin();
    let mut buffer = [0; 1];
    while let Ok(()) = stdin.read_exact(buffer.as_mut()) {
        let c = buffer[0] as char;
        if number.is_empty() && c == '-' {
            number.push(c);
            continue;
        }
        if c.is_numeric() {
            number.push(c);
        } else {
            break;
        }
    }
    number.parse::<i32>().unwrap_or(0)
}
