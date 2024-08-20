#![allow(non_snake_case)]

use std::io;

fn main() {
    let inline = read_numbers_from_input();
    let mut n = inline[0];
    let base_n = n;
    let base = inline[1];
    let mut ans = String::new();
    while n != 0 {
        let mut next = n / base;
        while n - next * base < 0 {
            next += 1;
        }
        let digit = n - next * base;
        let digit = if digit < 10 {
            (digit as u8 + b'0') as char
        } else {
            ((digit - 10) as u8 + b'A') as char
        };
        ans.push(digit);
        n = next;
    }
    let mut ans_chars: Vec<char> = ans.chars().collect();
    ans_chars.reverse();
    let ans_reversed: String = ans_chars.into_iter().collect();
    println!("{}={}(base{})", base_n, ans_reversed, base);
}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
