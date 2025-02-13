#![allow(non_snake_case)]
fn main() {
    println!("{}", read_i32());
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
