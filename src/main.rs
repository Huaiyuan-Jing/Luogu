#![allow(non_snake_case)]
fn main() {
    println!("Hello, world!");
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
